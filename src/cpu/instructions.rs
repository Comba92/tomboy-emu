use super::{CPU, addressing::Operand, Flags};

impl CPU {
  pub fn ld(&mut self, dst: &Operand, src: &Operand) {
    let data = self.get_from_source(src);
    self.set_to_destination(dst, data);
  }

  pub fn ldi(&mut self, src: &Operand, dst: &Operand) {

  }

  pub fn ldd(&mut self, src: &Operand, dst: &Operand) {
    
  }

  pub fn push(&mut self, src: &Operand) {
    let data = self.get_from_source(src);
    self.stack_push_16(data);
  }

  pub fn pop(&mut self, dst: &Operand) {
    let data = self.stack_pop_16();
    self.set_to_destination(dst, data);
  }

  pub fn add(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;

    self.f.remove(Flags::SUB);
    self.update_zero_and_carries(self.a, data, 0);

    self.a = self.a.wrapping_add(data);
  }

  pub fn adc(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let carry = self.carry();

    self.f.remove(Flags::SUB);
    self.update_zero_and_carries(self.a, data, carry);

    self.a = self.a.wrapping_add(data).wrapping_add(carry);
  }

  pub fn sub(&mut self, src: &Operand) {
    let data = (self.get_from_source(src) as u8).wrapping_neg();

    self.f.insert(Flags::SUB);
    self.update_zero_and_carries(self.a, data, 0);

    self.a = self.a.wrapping_add(data);
  }

  pub fn sbc(&mut self, src: &Operand) {
    let data = (self.get_from_source(src) as u8).wrapping_neg();
    let carry = self.carry().wrapping_neg();

    self.f.insert(Flags::SUB);
    self.update_zero_and_carries(self.a, data, carry);

    self.a = self.a.wrapping_add(data).wrapping_add(carry);
  }

  pub fn and(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let result = self.a & data;

    self.update_zero(result);
    self.f.remove(Flags::SUB);
    self.set_hcarry_and_unset_carry();
  }

  pub fn xor(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let result = self.a ^ data;

    self.update_zero(result);
    self.f.remove(Flags::SUB);
    self.unset_hcarry_and_carry();
  }

  pub fn or(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let result = self.a | data;

    self.update_zero(result);
    self.f.remove(Flags::SUB);
    self.unset_hcarry_and_carry();
  }

  pub fn cp(&mut self, src: &Operand) {
    let data = (self.get_from_source(src) as u8).wrapping_neg();
    let result = self.a.wrapping_add(data);
    
    self.f.insert(Flags::SUB);
    self.update_zero_and_carries(self.a, data, 0);
  }

  pub fn inc(&mut self, src: &Operand) {
    let data = self.get_from_source(src);
    let result = data.wrapping_add(1);

    self.f.remove(Flags::SUB);
    self.update_zero(result as u8);
    self.update_hcarry(data as u8, 1, 0);

    self.set_to_destination(src, result);
  }

  pub fn dec(&mut self, src: &Operand) {
    let data = self.get_from_source(src);
    let result = data.wrapping_sub(1);

    self.f.insert(Flags::SUB);
    self.update_zero(result as u8);
    self.update_hcarry(data as u8, 1u8.wrapping_neg(), 0);

    self.set_to_destination(src, result);
  }

  pub fn add_16(&mut self, src: &Operand) {
    let data = self.get_from_source(src);
    let hl = self.get_hl();

    self.f.remove(Flags::SUB);
    self.update_hcarry_16(hl, data);
    self.update_carry_16(hl, data);

    self.set_hl(hl.wrapping_add(data));
  }

  pub fn rlca(&mut self) {
    let carry = self.a >> 7;
    let result = (self.a << 1) | carry;
    self.a = result;

    self.update_flags_after_rotation(carry);
  }
  
  pub fn rrca(&mut self) {
    let carry = self.a << 7;
    let result = (self.a >> 1) | carry;
    self.a = result;

    self.update_flags_after_rotation(carry);
  }

  pub fn rla(&mut self) {
    let carry = self.carry();
    let bit = self.a >> 7;
    let result = (self.a << 1) | carry;
    self.a = result;

    self.update_flags_after_rotation(bit); 
  }

  pub fn rra(&mut self) {
    let carry = self.carry();
    let bit = self.a & 1;
    let result = (self.a >> 1) | (carry << 7);
    self.a = result;

    self.update_flags_after_rotation(bit); 
  }

  pub fn ccf(&mut self) {
    self.f.remove(Flags::SUB);
    self.f.remove(Flags::HCARRY);
    self.f.toggle(Flags::CARRY);
  }

  pub fn scf(&mut self) {
    self.f.remove(Flags::SUB);
    self.f.remove(Flags::HCARRY);
    self.f.insert(Flags::CARRY);
  }

  pub fn jp(&mut self, dst: &Operand) {
    let addr = self.get_from_source(dst);
    self.pc = addr;
  }

  pub fn jpc(&mut self, cond: &Operand, dst: &Operand) {
    let cond = self.get_from_source(cond);
    if cond != 0 {
      self.jp(dst);
    }
  }

  pub fn jr(&mut self, dst: &Operand) {
    let addr = self.get_from_source(dst) as i8;
    self.pc = self.pc.wrapping_add_signed(addr as i16);
  }

  pub fn jrc(&mut self, cond: &Operand, dst: &Operand) {
    let cond = self.get_from_source(cond);
    if cond != 0 {
      self.jr(dst);
    }
  }

  pub fn call(&mut self, dst: &Operand) {
    todo!("LOL")
  }

  pub fn callc(&mut self, cond: &Operand, dst: &Operand) {
    let cond = self.get_from_source(cond);
    if cond != 0 {
      self.call(dst);
    }
  }

}