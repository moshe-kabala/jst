#[macro_use(json, value, array)]
extern crate json;

use json::{Json, Value};

#[test]
fn test_json_macro() {
    let key = "var_key";
    let var_value = "var_val";

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
        // nested: {
        //     nested2:{
        //         // todo fix the bug in value
        //         // key: ["value"]
        //     }
        // },
        from_var: var_value,
        from_macro: value!("from_macro string"),
        from_value_var: Value::Str("from_value_var string".into()),
        // todo:
        // empty_obj: {},
        // empty_array: [],
        bool: true,
        "literal": true,
        [key]: "var_key",
        age: 56
    };

    // let array = jv!["value in array"];
    assert_eq!(person["name"], value!("jhon"));
    assert_eq!(person["from_var"], value!("var_val"));
    assert_eq!(person["from_macro"], value!("from_macro string"));
    assert_eq!(person["from_value_var"], value!("from_value_var string"));
    assert_eq!(person["bool"], value!(true));
    assert_eq!(person["like_rust"], value!(true));
    assert_eq!(
        person["var_key"],
        value!("var_key"),
        "person is {:?}",
        person
    );
    assert_eq!(person["literal"], value!(true));
    assert_eq!(person["age"], value!(56));
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
