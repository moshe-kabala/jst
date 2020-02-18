#[macro_use(obj, val, array)]
extern crate json;

use json::{Obj, Val};

#[test]
fn test_array_macro() {
    let array = arr![
        "some@gmail.com",
        45,
        ["value", 5],
        {key: "value"},
        5,
        [],
        ...[23, "some val"],
        ...arr![45,43],
        {nested: {key: "value"}},
        "some value",
        // nested array
        [[["val"], "val"],[], null],
        null,
        ...["some string",[]]
    ];

    let empty_array = arr![];

    assert_eq!(
        array,
        vec![
            val!("some@gmail.com"),
            val!(45),
            vec![val!("value").into(), 5.into()].into(),
            val!({key: "value"}),
            5.into(),
            vec![].into(),
            val!(23),
            val!("some val"),
            val!(45),
            val!(43),
            val!({nested: {key: "value"}}),
            "some value".into(),
            vec![
                vec![vec!["val".into()].into(), "val".into()].into(),
                vec![].into(),
                Val::Null
            ]
            .into(),
            val!(null),
            "some string".into(),
            vec![].into()
        ]
    );
    let expected_empty_array: Vec<Val> = vec![];
    assert_eq!(empty_array, expected_empty_array);
}
