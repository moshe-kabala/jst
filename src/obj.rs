use std::{self, collections::HashMap, fmt, ops::Index};

use crate::{Parser, ParserErr, Value};

#[derive(PartialEq)]
pub struct Json(HashMap<String, Value>);

impl Index<&str> for Json {
    type Output = Value;

    fn index(&self, key: &str) -> &Value {
        let r = self.get(key);
        if let Some(v) = r {
            v
        } else {
            &Value::Null
        }
    }
}

impl Json {
    pub fn new() -> Self {
        Json(HashMap::new())
    }

    pub fn from(str: &str) -> Result<Value, ParserErr> {
        let mut parser = Parser::new(str);
        parser.parse(true)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key.into())
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.0.get_mut(key.into())
    }

    pub fn set(&mut self, key: &str, val: Value) -> Option<Value> {
        self.0.insert(key.into(), val)
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.0.remove(key.into())
    }
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() == 0 {
            write!(f, "{{}}")
        } else {
            // map the key and value
            // join the values with \n
            // split into lines
            // add an indentation for each line
            // join and collect the result
            let lines = self
                .0
                .iter()
                .map(|(key, val)| format!("{:?}: {:?}", key, val))
                .collect::<Vec<String>>()
                .join(",\n")
                .split("\n")
                .map(|line| format!("    {}", line))
                .collect::<Vec<String>>()
                .join("\n");

            write!(f, "{{\n{}\n}}", lines)
        }
    }
}
