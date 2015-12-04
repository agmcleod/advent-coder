extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
  let input = String::from("iwrupvqb");
  let mut md5 = Md5::new();

  md5.input(input.as_bytes());
  let mut output = String::new();
  let mut bytes = output.as_bytes().clone();
  md5.result(bytes);
  let my_vec = bytes.iter().cloned().collect();
  println!("{:?}", String::from_utf8(my_vec).ok().expect("could not parse bytes"));
}
