extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

use std::collections::BTreeMap;

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("distances.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(err) => panic!("Could not read file {:?}", err)
    };


}
