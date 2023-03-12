use std::{collections::HashSet, rc::Rc};

/// Allocates reference counted string slices.
/// Saves memory when a lot of duplicate owned strings have to be used.
#[derive(Default)]
pub struct StringPool {
    cache: HashSet<Rc<str>>,
}

impl StringPool {
    /// Returns a reference counted string slice, returning references to
    /// same buffers for same string contents. Allocates a new owned string
    /// if neccessary.
    pub fn resolve(&mut self, string: &str) -> Rc<str> {
        match self.cache.get(string) {
            Some(rc) => rc.clone(),
            None => {
                let rc = Rc::<str>::from(string);
                self.cache.insert(rc.clone());
                rc
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StringPool;

    #[test]
    fn returns_same_buffers_for_same_strings() {
        let mut sp = StringPool::default();
        let rc1 = sp.resolve("test");
        let rc2 = sp.resolve("test");
        assert_eq!(rc1.as_ptr(), rc2.as_ptr());
    }

    #[test]
    fn returns_different_buffers_for_different_strings() {
        let mut sp = StringPool::default();
        let rc1 = sp.resolve("test1");
        let rc2 = sp.resolve("test2");
        assert_ne!(rc1.as_ptr(), rc2.as_ptr());
    }
}
