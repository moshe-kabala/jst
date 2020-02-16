#[macro_use(json, value, array)]
extern crate json;

use json::{Json, Value};

#[test]
fn test_json_parser() {
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
        assert_eq!(v["number"], value!(56.0));
        assert_eq!(
            v["nested_obj"]["nested1"]["nested2"]["key1"],
            value!("some value")
        );
        assert_eq!(v["boolean_true"], value!(true));
        assert_eq!(v["boolean_false"], value!(false));
        assert_eq!(v["null"], value!(null));
        assert_eq!(v["array"][0], value!(4564.0));
        assert_eq!(v["array"][1], value!("some string"));
        assert_eq!(v["array"][2]["blo"], value!("sfsf"));
        assert_eq!(v["array"][7], value!([4, 5]));
    } else if let Err(e) = from_string {
        panic!("{}", e);
    }
}

#[test]
fn test_array_parser() {
    let cases = [
        (r#"[3, 5, null]"#, value!([3, 5, null])),
        (r#"[{"key": "val"}, [2]]"#, value!([{key: "val"}, [2]])),
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
