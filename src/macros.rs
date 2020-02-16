// use crate::{Obj, Val};

#[macro_export(json_macros)]
macro_rules! val {
    ([]) => (Val::Array(array![]));
    ({}) => (Val::Obj(obj!{}));
    ([$($tt:tt)*]) => (Val::Array(array![$($tt)*]));
    (null) => (Val::Null);
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
        println!("next_e {}", stringify!({$($val)*}));
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
    //handle with expression value
    (@next ($val:expr, $($rest:tt)+) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* ($val) ($($rest)+));
    );

      // catch the last value
    //handle with json value
    (@next ({$($val:tt)*}) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* ({$($val)*}));
    );
    //handle with json value
    (@next (null) ($($next:tt)+) ($($args:tt)+)) => (
        $($next)*($($args)* (null));
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
        array!(@next_value $array ($($rest)*));
    );

    // not continue (there is no rest)
    (@push_and_continue $array:ident  ($($val:tt)+)) => (
        array!(@push $array  ($($val)*));
    );


     (@next_value $array:ident ($($rest:tt)*))=> (
        val!(@next ($($rest)*) (array!) (@push_and_continue $array ));
     );

     [$($tt:tt)*] => (
         {
             let mut array: Vec<Val>= Vec::new();
             array!(@next_value array ($($tt)*));
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
        obj!(@next_key $json ($($rest)*));
    );

    // not continue (there is no rest)
    (@set_and_continue $json:ident ($($key:tt)+) ($($val:tt)+)) => (
        obj!(@set $json ($($key)*) ($($val)*));
    );

     //
    (@next_value $json:ident ($($key:tt)+) ($($rest:tt)+)) => (
        val!(@next ($($rest)*) (obj!) (@set_and_continue $json ($($key)*)));
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


   // first station
   {$($tt:tt)*} => (
     {
         let mut j = Obj::new();
         obj!(@next_key j ($($tt)*));
         j
     }
    );
}
