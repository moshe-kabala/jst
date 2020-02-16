use std::{self, fmt, ops::Index};

use crate::{Obj, Parser, ParserErr};

#[derive(PartialEq)]
pub enum Val {
    Str(String),
    Obj(Obj),
    Num(f64),
    Array(Vec<Val>),
    Bool(bool),
    Null,
    Undefined
}

impl Index<&str> for Val {
    type Output = Val;

    fn index(&self, key: &str) -> &Val {
        match self {
            Val::Obj(obj) => obj.index(key),
            _ => &Val::Undefined,
        }
    }
}

impl Index<usize> for Val {
    type Output = Val;

    fn index(&self, key: usize) -> &Val {
        match self {
            Val::Array(a) => &a[key],
            _ => &Val::Undefined,
        }
    }
}

// todo adding range index
// impl Index<Range<usize>> for Val {
//     type Output = Val;

//     fn index(&self, key: Range<usize>) -> &Self::Output {
//         // todo
//         //Val::Array(a) => &Val::Array(Vec::from(&a[key])),
//         match self {
//             Val::Str(s) => &Val::Str(s[key].into()),
//             _ => &Val::Null,
//         }
//     }
// }

impl Val {
    pub fn parse(str: &str) -> Result<Val, ParserErr> {
        let mut parser = Parser::new(str);
        parser.parse(true)
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val: String = match self {
            Val::Bool(v) => {
                if *v {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            Val::Num(v) => format!("{}", v),
            Val::Obj(v) => format!("{:?}", v),
            Val::Str(v) => format!("{:?}", v),
            Val::Array(v) => {
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
            Val::Null => "null".into(),
            Val::Undefined => "".into(),
        };

        write!(f, "{}", val)
    }
}
