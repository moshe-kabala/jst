// use crate::{Obj, Val};


#[macro_export(json_macros)]
macro_rules! arr {
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
        arr!(@push $array ($($val)*));
        arr!(@flat_expr_or_continue $array ($($rest)*));
    );

    // not continue (there is no rest)
    (@push_and_continue $array:ident  ($($val:tt)+)) => (
        arr!(@push $array  ($($val)*));
    );


    (@next_value $array:ident ($($rest:tt)*))=> (
        val!(@next ($($rest)*) (arr!) (@push_and_continue $array ));
    );


    // catch ...[], and rest
    (@flat_expr_or_continue $array:ident (...[$($val:tt)*], $($rest:tt)*)) => (
        $array.extend(arr![$($val)*].clone());
        arr!(@flat_expr_or_continue $array ($($rest)*));
    );

    // catch ...[]
    (@flat_expr_or_continue $array:ident (...[$($val:tt)*])) => (
        $array.extend(arr![$($val)*].clone());
    );

     // catch ...expr, and rest
    (@flat_expr_or_continue $array:ident (...$val:expr , $($rest:tt)*))=> (
        $array.extend($val.clone());
        arr!(@flat_expr_or_continue $array ($($rest)*));
    );

    // catch ...expr
    (@flat_expr_or_continue $array:ident (...$val:expr)) => (
        $array.extend($val.clone());
    );

    // there is no flat expression, so continue
    (@flat_expr_or_continue $array:ident ($($rest:tt)*)) => (
        arr!(@next_value $array ($($rest)*));
    );

    [$($tt:tt)*] => (
        {
             let mut array: Vec<Val>= Vec::new();
             arr!(@flat_expr_or_continue array ($($tt)*));
             array
        }
    );
}
