#[macro_use(json, value, array)]
extern crate json;

use json::{Json, Value};

#[test]
fn test_value_macro() {
    let _str = value!("some string");
    let _num = value!(54);
    let _bool = value!(true);
    let _null = value!(null);
    let _array = value!(["string", 45]);

    let _json = value!({
      key:"string",
      num:45
    });

    //todo
    // let empty_json = value!({});
    // let empty_array = value!([]);

    assert_eq!(_str, Value::Str("some string".into()));
    assert_eq!(_num, Value::Num(54.0));
    assert_eq!(_bool, Value::Bool(true));
    assert_eq!(_null, Value::Null);
    assert_eq!(_array, Value::Array(vec![value!("string"), value!(45)]));
    assert_eq!(
        _json,
        Value::Obj(json! {
          key:"string",
          num:45
        })
    );
}
