// use crate::{Obj, Val};

#[macro_export(json_macros)]
macro_rules! obj{
    // empty object
    {} => (
        Obj::new()
    );

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
        $json.set(obj!(@key $($key)*), val!($($val)*));
    );

    (@set_and_continue $json:ident ($($key:tt)+) ($($val:tt)+) ($($rest:tt)+)) => (
        obj!(@set $json ($($key)*)  ($($val)*));
        obj!(@flat_expr_or_continue $json ($($rest)*));
    );

    // not continue (there is no rest)
    (@set_and_continue $json:ident ($($key:tt)+) ($($val:tt)+)) => (
        obj!(@set $json ($($key)*) ($($val)*));
    );

     //
    (@next_value $json:ident ($($key:tt)+) ($($rest:tt)+)) => (
        val!(@next ($($rest)*) (obj!) (@set_and_continue $json ($($key)*)));
    );

    // handle with key without value
    // let name = "some key";
    // let age = 45;
    // {
    //    key,
    //    age
    // }
    //
    (@next_key $json:ident ($key:ident, $($rest:tt)+)) => (
        obj!(@next_value $json ($key) ($key,$($rest)*));
    );

    (@next_key $json:ident ($key:ident)) => (
        obj!(@next_value $json ($key) ($key));
    );

    (@next_key $json:ident ($key:literal: $($rest:tt)+)) => (
        obj!(@next_value $json ($key) ($($rest)*));
    );

    (@next_key $json:ident ($key:ident: $($rest:tt)+)) => (
        obj!(@next_value $json ($key) ($($rest)*));
    );

    (@next_key $json:ident ([$key:expr] : $($rest:tt)+)) => (
        obj!(@next_value $json ($key) ($($rest)*));
    );


    // catch ...{}, and rest
    (@flat_expr_or_continue $json:ident (...{$($val:tt)*}, $($rest:tt)*)) => (
        $json.extend(obj!{$($val)*}.clone().into_iter());
        obj!(@flat_expr_or_continue $json ($($rest)*));
    );

    // catch ...{}
    (@flat_expr_or_continue $json:ident (...{$($val:tt)*})) => (
        $json.extend(obj!{$($val)*}.clone().into_iter());
    );

    // catch ...expr, and rest
    (@flat_expr_or_continue $json:ident (...$val:expr , $($rest:tt)*))=> (
        $json.extend($val.clone().into_iter());
        obj!(@flat_expr_or_continue $json ($($rest)*));
    );

    // catch ...expr
    (@flat_expr_or_continue $json:ident (...$val:expr)) => (
        $json.extend($val.clone().into_iter());
    );

    // there is no flat expression, so continue
    (@flat_expr_or_continue $json:ident ($($rest:tt)*)) => (
        obj!(@next_key $json ($($rest)*));
    );


   // first station
   {$($tt:tt)*} => (
     {
         let mut j = Obj::new();
         obj!(@flat_expr_or_continue j ($($tt)*));
         j
     }
    );
}
