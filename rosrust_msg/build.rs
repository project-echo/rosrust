use std::ffi::OsStr;
use std::path::Path;
use std::{env, fs};

macro_rules! build_println {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    rerun_if_env_changed("ROS_PACKAGE_PATH");
    rerun_if_env_changed("ROSRUST_MSG_PATH");

    let ros_package_paths = env::var("ROS_PACKAGE_PATH")
        .unwrap_or_default()
        .split(':')
        .map(String::from)
        .collect::<Vec<String>>();
    build_println!("ros_package_paths: {:?}", ros_package_paths);

    let extra_paths = env::var("ROSRUST_MSG_PATH")
        .unwrap_or_default()
        .split(':')
        .map(String::from)
        .collect::<Vec<String>>();
    build_println!("extra_paths: {:?}", extra_paths);

    let paths = ros_package_paths
        .iter()
        .chain(extra_paths.iter())
        .collect::<Vec<_>>();
    for path in &paths {
        rerun_if_folder_content_changed(Path::new(path));
    }

    let messages = paths
        .iter()
        .flat_map(|path| find_all_messages_and_services(Path::new(path)))
        .collect::<Vec<(String, String)>>();

    let file_name = format!("{}/{}", out_dir, "messages.rs");

    let package_names = messages
        .iter()
        .map(|(pkg, msg)| format!("{}/{}", pkg, msg))
        .collect::<Vec<String>>()
        .join(",");
    let package_tuples = messages
        .iter()
        .map(|(pkg, msg)| format!("(\"{}\",\"{}\")", pkg, msg))
        .collect::<Vec<String>>()
        .join(",");

    // Panic on an empty message list: there is no use for this, and it avoids
    // a cryptic error in the proc macro invocation later on
    if package_names.is_empty() {
        panic!("empty package_names: are any of ROS_PACKAGE_PATH and ROSRUST_MSG_PATH defined? is your catkin_ws or /opt/ros/<VERSION>/setup.bash sourced?");
    }
    build_println!("package_names: {:?}", package_names);

    let file_content = format!(
        r#"
rosrust::rosmsg_include!({},IGNORE_BAD);
pub static MESSAGES: &[(&str, &str)]=&[{}];
        "#,
        package_names, package_tuples
    );
    build_println!("package_names: {:?}", package_tuples);

    fs::write(file_name, file_content).unwrap();
}

fn rerun_if_file_changed(key: &str) {
    println!("cargo:rerun-if-changed={}", key);
}

fn rerun_if_env_changed(key: &str) {
    println!("cargo:rerun-if-env-changed={}", key);
}

pub fn rerun_if_folder_content_changed(folder: &Path) {
    if !folder.is_dir() {
        if folder.extension() == Some(OsStr::new("msg"))
            || folder.extension() == Some(OsStr::new("srv"))
        {
            if let Some(name) = folder.to_str() {
                rerun_if_file_changed(name);
            }
        }
        return;
    }

    if let Ok(children) = fs::read_dir(folder) {
        for child in children.filter_map(Result::ok) {
            rerun_if_folder_content_changed(&child.path());
        }
    }
}

fn find_all_messages_and_services(root: &Path) -> Vec<(String, String)> {
    if !root.is_dir() {
        return identify_message_or_service(root).into_iter().collect();
    }
    let mut items = vec![];
    if let Ok(children) = fs::read_dir(root) {
        for child in children.filter_map(|v| v.ok()) {
            items.append(&mut find_all_messages_and_services(&child.path()));
        }
    }
    items
}

fn identify_message_or_service(filename: &Path) -> Option<(String, String)> {
    let extension = filename.extension()?;
    let message = filename.file_stem()?;
    let parent = filename.parent()?;
    let grandparent = parent.parent()?;
    let package = grandparent.file_name()?;
    if Some(extension) != parent.file_name() {
        return None;
    }
    match extension.to_str() {
        Some("msg") => {}
        Some("srv") => {}
        _ => return None,
    }
    Some((package.to_str()?.into(), message.to_str()?.into()))
}
