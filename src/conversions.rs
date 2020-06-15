pub fn hex_to_u8(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
  let mut bytes = Vec::new();
  for i in 0..(hex.len()/2) {
    let byte = u8::from_str_radix(&hex[2*i .. 2*i + 2], 16)?;
    bytes.push(byte);
  }

  return Ok(bytes);
}

pub fn u8_to_string(data: &Vec<u8>) -> String {
  let mut buffer = String::with_capacity(data.len());

  for byte in data.iter() {
    buffer.push((*byte).into());
  }

  return buffer;
}

// adapted from https://docs.rs/crate/hex/0.3.1/source/src/lib.rs
pub fn u8_to_hex(data: Vec<u8>) -> String {
  let mut buffer = String::with_capacity(data.len() * 2);

  static CHARS: &'static [u8] = b"0123456789abcdef";
  for &byte in data.iter() {
    buffer.push(CHARS[(byte >> 4) as usize].into());
    buffer.push(CHARS[(byte & 0xf) as usize].into());
  }

  return buffer;
}
