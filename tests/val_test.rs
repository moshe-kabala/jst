#[macro_use(obj, val, arr)]
extern crate jst;

use jst::{Obj, Val};

#[test]
fn test_value_macro() {
  let _str = val!("some string");
  let _num = val!(54);
  let _bool = val!(true);
  let _null = val!(null);
  let _array = val!(["string", 45]);

  let _json = val!({
    key:"string",
    num:45
  });

  //todo
  // let empty_json = val!({});
  // let empty_array = val!([]);

  assert_eq!(_str, Val::Str("some string".into()));
  assert_eq!(_num, Val::Num(54.0));
  assert_eq!(_bool, Val::Bool(true));
  assert_eq!(_null, Val::Null);
  assert_eq!(_array, Val::Array(vec![val!("string"), val!(45)]));
  assert_eq!(
    _json,
    Val::Obj(obj! {
      key:"string",
      num:45
    })
  );
}
