extern crate json_utils;
extern crate rustc_serialize;

use json_utils::FromJson;
use json_utils::JsonMap;

use rustc_serialize::json::{Json, ToJson, Object, DecoderError};

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
fn read_invalid_vec() {
    let null = None::<Vec<i64>>.to_json();
    Vec::<i64>::from_json(&null).unwrap();
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