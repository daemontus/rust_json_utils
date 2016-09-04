extern crate json_utils;
extern crate rustc_serialize;

use json_utils::*;

use rustc_serialize::json::{Json, ToJson, Object};

//Deserialization tests

#[test]
fn read_i64() {
    let j = (-12).to_json();
    assert_eq!(-12, i64::from_json(&j).unwrap());
}

#[test]
#[should_panic(expected = "ExpectedError(\"i64\", \"null\")")]
fn read_invalid_i64() {
    let null = None::<i64>.to_json();
    i64::from_json(&null).unwrap();
}

#[test]
fn read_u64() {
    let j = 43.to_json();
    assert_eq!(43, u64::from_json(&j).unwrap());
}

#[test]
#[should_panic(expected = "ExpectedError(\"u64\", \"null\")")]
fn read_invalid_u64() {
    let null = None::<u64>.to_json();
    u64::from_json(&null).unwrap();
}

#[test]
fn read_f64() {
    let j = (-3.14).to_json();
    assert_eq!(-3.14, f64::from_json(&j).unwrap());
}

#[test]
#[should_panic(expected = "ExpectedError(\"f64\", \"null\")")]
fn read_invalid_f64() {
    let null = None::<f64>.to_json();
    f64::from_json(&null).unwrap();
}

#[test]
fn read_bool() {
    let j = true.to_json();
    assert_eq!(true, bool::from_json(&j).unwrap());
}

#[test]
#[should_panic(expected = "ExpectedError(\"bool\", \"null\")")]
fn read_invalid_bool() {
    let null = None::<bool>.to_json();
    bool::from_json(&null).unwrap();
}

#[test]
fn read_string() {
    let j = "test".to_string().to_json();
    assert_eq!("test".to_string(), String::from_json(&j).unwrap());
}

#[test]
#[should_panic(expected = "ExpectedError(\"String\", \"null\")")]
fn read_invalid_string() {
    let null = None::<String>.to_json();
    String::from_json(&null).unwrap();
}

#[test]
fn read_vec() {
    let v: Vec<i64> = vec![6,5,4,3,2];
    let j = v.to_json();
    assert_eq!(v, Vec::from_json(&j).unwrap());
}

#[test]
#[should_panic(expected = "ExpectedError(\"Array\", \"null\")")]
fn read_missing_vec() {
    let null = None::<Vec<i64>>.to_json();
    Vec::<i64>::from_json(&null).unwrap();
}

#[test]
#[should_panic(expected = "ExpectedError(\"u64\", \"-4\")")]
fn read_invalid_vec() {
    let null = vec!(2, 3, 4, -4, 5).to_json();
    Vec::<u64>::from_json(&null).unwrap();
}

#[test]
fn read_none() {
    let j = None::<bool>.to_json();
    assert_eq!(None::<bool>, Option::from_json(&j).unwrap());
}

#[test]
fn read_some() {
    let j = Some::<i64>(67).to_json();
    assert_eq!(Some::<i64>(67), Option::from_json(&j).unwrap());
}

#[test]
#[should_panic(expected = "ExpectedError(\"i64\", \"4.12\")")]
fn read_invalid_option() {
    let j = 4.12.to_json();
    Option::<i64>::from_json(&j).unwrap();
}

// JsonMap tests

#[test]
fn read_write_item() {
    let mut map = Object::new();
    map.write_item("test", &12);
    assert_eq!(12, map.read_item::<i64>("test").unwrap());
}

#[test]
#[should_panic(expected = "MissingFieldError(\"test\")")]
fn read_missing_item() {
    let map = Object::new();
    map.read_item::<i64>("test").unwrap();
}

#[test]
fn read_write_optional_item() {
    let mut map = Object::new();
    map.write_item("test", &Some("Hello".to_string()));
    assert_eq!(Some("Hello".to_string()), map.read_optional_item("test").unwrap());
}

#[test]
fn read_missing_optional_item() {
    let mut map = Object::new();
    assert_eq!(None::<f64>, map.read_optional_item("test").unwrap());
    map.write_item("test", &None::<f64>);
    assert_eq!(None::<f64>, map.read_optional_item("test").unwrap());
}

#[test]
fn create_object_test() {
    let result = create_object(|map| {
        map.write_item("int", &13.to_json());
        map.write_item("float", &54.3.to_json());
    });
    let mut map = Object::new();
    map.write_item("int", &13.to_json());
    map.write_item("float", &54.3.to_json());
    assert_eq!(Json::Object(map), result);
}

#[test]
fn as_object_test() {
    let mut map = Object::new();
    let mut map_inner = Object::new();
    map_inner.write_item("test", &"Hello!".to_string());
    map.insert("inner".to_string(), Json::Object(map_inner));
    let result = map.with_key("inner", |json| {
        as_object(json, |obj| {
            obj.read_item::<String>("test")
        })
    });
    assert_eq!("Hello!".to_string(), result.unwrap());
}

#[test]
#[should_panic(expected = "ExpectedError(\"Object\", \"43\")")]
fn as_object_invalid_type_test() {
    let mut map = Object::new();
    map.write_item("test", &43.to_json());
    map.with_key("test", |json| {
        as_object(json, |_| { Ok(12) })
    }).unwrap();
}