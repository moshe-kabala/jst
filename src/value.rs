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
    ([$($tt:tt)*]) => (Value::Array(array![$($tt)*]));
    ({$($tt:tt )*}) => (Value::Obj(json!{$($tt)*}));
    (null) => (Value::Null);
    ($val:expr) => ($val.into());

 // call anther macro with next values rules

 //handle with array value
 (@next ([$($val:tt)*], $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
     $($next)*($($args)* ([$($val)*]) ($($rest)+));
 );
 //handle with json value
 (@next ({$($val:tt)*}, $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
     $($next)*($($args)* ({$($val)*}) ($($rest)+));
    );
    //handle with json value
    (@next (null, $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* (null) ($($rest)+));
    );
    //handle with expression value
(@next ($val:expr, $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
    $($next)*($($args)* ($val) ($($rest)+));
 );



  //handle with null value
(@next (null, $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
    $($next)*($($args)* (null) ($($rest)+));
);

 // catch the last value
(@next ($($val:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
    $($next)*($($args)* ($($val)+));
);

}

#[macro_export(json_macros)]
macro_rules! array {
    (@push $array:ident ($($val:tt)+)) => (
        $array.push( value!($($val)*));
     );


    (@push_and_continue $array:ident  ($($val:tt)+) ($($rest:tt)+)) => (
        array!(@push $array ($($val)*));
        array!(@next_value $array ($($rest)*));
    );

    // not continue (there is no rest)
    (@push_and_continue $array:ident  ($($val:tt)+)) => (
        array!(@push $array  ($($val)*));
    );


     (@next_value $array:ident ($($rest:tt)*))=> (
        value!(@next ($($rest)*) (array!) (@push_and_continue $array ));
     );

     [$($tt:tt)*] => (
        {
            let mut array: Vec<Value>= Vec::new();
            array!(@next_value array ($($tt)*));
            array
        }
     );
}

#[macro_export(json_macros)]
macro_rules! json{

    (@key $key:literal) => (
        $key.into()
    );

    (@key $key:ident) => (
        stringify!($key)
    );

    (@key $key:expr) => (
        $key.into()
    );




    // set
    (@set $json:ident ($($key:tt)+) ($($val:tt)+)) => (
        $json.set(json!(@key $($key)*), value!($($val)*));
    );


    (@set_and_continue $json:ident ($($key:tt)+) ($($val:tt)+) ($($rest:tt)+)) => (
        json!(@set $json ($($key)*)  ($($val)*));
        json!(@next_key $json ($($rest)*));
    );

    // not continue (there is no rest)
    (@set_and_continue $json:ident ($($key:tt)+) ($($val:tt)+)) => (
        json!(@set $json ($($key)*) ($($val)*));
    );

     //
    (@next_value $json:ident ($($key:tt)+) ($($rest:tt)+)) => (
        value!(@next ($($rest)*) (json!) (@set_and_continue $json ($($key)*)));
    );

    (@next_key $json:ident ($key:literal: $($rest:tt)+)) => (
        json!(@next_value $json ($key) ($($rest)*));
    );

    (@next_key $json:ident ($key:ident: $($rest:tt)+)) => (
        json!(@next_value $json ($key) ($($rest)*));
    );

    (@next_key $json:ident ([$key:expr] : $($rest:tt)+)) => (
        json!(@next_value $json ($key) ($($rest)*));
    );


   // first station
   {$($tt:tt)+} => (
     {
         let mut j = Json::new();
         json!(@next_key j ($($tt)*));
         j
     }
    );
}
