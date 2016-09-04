extern crate rustc_serialize;

use rustc_serialize::json::{Json, ToJson, Object, DecoderError};

use rustc_serialize::json::DecoderError::*;

pub trait FromJson<T> {
    ///Construct new instance of T from a json struct.
    fn from_json(&Json) -> Result<T, DecoderError>;
}

///An abstraction above classic json map.
pub trait JsonMap {
    //helper functions for accessing data

    /// Read json value based on a key and perform an action on it (avoiding copying).
    fn with_key<F, R>(&self, key: &str, action: F) -> Result<R, DecoderError>
        where F: Fn(&Json) -> Result<R, DecoderError>;

    /// Read optional json value and perform action on it.
    /// Value if considered missing if it is not present in the map, or if it is null.
    fn with_optional_key<F, R>(&self, key: &str, action: F) -> Result<Option<R>, DecoderError>
        where F: Fn(&Json) -> Result<R, DecoderError>;

    /// Read an item based on a given key. Item must implement FromJson.
    fn read_item<T: FromJson<T>>(&self, key: &str) -> Result<T, DecoderError> {
        self.with_key(key, T::from_json)
    }

    //helper functions for modifying data

    /// Write an item that implements the ToJson trait.
    fn write_item<T: ToJson>(&mut self, key: &str, item: T);
}

// Implementations of FromJson for basic types

impl FromJson<i64> for i64 {
    fn from_json(json: &Json) -> Result<i64, DecoderError> {
        json.as_i64().ok_or_else(|| expected("i64", json) )
    }
}

impl FromJson<u64> for u64 {
    fn from_json(json: &Json) -> Result<u64, DecoderError> {
        json.as_u64().ok_or_else(|| expected("u64", json) )
    }
}

impl FromJson<f64> for f64 {
    fn from_json(json: &Json) -> Result<f64, DecoderError> {
        json.as_f64().ok_or_else(|| expected("f64", json) )
    }
}

impl FromJson<bool> for bool {
    fn from_json(json: &Json) -> Result<bool, DecoderError> {
        json.as_boolean().ok_or_else(|| expected("bool", json) )
    }
}

impl FromJson<String> for String {
    fn from_json(json: &Json) -> Result<String, DecoderError> {
        json.as_string().map(|s| s.to_string()).ok_or_else(|| expected("String", json))
    }
}

impl <T: FromJson<T>> FromJson<Vec<T>> for Vec<T> {
    fn from_json(json: &Json) -> Result<Vec<T>, DecoderError> {
        json.as_array().ok_or_else(|| expected("Array", json)).and_then(|v| {
            let mut output = vec!();
            let mut error = None;
            for item in v {
                match T::from_json(item) {
                    Ok(value) => output.push(value),
                    Err(e) => error = Some(e)
                }
                if error.is_some() { break }
            }
            if let Some(e) = error { Err(e) } else { Ok(output) }
        })
    }
}

impl <T: FromJson<T>> FromJson<Option<T>> for Option<T> {
    fn from_json(json: &Json) -> Result<Option<T>, DecoderError> {
        if json.is_null() { Ok(None) } else { T::from_json(json).map(Some) }
    }
}

// JsonMap implementation

impl JsonMap for Object {

    fn with_key<F, R>(&self, key: &str, action: F) -> Result<R, DecoderError>
        where F: Fn(&Json) -> Result<R, DecoderError> {
        self.get(key).ok_or_else(|| MissingFieldError(key.to_string())).and_then(action)
    }

    fn with_optional_key<F, R>(&self, key: &str, action: F) -> Result<Option<R>, DecoderError>
        where F: Fn(&Json) -> Result<R, DecoderError> {
        match self.get(key) {
            None => Ok(None),
            Some(&Json::Null) => Ok(None),
            Some(ref json) => action(json).map(Some)
        }
    }

    fn write_item<T: ToJson>(&mut self, key: &str, item: T) {
        self.insert(key.to_string(), item.to_json());
    }
}

// Some utility stuff

pub fn create_object<F>(builder: F) -> Json
    where F: Fn(&mut Object) {
    let mut result = Object::new();
    builder(&mut result);
    Json::Object(result)
}

fn as_object<F, R>(json: &Json, builder: F) -> Result<R, DecoderError>
    where F: Fn(&Object) -> Result<R, DecoderError> {
    if let &Json::Object(ref map) = json { builder(map) } else { Err(expected("Object", json)) }
}

// Some internal utility stuff

fn expected(expected: &str, got: &Json) -> DecoderError {
    ExpectedError(expected.to_string(), got.to_string())
}