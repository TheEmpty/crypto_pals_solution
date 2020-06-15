extern crate base64;

use crate::conversions::hex_to_u8;

fn hex_to_base64(hex: Vec<u8>) -> String {
  return base64::encode(hex);
}

pub fn main() -> Result<(), std::num::ParseIntError> {
  let hex = hex_to_u8("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")?;
  let result = hex_to_base64(hex);
  assert_eq!(result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
  println!("1.1 base64: {}", result);
  Ok(())
}
