extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::Object;
use rustc_serialize::json::Array;
use std::fs::File;
use std::io::Read;

fn parse_json_object(object: &Object, total: &mut i32) {
    let values = object.values().cloned().collect::<Vec<Json>>();
    let mut has_red = false;

    for v in values.iter() {
        match *v {
            Json::String(ref s) => {
                if s == "red" {
                    has_red = true
                }
            },
            _ => {}
        }
    }

    if !has_red {
        for (key, value) in object.iter() {
            process_value(value, total)
        }
    }
}

fn parse_json_array(object: &Array, total: &mut i32) {
    for value in object.iter() {
        process_value(value, total)
    }
}

fn process_value(value: &Json, total: &mut i32) {
    match *value {
        Json::U64(v) => *total += v as i32,
        Json::I64(v) => *total += v as i32,
        Json::Array(ref v) => parse_json_array(&value.as_array().unwrap(), total),
        Json::Object(ref v) => parse_json_object(&value.as_object().unwrap(), total),
        _ => {}
    }
}

fn main() {
    let mut file = File::open("input.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut total = 0i32;
    let json = Json::from_str(&data).unwrap();
    process_value(&json, &mut total);
    println!("{}", total);
}
