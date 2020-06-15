mod conversions;
mod set;

fn main() -> Result<(), std::num::ParseIntError> {
  set::one::one::main()?;
  set::one::two::main()?;
  set::one::three::main()?;
  set::one::four::main()?;
  set::one::five::main()?;

  Ok(())
}
