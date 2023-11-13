use super::{addressing::Opcode, CPU};

impl CPU {
  pub fn decode(&mut self, opcode: &Opcode) -> Result<(), &'static str> {
    match opcode.code {
      0x00 => Err("NOP"),
      _ => unimplemented!("Todo all instructions...")
    }
  }
}