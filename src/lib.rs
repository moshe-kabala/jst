mod obj;
mod parser;
mod value;

pub use self::obj::Json;
pub use self::parser::{Parser, ParserErr};
pub use self::value::Value;

#[cfg(test)]
mod tests {

    use crate::{Json, Value};

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
            assert_eq!(v["nested_obj"]["nested1"]["nested2"]["key1"], Value::Str("some value".into()));
            assert_eq!(v["boolean_true"], Value::Bool(true));
            assert_eq!(v["boolean_false"], Value::Bool(false));
            assert_eq!(v["null"], Value::Null);
            assert_eq!(v["array"][0], Value::Num(4564.0));
            assert_eq!(v["array"][1], Value::Str("some string".into()));
            assert_eq!(v["array"][2]["blo"], Value::Str("sfsf".into()));
            assert_eq!(v["array"][7], Value::Array(vec![Value::Num(4.0), Value::Num(5.0)]));

        } else if let Err(e) = from_string {
            // todo ("{:?}", e)
        }
    }
}