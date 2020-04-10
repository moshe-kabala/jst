# Jst

The package includes:

1. JSON data structure
2. JSON parser
3. JSON to string
4. Macros to writing JSON same as a Javascript object

# Convertors and Macros

## Macros

The package support obj!, val!, and arr! macros to write any json object in much more convinces way ( Write like JavaScript Object syntax and getting completion error).

### Json macro

```
    // basic usage

    let dog = obj! {
      color: "brown",
      type: "Akbash",
      eating : [
        "WholeHearted",
        "Royal Canin"
      ]
    };

    // advance usage

    let key = "var_key";
    let age = 45;
    let like_banana = true;

    let person = obj! {
        name: "jhon",
        // use value name as a key
        age,
        like_banana,
        like_rust: true,
        like_go: null,
        emails : [
            "some@gmail.com",
            "some2@gmail.com"
        ],
        // you can flat obj into - the dog is copy not moved
        ...dog
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

Create json::Val enum from any valid json format

```

let str = val!("some string");
let num = val!(45);
let bool = val!(true);
let null = val!(null);
let array = val!(["string", 45]);

let json = val!({
  key:"string",
  num:45
});

```

### array macro

Create a Vec\<json::Val\> vector.

```
// the type is Vec<Val>
let arr = arr![
  "string",
  45,
  true,
  [],
  {key: "value"}
];


// Extend the arr2 by ...arr
let arr2 = arr![
  ...arr,
  "val"
];

```

## Convertors

Form any numerical number, String, &str, boolean and HashMap<String, json::Val> you can call 'into()' method to convert the value to Obj::Val enum

```
let str:Val = "some string".into();
let num:Val = 78.into();
let bool:Val = true.into();
let array:Val = vec!["string".into(), 45.into()];

// the short way is to use macros (obj!, val! and arr!)

let str = val!("some string");
let num = val!(54);
let bool = val!(true);
let null = val!(null);
let array_val = val!(["string", 45]);
// the type is Vec<Val>
let array = arr!["string", 45];

let obj_val = val!({
  key:"string",
  num:45
});
// the type is Object
let obj = obj!{
  key:"string",
  num:45
};


```

# Parser

Parse a json from a string

```
use json::{Val, Json};

fn main() {

  let mut j = Obj::new();

  let from_string = Obj::from(
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
  if let Ok(Val::Obj(v)) = from_string {
    println!("{:?}", v);
  } else if let Err(e) = from_string {
    println!("{:?}", e)
  }
}
```

```
let val = Val::from("[3, 5, null]");

if let Ok(v) = val) {
  assert_eq!(v, Val::Array(vec![Val::Num(3.0), Val::Num(5.0), Val::Null])) // true
}
```

# Deceleration

The code not ready for production. if you looking for a json library take a look at [Serde Json](https://github.com/serde-rs/json)

## Todo

1. Improve parser tests
2. Improve indexer (enable to set a value by index)
3. Adding Error code
4. Serialize and Deserialize
5. macro for destructuring object and array
6. Adding json-schema validator
7. Support yml format
8. Option to use share pointer for object and array

# Licence

MIT
