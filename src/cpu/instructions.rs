use super::{CPU, operands::OperandsType};

impl CPU {
  pub fn ld(&mut self, operands: &Vec<OperandsType>) {
    let value_to_load = self.get_operand(&operands[1]);
    println!("[LD]: {:x}", value_to_load);
    self.set_operand(&operands[0], value_to_load);
  }
}