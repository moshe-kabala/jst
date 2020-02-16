use std::{self, fmt, ops::Index};

use crate::{Json, Parser, ParserErr};

#[derive(PartialEq)]
pub enum Value {
    Str(String),
    Obj(Json),
    Num(f64),
    Array(Vec<Value>),
    Bool(bool),
    Null,
}

impl Index<&str> for Value {
    type Output = Value;

    fn index(&self, key: &str) -> &Value {
        match self {
            Value::Obj(obj) => obj.index(key),
            _ => &Value::Null,
        }
    }
}

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, key: usize) -> &Value {
        match self {
            Value::Array(a) => &a[key],
            _ => &Value::Null,
        }
    }
}

// todo adding range index
// impl Index<Range<usize>> for Value {
//     type Output = Value;

//     fn index(&self, key: Range<usize>) -> &Self::Output {
//         // todo
//         //Value::Array(a) => &Value::Array(Vec::from(&a[key])),
//         match self {
//             Value::Str(s) => &Value::Str(s[key].into()),
//             _ => &Value::Null,
//         }
//     }
// }

impl Value {
    pub fn parse(str: &str) -> Result<Value, ParserErr> {
        let mut parser = Parser::new(str);
        parser.parse(true)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val: String = match self {
            Value::Bool(v) => {
                if *v {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            Value::Num(v) => format!("{}", v),
            Value::Obj(v) => format!("{:?}", v),
            Value::Str(v) => format!("{:?}", v),
            Value::Array(v) => {
                if v.len() == 0 {
                    String::from("[]")
                } else {
                    // map the array values
                    // join the values with ,\n
                    // split into lines
                    // add an indentation for each line
                    // join and collect the result
                    let lines = v
                        .iter()
                        .map(|item| format!("{:?}", item))
                        .collect::<Vec<String>>()
                        .join(",\n")
                        .split("\n")
                        .map(|line| format!("    {}", line))
                        .collect::<Vec<String>>()
                        .join("\n");
                    format!("[\n{}\n]", lines)
                }
            }
            Value::Null => "null".into(),
        };

        write!(f, "{}", val)
    }
}
