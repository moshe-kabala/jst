use num::ToPrimitive;
use std::{self, collections::HashMap, fmt, ops::Index};

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

impl std::convert::From<String> for Value {
    fn from(v: String) -> Value {
        Value::Str(v)
    }
}

impl std::convert::From<&str> for Value {
    fn from(v: &str) -> Value {
        Value::Str(v.into())
    }
}

impl std::convert::From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Value {
        Value::Array(v)
    }
}

impl std::convert::From<HashMap<String, Value>> for Value {
    fn from(v: HashMap<String, Value>) -> Value {
        Value::Obj(Json::from_map(v))
    }
}

impl std::convert::From<Json> for Value {
    fn from(v: Json) -> Value {
        Value::Obj(v)
    }
}

impl std::convert::From<bool> for Value {
    fn from(v: bool) -> Value {
        Value::Bool(v)
    }
}

macro_rules! impl_from_num {
($($type:ident),*) => (
   $(

       impl std::convert::From<$type> for Value {
           fn from(v: $type) -> Value {
               Value::Num(v.to_f64().unwrap())
            }
        }
    )*
)
}

impl_from_num!(usize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

#[macro_export(json_macros)]
macro_rules! value {
 (null) => (Value::Null);
 ([$($val:expr),*]) => (Value::Array(jv![$($val),*]));
 ({$($tt:tt )+}) => (Value::Obj({$($tt),*}));
 ($val:expr) => ($val.into());
}

#[macro_export(json_macros)]
macro_rules! array {
    [$( $val:tt ),*] => (
        {
            let mut v: Vec<Value>= Vec::new();
            $(
               v.push( array!($val));
            )*
            v
        }
     );
}

#[macro_export(json_macros)]
macro_rules! json{



    (@key $key:ident) => (
        stringify!($key)
    );

    (@key $key:expr) => (
        $key.into()
    );

  


    // set
    (@set $json:ident ($($key:tt)+) ($($val:tt)+)) => (
        //$json.set(json!(@key $($key)*), value!($($val)*));
    );

    // // handle with array value
    // (@next_value $json:ident ($($key:tt)+) ([$($val:tt)*], $($rest:tt)*)) => (
    //     json!(@set ($($key)*) ([$($val)*]));
    //     json!(@next_key $json ($($rest:tt)*));
    // );

    // // handle with object value
    // (@next_value $json:ident ($($key:tt)+) ({$($val:tt)*}, $($rest:tt)*)) => (
    //     json!(@set ($($key)*) ({$($val)*}));
    //     json!(@next_key $json ($($rest:tt)*));
    // );

    //  // handle with null value
    //  (@next_value $json:ident ($($key:tt)+) ($val:ident, $($rest:tt)*)) => (
    //     json!(@set ($($key)*) ($val));
    //     json!(@next_key $json ($($rest:tt)*));
    // );

     // handle with expression value
     (@next_value $json:ident ($($key:tt)+) ($val:expr, $($rest:tt)+)) => (
        json!(@set $json ($($key)*) ($val));
        json!(@next_key $json ($($rest)*));
    );

     // handle with expression value
    // (@next_value $json:ident ($($key:tt)+) ($val:expr)) => (
        
    // );

    // catch the last key value
    (@next_value $json:ident ($key:ident) ($($val:tt)+)) => (
        json!(@set ($($key)*) ($($val)*));
    );

    (@next_key $json:ident ($key:ident: $($rest:tt)+)) => (
        json!(@next_value $json ($key) ($($rest)*));
    );

    (@next_key $json:ident (($key:expr): $($rest:tt)+)) => (
        json!(@next_value $json ($key) ($($rest)*));
    );



   {$($tt:tt)+} => (
     {
         let mut j = Json::new();
         json!(@next_key j ($($tt)*));
         j
     }
    );
}
