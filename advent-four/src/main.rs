extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
  
  let mut md5 = Md5::new();

  let mut n = 1;

  loop {
    md5.reset();
    let mut input = format!("iwrupvqb{:?}", n);
    md5.input_str(input.as_ref());  
    let result = md5.result_str();

    if result.starts_with("000000") {
      break
    }

    n += 1;
  }

  println!("{:?}", n);
}
