// use crate::{Json, Value};

#[macro_export(json_macros)]
macro_rules! value {
    ([]) => (Value::Array(array![]));
    ({}) => (Value::Obj(json!{}));
    ([$($tt:tt)*]) => (Value::Array(array![$($tt)*]));
    (null) => (Value::Null);
    ({$($tt:tt )*}) => (Value::Obj(json!{$($tt)*}));
    ($val:expr) => ( Value::from($val));

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
    // empty array
    [] => (
        {
           let v:Vec<Value> = vec![];
           v
        }
    );

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
    // empty object
    {} => (
        Json::new()
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
   {$($tt:tt)*} => (
     {
         let mut j = Json::new();
         json!(@next_key j ($($tt)*));
         j
     }
    );
}
