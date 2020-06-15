use crate::conversions::{hex_to_u8, u8_to_string};

pub fn main() -> Result<(), std::num::ParseIntError> {
  let input = hex_to_u8("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;
  let result = crack_single_byte_xor(input);
  assert_eq!(result, "Cooking MC\'s like a pound of bacon");
  println!("1.3 cracking single byte xor: {}", result);
  Ok(())
}

pub fn crack_single_byte_xor(secret: Vec<u8>) -> String {
  let temp = single_character_xor(0, &secret);
  let mut best_result = u8_to_string(&temp);
  let mut best_key_score = score(&best_result);

  for i in 1..255 {
    let temp = single_character_xor(i, &secret);
    let result = u8_to_string(&temp);
    let score = score(&result);

    if score >= best_key_score {
      best_key_score = score;
      best_result = result;
    }
  }

  return best_result;
}

fn single_character_xor(key: u8, secret: &Vec<u8>) -> Vec<u8> {
  // can be converted to use repeating_key_xor
  let mut result = Vec::new();
  for i in 0..secret.len() {
    let res = key ^ secret[i];
    result.push(res);
  }

  return result;
}

pub fn score(str: &String) -> i32 {
  let mut count = 0;
  for it in str.chars() {

    if it == 'e' || it == 'E' || it == 't' || it == 'T' {
      // is e or t
      count = count + 1;
    } 

    if (it >= 'a' && it <= 'z') || (it >= 'A' && it <= 'Z') || it == ' ' {
      count = count + 1;
    } else {
      count = count - 2;
    }
  }

  return count;
}
