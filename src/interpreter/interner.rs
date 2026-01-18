use std::{collections::HashMap, rc::Rc};

pub type InternedString = usize;

pub struct Interner {
    map: HashMap<String, InternedString>,
    strings: Vec<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            strings: Vec::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> InternedString {
        match self.map.get(s) {
            Some(interned) => *interned,
            None => {
                let interned = self.strings.len();
                self.strings.push(s.to_string());
                self.map.insert(s.to_string(), interned);
                interned
            }
        }
    }

    pub fn resolve(&self, interned: InternedString) -> &str {
        &self.strings[interned]
    }
}
