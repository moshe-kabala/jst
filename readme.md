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





# Converters and Macros
## Macros
The package support json!, value! and array macros to write any json object in much more convinces way (instead of string). 

### Json macro
```
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
```

### value macro
Create any json value enum
```

let str = value!("some string");
let num = value!("some string");
let bool = value!("some string");
let array = value!(["string", 45]);


let json = value!({
  key:"string", 
  num:45
});

```

### array macro
Create a Vec<json::Value> vector.
```
// the type is Vec<Value>
let array2 = array!["string", 45, true, [], {key: "value"}];
```
## Conversion
Form any numerical number, String, &str, boolean and HashMap<String, json::Value> you can call 'into()' method to convert the value to Json Value enum

```
let str:Value = "some string".into();
let num:Value = 78.into();
let bool:Value = true.into();
let array:Value = vec!["string".into(), 45.into()];

// the short way is to use macros (json!, value! and array!)

let str = value!("some string");
let num = value!("some string");
let bool = value!("some string");
let array = value!(["string", 45]);
// the type is Vec<Value>
let array2 = array!["string", 45];

let json = value!({
  key:"string", 
  num:45
});
// the type is Json
let json2 = json!{
  key:"string", 
  num:45
};


```


## Todo

1. Improve parser tests
2. Improve indexer (enable to set a value by index)
3. Adding Error code
4. Adding more tests for json, array and value macros
5. Adding json-schema validator

# Deceleration
The code not ready for production. if you looking for a json library take a look at [Serde Json](https://github.com/serde-rs/json)

# Licence
MIT
