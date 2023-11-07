use super::{CPU, optable::OperandsType};

impl CPU {
  pub fn ld(&mut self, operands: &Vec<OperandsType>) {
    let value = self.get_operand(&operands[1]);
    self.set_operand(&operands[0], value);
  }
}