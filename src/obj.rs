use std::{self, collections::HashMap, fmt, ops::Index};

use crate::{Parser, ParserErr, Val};

#[derive(PartialEq)]
pub struct Obj(HashMap<String, Val>);

impl Index<&str> for Obj {
    type Output = Val;

    fn index(&self, key: &str) -> &Val {
        let r = self.get(key);
        if let Some(v) = r {
            v
        } else {
            &Val::Null
        }
    }
}

// trait Json: fmt::Display + fmt::Debug {
//     fn parse(str: &str) -> Result<Val, ParserErr>;
//     fn new() -> Self;
//     fn from_map(v: HashMap<String, Val>) -> Self;
//     // fn len(&self) -> usize;
//     // fn get(&self, key: &str) -> Option<&Val>;
//     // fn get_mut(&mut self, key: &str) -> Option<&mut Val>;
//     // fn set(&mut self, key: &str, val: Val) -> Option<Val>;
//     // fn remove(&mut self, key: &str) -> Option<Val>;
//     //fn index(&self, key: &str) -> &Val;
// }

impl Obj {
    pub fn new() -> Self {
        Obj(HashMap::new())
    }

    pub fn parse(str: &str) -> Result<Val, ParserErr> {
        let mut parser = Parser::new(str);
        parser.parse(true)
    }

    pub fn from_map(v: HashMap<String, Val>) -> Self {
        Self(v)
    }
}

impl Obj {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, key: &str) -> Option<&Val> {
        self.0.get(key.into())
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Val> {
        self.0.get_mut(key.into())
    }

    pub fn set(&mut self, key: &str, val: Val) -> Option<Val> {
        self.0.insert(key.into(), val)
    }

    pub fn remove(&mut self, key: &str) -> Option<Val> {
        self.0.remove(key.into())
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Obj {
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
