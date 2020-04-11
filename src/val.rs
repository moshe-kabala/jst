use std::{self, fmt, ops::Index};

use crate::{Obj, Parser, ParserErr};

/// ## Any Object Value options
/// The Val enum wrap the values to make the code
/// more readable for example
/// let obj = val! {
///
/// }
///
/// let result = obj["some key"]["anther key"][2]
///
/// even th
///
///
///
///
///
#[derive(PartialEq, Clone)]
pub enum Val {
    Str(String),
    Obj(Obj),
    Num(f64),
    Array(Vec<Val>),
    Bool(bool),
    Null,
    Undef,
}

impl Index<&str> for Val {
    type Output = Val;

    fn index(&self, key: &str) -> &Val {
        match self {
            Val::Obj(obj) => obj.index(key),
            _ => &Val::Undef,
        }
    }
}

pub fn array_to_str(arr: &Vec<Val>) -> String {
    if arr.len() == 0 {
        "[]".into()
    } else {
        // map the array values
        // join the values with ,\n
        // split into lines
        // add an indentation for each line
        // join and collect the result
        let lines = arr
            .iter()
            .map(|item| format!("{:?}", item))
            .collect::<Vec<String>>()
            .join(",\n")
            .split("\n")
            .map(|line| format!("    {}", line))
            .collect::<Vec<String>>()
            .join("\n");
        format! ("[\n{}\n]", lines)
    }
}

impl Index<usize> for Val {
    type Output = Val;

    fn index(&self, key: usize) -> &Val {
        match self {
            Val::Array(a) => {
                if key >= a.len() {
                    &Val::Undef
                } else {
                    &a[key]
                }
            },
            _ => &Val::Undef,
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

macro_rules! impl_unwrap_and_getter {



    ( $(($type:ty, $ty_name:ident, $mc:pat, $var:ident, $unwrap:ident, $unwrap_or:ident)),*) => (
        $(
            pub fn $unwrap(self)-> $type {
                match self {
                    $mc => $var,
                    _ => panic!("[Val.get_{}] val: {:?} is not a str", self, stringify!($ty_name))
                }
            }
            pub fn $unwrap_or(self, def: $type)-> $type {
                match self {
                    $mc => $var,
                    _ => def
                }
            }
        )*
    )
}

impl Val {
    pub fn parse(str: &str) -> Result<Val, ParserErr> {
        let mut parser = Parser::new(str);
        parser.parse(true)
    }

    impl_unwrap_and_getter!(
        (String, str, Val::Str(v), v, unwrap_str, unwrap_str_or),
        (f64, num, Val::Num(v), v, unwrap_num, unwrap_num_or),
        (bool, bool, Val::Bool(v), v, unwrap_bool, unwrap_bool_or),
        (Vec<Val>, arr, Val::Array(v), v, unwrap_arr, unwrap_arr_or),
        (Obj, obj, Val::Obj(v), v, unwrap_obj, unwrap_obj_or)
    );
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
            Val::Array(v) => array_to_str(v),
            Val::Null => "null".into(),
            Val::Undef => "undefined".into(),
        };

        write!(f, "{}", val)
    }
}
