use std::collections::HashMap;

use crate::interpreter::interner::InternedString;

#[derive(Debug, Clone)]
pub struct LoxObject {
    pub properties: HashMap<InternedString, LoxValue>,
}

#[derive(Debug, Clone)]
pub enum LoxValue {
    Number(f64),
    Boolean(bool),
    String(InternedString),
    Object(LoxObject),
}
