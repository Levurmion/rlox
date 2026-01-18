use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LoxObject {
    pub properties: HashMap<String, LoxValue>,
}

#[derive(Debug, Clone)]
pub enum LoxValue {
    Number(f64),
    Boolean(bool),
    String(String),
    Object(LoxObject),
}
