// use crate::{Obj, Val};

///
/// ## Value Macro
/// Create json::Val enum from any valid json format
///
/// ### examples
// / ```
// / #[macro_use(val, obj, array)]
// / extern crate json;
// /
// / use json::{Val, Obj};
// /
// / let str = val!("some string");
// / let num = val!(45);
// / let bool = val!(true);
// / let null = val!(null);
// / let array = val!([
// /     "string",
// /     45,
// /     null,
// /     [{key: "val"}, undefined]
// / ]);
// /
// / let json = val!({
// /   key:"string",
// /   num:45
// / });
// / ```
///
#[macro_export(json_macros)]
macro_rules! des {
    

    {@set_let_mut_obj ($c:expr) ($($obj:tt)+) ($var:ident, $($tt:tt)+)} => {
        let name = val!("avi");
        let mut  $var = &mut $($obj)*[stringify!($var)];
        des!{@set_let_mut_obj ($c+1) ($($obj)+) ($($tt)*)};
    };

    {@set_let_mut_obj ($c:expr) ($($obj:tt)+) ($var:ident)} => {
        let mut $var = $($obj)*[stringify!($var)];
    };

    {@destructing let mut {$($tt:tt)*} = $obj:expr; $($rest:tt)*} => (
        des!{@set_let_mut_obj (0) ($obj) ($($tt)*)};
        des!(@destructing $($rest)*); 
    );

   

    {@destructing let {$($tt:tt)*} = $obj:expr; $($rest:tt)*} => (

        //des!(@destructing $($rest)*);
    );

    {@destructing let mut {$($tt:tt)*} = $obj:expr;} => (
        des!{@set_let_mut_obj (0) ($obj) ($($tt)*)};
    );

    {@destructing let {$($tt:tt)*} = $obj:expr;} => (

        des!(@destructing $($rest)*);
    );

    {@destructing let mut [$($tt:tt)*] = $obj:expr; $($rest:tt)*} => (
        
        des!(@destructing $($rest)*);
    );

    {@destructing let mut [$($tt:tt)*] = $obj:expr; } => (
        
        des!(@destructing $($rest)*);
    );

    {@destructing let [$($tt:tt)*] = $obj:expr; $($rest:tt)*} => (  
        des!(@destructing $($rest)*);
    );

    {@destructing let [$($tt:tt)*] = $obj:expr;} => (  
        des!(@destructing $($rest)*);
    );

    //the iteration end here
    {@destructing $st:stmt; $($rest:tt)*}=> (
        $st;
        des!(@destructing $($rest)*);
    );

    // the iteration end here
    {@destructing} => (
        
    );

    // the first station
    {$($tt:tt)+} => {
        des!(@destructing $($tt)+);
    }
}
