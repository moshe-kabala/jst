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
macro_rules! val {
    ([]) => (Val::Array(array![]));
    ({}) => (Val::Obj(obj!{}));
    ([$($tt:tt)*]) => (Val::Array(array![$($tt)*]));
    (null) => (Val::Null);
    (undefined) => (Val::Undef);
    ({$($tt:tt)*}) => (Val::Obj(obj!{$($tt)*}));
    ($val:expr) => ( Val::from($val));

    // call anther macro with next values rules

    // handle with json value
    // the order is important, the rule should be before the next json rule
    // the reason is for the following value:
    // {
    //     nested: {
    //          nested2: {key:val},
    //          key:v
    //     }
    // }
    //
    // the second rule will catch
    // {
    //     nested2: {key:val},
    //
    // instead of
    //      {
    //          nested2: {key:val},
    //          key:v
    //     }
    (@next ({$($val:tt)*}) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* ({$($val)*}));
    );
    //handle with array value
    (@next ([$($val:tt)*]) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* ([$($val)*]));
    );

    //handle with array value
    (@next ([$($val:tt)*], $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* ([$($val)*]) ($($rest)+));
    );
    //handle with json value
    (@next ({$($val:tt)*}, $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* ({$($val)*}) ($($rest)+));
    );
    //handle with null value
    (@next (null, $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* (null) ($($rest)+));
    );
     //handle with undefined value
     (@next (undefined, $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* (undefined) ($($rest)+));
    );
    //handle with expression value
    (@next ($val:expr, $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* ($val) ($($rest)+));
    );

      // catch the last value
    //handle with json value
    (@next ({$($val:tt)*}) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* ({$($val)*}));
    );
    //handle with null value
    (@next (null) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* (null));
    );
    //handle with undefined value
    (@next (undefined) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* (undefined));
    );
    //handle with expression value
    (@next ($val:expr) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* ($val));
    );




}

#[macro_export(json_macros)]
macro_rules! array {
    // empty array
    [] => (
        {
           let v:Vec<Val> = vec![];
           v
        }
    );

    (@push $array:ident ($($val:tt)+)) => (
        $array.push( val!($($val)*));
     );


    (@push_and_continue $array:ident  ($($val:tt)+) ($($rest:tt)+)) => (
        array!(@push $array ($($val)*));
        array!(@flat_expr_or_continue $array ($($rest)*));
    );

    // not continue (there is no rest)
    (@push_and_continue $array:ident  ($($val:tt)+)) => (
        array!(@push $array  ($($val)*));
    );


    (@next_value $array:ident ($($rest:tt)*))=> (
        val!(@next ($($rest)*) (array!) (@push_and_continue $array ));
    );


    // catch ...[], and rest
    (@flat_expr_or_continue $array:ident (...[$($val:tt)*], $($rest:tt)*)) => (
        $array.extend(array![$($val)*].clone());
        array!(@flat_expr_or_continue $array ($($rest)*));
    );

    // catch ...[]
    (@flat_expr_or_continue $array:ident (...[$($val:tt)*])) => (
        $array.extend(array![$($val)*].clone());
    );

     // catch ...expr, and rest
    (@flat_expr_or_continue $array:ident (...$val:expr , $($rest:tt)*))=> (
        $array.extend($val.clone());
        array!(@flat_expr_or_continue $array ($($rest)*));
    );

    // catch ...expr
    (@flat_expr_or_continue $array:ident (...$val:expr)) => (
        $array.extend($val.clone());
    );

    // there is no flat expression, so continue
    (@flat_expr_or_continue $array:ident ($($rest:tt)*)) => (
        array!(@next_value $array ($($rest)*));
    );

    [$($tt:tt)*] => (
        {
             let mut array: Vec<Val>= Vec::new();
             array!(@flat_expr_or_continue array ($($tt)*));
             array
        }
    );
}

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
