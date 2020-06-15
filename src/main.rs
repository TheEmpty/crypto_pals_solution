extern crate base64;

use std::fs::File;
use std::io::{BufReader, BufRead};

fn hex_to_u8(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
  let mut bytes = Vec::new();
  for i in 0..(hex.len()/2) {
    let byte = u8::from_str_radix(&hex[2*i .. 2*i + 2], 16)?;
    bytes.push(byte);
  }

  return Ok(bytes);
}

fn u8_to_string(data: &Vec<u8>) -> String {
  let mut buffer = String::with_capacity(data.len());

  for byte in data.iter() {
    buffer.push((*byte).into());
  }

  return buffer;
}

// adapted from https://docs.rs/crate/hex/0.3.1/source/src/lib.rs
fn u8_to_hex(data: Vec<u8>) -> String {
  let mut buffer = String::with_capacity(data.len() * 2);

  static CHARS: &'static [u8] = b"0123456789abcdef";
  for &byte in data.iter() {
    buffer.push(CHARS[(byte >> 4) as usize].into());
    buffer.push(CHARS[(byte & 0xf) as usize].into());
  }

  return buffer;
}

fn main() -> Result<(), std::num::ParseIntError> {
  // 1.1
  let hex = hex_to_u8("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")?;
  let result11 = hex_to_base64(hex);
  assert_eq!(result11, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
  println!("1.1 base64: {}", result11);

  // 1.2
  let first = hex_to_u8("1c0111001f010100061a024b53535009181c")?;
  let second = hex_to_u8("686974207468652062756c6c277320657965")?;
  let result12 = u8_to_hex(fixed_xor(first, second));
  assert_eq!(result12, "746865206b696420646f6e277420706c6179");
  println!("1.2 fixed xor: {}", result12);

  // 1.3
  let input = hex_to_u8("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;
  let result13 = crack_single_byte_xor(input);
  assert_eq!(result13, "Cooking MC\'s like a pound of bacon");
  println!("1.3 cracking single byte xor: {}", result13);

  // 1.4
  let result14 = one_point_four();
  assert_eq!(result14, "Now that the party is jumping\n");
  println!("1.4 scoring scores: {}", result14.trim_end());

  // 1.5
  let key = "ICE".as_bytes().to_vec();
  let secret15 = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes().to_vec();
  let encrypted15 = u8_to_hex(repeating_key_xor(&key, secret15));
  assert_eq!(encrypted15, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
  println!("1.5 repeating key XOR: {}", encrypted15);

  Ok(())
}

// 1.1
fn hex_to_base64(hex: Vec<u8>) -> String {
  return base64::encode(hex);
}

// 1.2
fn fixed_xor(first: Vec<u8>, second: Vec<u8>) -> Vec<u8> {
  // can be converted to use repeating_key_xor
  assert_eq!(first.len(), second.len());

  let mut result = Vec::new();
  for i in 0..first.len() {
    let res = first[i] ^ second[i];
    result.push(res);
  }

  return result;
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

fn score(str: &String) -> i32 {
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

fn crack_single_byte_xor(secret: Vec<u8>) -> String {
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

fn one_point_four() -> String {
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

  return best_result;
}

fn repeating_key_xor(key: &Vec<u8>, secret: Vec<u8>) -> Vec<u8> {
  let mut result = Vec::with_capacity(secret.len());

  for i in 0..secret.len() {
    let res = key[i % key.len()] ^ secret[i];
    result.push(res);
  }

  return result;
}
