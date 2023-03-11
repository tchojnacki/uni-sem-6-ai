use std::{collections::HashSet, rc::Rc};

#[derive(Default)]
pub struct StringPool {
    cache: HashSet<Rc<str>>,
}

impl StringPool {
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
