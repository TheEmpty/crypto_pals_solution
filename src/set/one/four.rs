use crate::set::one::three::{crack_single_byte_xor, score};
use crate::conversions::hex_to_u8;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() -> Result<(), std::num::ParseIntError> {
  let mut best_score = i32::min_value();
  let mut best_result = String::new();

  let file = File::open("crypto-pals-1.4.txt").expect("Failed to load file");
  let buffer = BufReader::new(file).lines();

  for line in buffer {
    let this_line = line.expect("no line found");
    let result = crack_single_byte_xor(hex_to_u8(&this_line).expect("failed to convert line from hex"));
    let score = score(&result);
    if score >= best_score {
      best_score = score;
      best_result = result;
    }
  }

  assert_eq!(best_result, "Now that the party is jumping\n");
  println!("1.4 scoring scores: {}", best_result.trim_end());

  Ok(())
}