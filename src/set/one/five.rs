use crate::conversions::u8_to_hex;

pub fn main() -> Result<(), std::num::ParseIntError> {
  let key = "ICE".as_bytes().to_vec();
  let secret = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes().to_vec();
  let encrypted = u8_to_hex(repeating_key_xor(&key, secret));
  assert_eq!(encrypted, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
  println!("1.5 repeating key XOR: {}", encrypted);

  Ok(())
}

fn repeating_key_xor(key: &Vec<u8>, secret: Vec<u8>) -> Vec<u8> {
  let mut result = Vec::with_capacity(secret.len());

  for i in 0..secret.len() {
    let res = key[i % key.len()] ^ secret[i];
    result.push(res);
  }

  return result;
}
