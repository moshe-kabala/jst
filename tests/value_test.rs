#![recursion_limit = "256"]
#[macro_use(json, value, array)]
extern crate json;

use json::{Json, Value};

#[test]
fn it_works() {
    let from_string = Json::from(
        r#"{
                "number": 56,
                "string": "some string\" string",
                "boolean_true": true,
                "boolean_false": false,
                "null": null,
                "obj": {
                    "key1": 456
                },
                "empty_obj": {},
                "empty_obj_new_line": {

                },
                "nested_obj" : {
                    "nested1": {
                    "nested2": {
                        "key1": "some value",
                        "key2": "anther value"
                    }
                    }
                },
                "array": [4564, "some string", {"bla":90, "blo": "sfsf"}, null, true, false, [], [4,5]],
                "key": 2
                }
            "#,
    );

    if let Ok(Value::Obj(v)) = from_string {
        assert_eq!(v.len(), 11);
        assert_eq!(v["number"], Value::Num(56.0));
        assert_eq!(
            v["nested_obj"]["nested1"]["nested2"]["key1"],
            Value::Str("some value".into())
        );
        assert_eq!(v["boolean_true"], Value::Bool(true));
        assert_eq!(v["boolean_false"], Value::Bool(false));
        assert_eq!(v["null"], Value::Null);
        assert_eq!(v["array"][0], Value::Num(4564.0));
        assert_eq!(v["array"][1], Value::Str("some string".into()));
        assert_eq!(v["array"][2]["blo"], Value::Str("sfsf".into()));
        assert_eq!(
            v["array"][7],
            Value::Array(vec![Value::Num(4.0), Value::Num(5.0)])
        );
    } else if let Err(e) = from_string {
        panic!("{}", e);
    }
}

#[test]
fn check_array_parser() {
    let mut obj = Json::new();
    obj.set("key", Value::Str("val".into()));

    let cases = [
        (
            r#"[3, 5, null]"#,
            Value::Array(vec![Value::Num(3.0), Value::Num(5.0), Value::Null]),
        ),
        (
            r#"[{"key": "val"}, [2]]"#,
            Value::Array(vec![Value::Obj(obj), Value::Array(vec![Value::Num(2.0)])]),
        ),
    ];

    for (value, expected) in cases.iter() {
        let val = Value::parse(value);
        if let Ok(v) = val {
            assert_eq!(v, *expected)
        } else if let Err(e) = val {
            panic!("json: {} ,error: {}", value, e);
        }
    }
}

#[test]
fn test_json_macro() {
    let key = "var_key";

    let person = json! {
        name: "jhon",
        like_rust: true,
        like_go: null,
        emails : [
            "some@gmail.com",
            "some2@gmail.com"
        ],
        address: {
            city: "somewhere",
            zip: 5612
        },
        "literal": true,
        [key]: "var_key",
        age: 56
    };

    // let array = jv!["value in array"];
    assert_eq!(person["name"], value!("jhon"), "person is {:?}", person);
    assert_eq!(person["like_rust"], value!(true), "person is {:?}", person);
    assert_eq!(
        person["var_key"],
        value!("var_key"),
        "person is {:?}",
        person
    );
    assert_eq!(person["literal"], value!(true), "person is {:?}", person);
    assert_eq!(person["age"], value!(56), "person is {:?}", person);
    assert_eq!(
        person["emails"],
        value!(["some@gmail.com", "some2@gmail.com"]),
        "person is {:?}",
        person
    );
    assert_eq!(
        person["address"],
        value!({ city: "somewhere",
    zip: 5612}),
        "person is {:?}",
        person
    );

}
#[test]
fn test_array_macro() {
    let emails = array!["some@gmail.com", "some2@gmail.com"];
    assert_eq!(
        emails,
        vec![value!("some@gmail.com"), value!("some2@gmail.com")]
    );
}

#[test]
fn test_value_macro() {
    let emails = array!["some@gmail.com", "some2@gmail.com"];
    assert_eq!(
        emails,
        vec![value!("some@gmail.com"), value!("some2@gmail.com")]
    );
}
