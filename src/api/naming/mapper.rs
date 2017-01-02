use std::collections::HashMap;
use super::path::{Buffer, Slice, Path};

pub struct Mapper {
    children: HashMap<String, Mapper>,
    value: Option<Buffer>,
}

impl Mapper {
    pub fn new() -> Mapper {
        Mapper {
            children: HashMap::new(),
            value: None,
        }
    }

    pub fn add(&mut self, keys: &[String], value: Buffer) {
        match keys.split_first() {
            None => {
                self.value = Some(value);
            }
            Some((key, child_keys)) => {
                self.children
                    .entry(key.clone())
                    .or_insert(Mapper::new())
                    .add(child_keys, value);
            }
        }
    }

    pub fn translate(&self, keys: &[String]) -> Option<Slice> {
        match keys.split_first() {
            None => self.value.as_ref().map(|v| v.slice()),
            Some((key, child_keys)) => self.children.get(key).and_then(|v| v.translate(child_keys)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::path::{Buffer, Path};

    #[test]
    fn matches_existing_paths() {
        let mut mapper = Mapper::new();
        let src1 = "/foo/bar".parse::<Buffer>().unwrap();
        let dst = "/a/b/c".parse::<Buffer>().unwrap();
        mapper.add(src1.get(), dst);
        let src2 = "/foo/ter".parse::<Buffer>().unwrap();
        let dst = "/d/e/f".parse::<Buffer>().unwrap();
        mapper.add(src2.get(), dst);
        assert_eq!("/a/b/c",
                   format!("{}", mapper.translate(src1.get()).unwrap()));
        assert_eq!("/d/e/f",
                   format!("{}", mapper.translate(src2.get()).unwrap()));
    }

    #[test]
    fn allows_root_path() {
        let mut mapper = Mapper::new();
        let src1 = "/".parse::<Buffer>().unwrap();
        let dst = "/a/b/c".parse::<Buffer>().unwrap();
        mapper.add(src1.get(), dst);
        let src2 = "/foo/ter".parse::<Buffer>().unwrap();
        let dst = "/".parse::<Buffer>().unwrap();
        mapper.add(src2.get(), dst);
        assert_eq!("/a/b/c",
                   format!("{}", mapper.translate(src1.get()).unwrap()));
        assert_eq!("", format!("{}", mapper.translate(src2.get()).unwrap()));
    }

    #[test]
    fn fails_missing_paths() {
        let mut mapper = Mapper::new();
        let src1 = "/foo/bar".parse::<Buffer>().unwrap();
        let dst = "/a/b/c".parse::<Buffer>().unwrap();
        mapper.add(src1.get(), dst);
        let src2 = "/foo/ter".parse::<Buffer>().unwrap();
        let dst = "/d/e/f".parse::<Buffer>().unwrap();
        mapper.add(src2.get(), dst);
        let src3 = "/foo/bla".parse::<Buffer>().unwrap();
        assert!(mapper.translate(src3.get()).is_none());
    }

    #[test]
    fn allows_to_redefine() {
        let mut mapper = Mapper::new();
        let src = "/foo/bar".parse::<Buffer>().unwrap();
        let dst1 = "/a/b/c".parse::<Buffer>().unwrap();
        mapper.add(src.get(), dst1);
        assert_eq!("/a/b/c",
                   format!("{}", mapper.translate(src.get()).unwrap()));
        let dst2 = "/d/e/f".parse::<Buffer>().unwrap();
        mapper.add(src.get(), dst2);
        assert_eq!("/d/e/f",
                   format!("{}", mapper.translate(src.get()).unwrap()));
    }
}
