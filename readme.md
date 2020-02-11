# Json

The package includes some functionalities to  serializing and deserializing json with rust

1. Json data structure
2. Json parser
3. Json to string

# Example
```

use json::{Value, Json};

fn main() {

  let mut j = Json::new();

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


  // Print either json or err
  if let Ok(Value::Obj(v)) = from_string {
    println!("{:?}", v);
  } else if let Err(e) = from_string {
    println!("{:?}", e)
  }
}
```

```
let val = Value::from("[3, 5, null]");

if let Ok(v) = val) {
  assert_eq!(v, Value::Array(vec![Value::Num(3.0), Value::Num(3.0), Value::Null])) // true
}
```


## Todo

1. Improve Tests
2. Improve indexer (solve range indexer for string and array)
3. Adding Error code
4. Adding json macro

# Deceleration
The code written only for practicing rust. if you looking for a json library take a look at [Serde Json](https://github.com/serde-rs/json)

# Licence
MIT
