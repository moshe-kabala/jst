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
    ([]) => (Val::Array(arr![]));
    ({}) => (Val::Obj(obj!{}));
    ([$($tt:tt)*]) => (Val::Array(arr![$($tt)*]));
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
