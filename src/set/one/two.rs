use crate::conversions::{hex_to_u8, u8_to_hex};

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

pub fn main() -> Result<(), std::num::ParseIntError> {
  let first = hex_to_u8("1c0111001f010100061a024b53535009181c")?;
  let second = hex_to_u8("686974207468652062756c6c277320657965")?;
  let result = u8_to_hex(fixed_xor(first, second));
  assert_eq!(result, "746865206b696420646f6e277420706c6179");
  println!("1.2 fixed xor: {}", result);
  Ok(())
}