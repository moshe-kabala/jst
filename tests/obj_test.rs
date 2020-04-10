#[macro_use(obj, val, arr)]
extern crate jst;

use jst::{Obj, Val};

#[test]
fn test_json_macro() {
    let key = "var_key";
    let var_value = "var_val";

    let key_only = "not put the value";

    let some_object = obj! {
        some_object_flatten_key: "value"
    };

    let last_flat_from_obj = obj! {
        last_flat_key: "value"
    };

    let person = obj! {
        name: "jhon",
        like_rust: true,
        like_go: null,
        // put only the variable name instead 'key_only: key_only'
        key_only,
        emails : [
            "some@gmail.com",
            "some2@gmail.com"
        ],
        address: {
            city: "somewhere",
            zip: 5612
        },
        nested: {
            nested2:{
                key: "value",
                nested3: {
                    key: "value"
                }
            }
        },
        from_var: var_value,
        from_macro: val!("from_macro string"),
        from_value_var: Val::Str("from_value_var string".into()),
        empty_obj: {},
        // Like javascript you can flat obj into anther
        // NOTE the object is CLONE into
        ...{
            flat_key: "value",
            nested_flat_into: {
                nested2: {
                    key: "value"
                }
            },
            ...last_flat_from_obj
        },
        // flat from expression
        ...some_object,
        empty_array: [],
        bool: true,
        "literal": true,
        [key]: "var_key",
        age: 56,
        ...{last_flat: "value"}
    };

    let empty_obj = obj! {};

    // let array = jv!["value in array"];
    assert_eq!(person["name"], val!("jhon"));
    assert_eq!(person["from_var"], val!("var_val"));
    assert_eq!(person["from_macro"], val!("from_macro string"));
    assert_eq!(person["from_value_var"], val!("from_value_var string"));
    assert_eq!(person["bool"], val!(true));
    assert_eq!(person["like_rust"], val!(true));
    assert_eq!(person["var_key"], val!("var_key"), "person is {:?}", person);
    assert_eq!(person["literal"], val!(true));
    assert_eq!(person["key_only"], val!("not put the value"));
    assert_eq!(person["age"], val!(56));
    assert_eq!(
        person["emails"],
        val!(["some@gmail.com", "some2@gmail.com"]),
        "person is {:?}",
        person
    );
    assert_eq!(
        person["address"],
        val!({ city: "somewhere",
    zip: 5612}),
        "person is {:?}",
        person
    );

    assert_eq!(person["flat_key"], val!("value"));
    assert_eq!(person["nested_flat_into"]["nested2"]["key"], val!("value"));
    assert_eq!(person["some_object_flatten_key"], val!("value"));

    assert_eq!(person["last_flat_key"], val!("value"));

    assert_eq!(empty_obj, Obj::new());
}
