use crate::{Obj, Val};
use num::ToPrimitive;
use std::{self, collections::HashMap};

impl std::convert::From<String> for Val {
    fn from(v: String) -> Val {
        Val::Str(v)
    }
}

impl std::convert::From<&str> for Val {
    fn from(v: &str) -> Val {
        Val::Str(v.into())
    }
}

impl std::convert::From<Vec<Val>> for Val {
    fn from(v: Vec<Val>) -> Val {
        Val::Array(v)
    }
}

impl std::convert::From<HashMap<String, Val>> for Val {
    fn from(v: HashMap<String, Val>) -> Val {
        Val::Obj(Obj::from_map(v))
    }
}

impl std::convert::From<Obj> for Val {
    fn from(v: Obj) -> Val {
        Val::Obj(v)
    }
}

impl std::convert::From<bool> for Val {
    fn from(v: bool) -> Val {
        Val::Bool(v)
    }
}

macro_rules! impl_from_num {
($($type:ident),*) => (
   $(

       impl std::convert::From<$type> for Val {
           fn from(v: $type) -> Val {
               Val::Num(v.to_f64().unwrap())
            }
        }
    )*
)
}

impl_from_num!(usize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
