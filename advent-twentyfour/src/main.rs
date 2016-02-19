use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn comb(arr: &Vec<usize>, n: usize, write_to: &mut Vec<Vec<usize>>) {
  let mut incl_arr = vec![false; arr.len()];
  comb_intern(arr, n, &mut incl_arr, 0, write_to);
}

fn comb_intern(arr: &Vec<usize>, n: usize, incl_arr: &mut Vec<bool>, index: usize, write_to: &mut Vec<Vec<usize>>) {
  if arr.len() < n + index {
      return;
  }
  if n == 0 {
    let it = arr.iter().zip(incl_arr.iter()).filter_map(|(val, incl)|
      if *incl {
        Some(val)
      } else {
        None
      }
    ).cloned().collect::<Vec<usize>>();

    write_to.push(it);
    return;
  }

  incl_arr[index] = true;
  comb_intern(arr, n-1, incl_arr, index+1, write_to);
  incl_arr[index] = false;

  comb_intern(arr, n, incl_arr, index+1, write_to);
}

fn mul(values: &Vec<usize>) -> usize {
    values.iter().fold(1, |total, value| total * value)
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("weights.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn sum(values: &Vec<usize>) -> usize {
    values.iter().fold(0, |sum, &v| sum + v)
}

fn calculate_weights(weights: &Vec<usize>, target: &usize) {
    for i in 0..weights.len() {
        let mut out = Vec::<Vec<usize>>::new();
        comb(&weights, i, &mut out);
        let mut results = Vec::<usize>::new();
        for c in out.iter() {
            if sum(c) == *target {
                results.push(mul(c));
            }
        }
        if results.len() > 0 {
            results.sort();
            println!("{:?}", results.get(0).unwrap());
            break
        }
    }
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e)
    };

    let mut weights: Vec<usize> = Vec::new();

    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        if *line == "" {
            continue
        }

        weights.push(line.parse().ok().expect("nope"));
    }

    let target = weights.iter().fold(0, |sum, &w| sum + w) / 3;
    let target = weights.iter().fold(0, |sum, &w| sum + w) / 4; // part 2

    calculate_weights(&weights, &target);
}
