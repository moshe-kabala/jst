use crate::{Json, Value};
use num::ToPrimitive;
use std::{self, collections::HashMap};

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
