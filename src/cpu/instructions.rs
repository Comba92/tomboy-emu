use super::{CPU, registers::{Register8, Register16}};

impl CPU {
  fn get_alu_op_operands(&self, reg: u8) -> (u8, u8) {
    let a = self.registers.get(Register8::A as u8);
    
    let other = if reg == Register8::F as u8 {
      let addr = self.registers.get_16(Register16::HL as u8);
      self.mem_read(addr)
    } else { self.registers.get(reg) };

    (a, other)
  }

  pub(super) fn load_r_to_r(&mut self, dst: u8, src: u8) {
    let data = self.registers.get(src);
    self.registers.set(dst, data);
  }

  pub(super) fn load_hl_indirect_to_r(&mut self, dst: u8) {
    let addr = self.registers.get_hl();
    let data = self.mem_read(addr);
    self.registers.set(dst, data);
  }

  pub(super) fn load_r_to_hl_indirect(&mut self, src: u8) {
    let addr = self.registers.get_hl();
    let data = self.registers.get(src);
    self.mem_write(addr, data)
  }

  pub(super) fn load_sp_to_mem(&mut self) {
    let addr = self.mem_read_16(self.pc);
    self.mem_write_16(addr, self.sp);
  }

  pub(super) fn load_immediate_to_r(&mut self, reg: u8) {
    let data = self.mem_read(self.pc);
    self.registers.set(reg, data);
  }

  pub(super) fn load_immediate_16_to_rr(&mut self, reg: u8) {
    let data = self.mem_read_16(self.pc);
    if reg == 3 { self.sp = data; }
    else { self.registers.set_16(reg, data); }
  }

  pub(super) fn load_rr_indirect_to_a(&mut self, reg: u8) {
    let addr = self.registers.get_16(reg);
    let data = self.mem_read(addr);
    self.registers.set(Register8::A as u8, data);

    // if it is hl
    if reg == 2 { self.registers.set_16(Register16::HL as u8, addr.wrapping_add(1)); }
    if reg == 3 { self.registers.set_16(Register16::HL as u8, addr.wrapping_sub(1)); }
  }

  pub(super) fn load_a_to_indirect_rr(&mut self, reg: u8) {
    let data = self.registers.get(Register8::A as u8);
    let addr = self.registers.get_16(reg);
    self.mem_write(addr, data);

    // if it is hl
    if reg == 2 { self.registers.set_16(Register16::HL as u8, addr.wrapping_add(1)); }
    if reg == 3 { self.registers.set_16(Register16::HL as u8, addr.wrapping_sub(1)); }
  }

  pub(super) fn inc_r(&mut self, reg: u8) {
    let value = self.registers.get(reg);
    self.registers.set(reg, value.wrapping_add(1));

    self.registers.set_zero(value == 0);
    self.registers.set_sub(false);
    self.registers.update_hcarry(value, 1);
  }

  pub(super) fn dec_r(&mut self, reg: u8) {
    let value = self.registers.get(reg);
    self.registers.set(reg, value.wrapping_sub(1));

    self.registers.set_zero(value == 0);
    self.registers.set_sub(true);
    self.registers.update_hcarry(value, 1u8.wrapping_neg());
  }

  pub(super) fn inc_rr(&mut self, reg: u8) {
    let value = self.registers.get_16(reg);
    self.registers.set_16(reg, value.wrapping_add(1));
  }

  pub(super) fn dec_rr(&mut self, reg: u8) {
    let value = self.registers.get_16(reg);
    self.registers.set_16(reg, value.wrapping_sub(1));
  }

  pub(super) fn add(&mut self, reg: u8) {
    let (a, other) = self.get_alu_op_operands(reg);

    let result = a.wrapping_add(other);
    self.registers.set(Register8::A as u8, result);

    self.registers.set_sub(false);
    self.registers.update_zero_and_carries_flags(a, other);
  }

  pub(super) fn adc(&mut self, reg: u8) {
    let (a, other) = self.get_alu_op_operands(reg);

    let carry = self.registers.carry() as u8;
    let result = a.wrapping_add(other).wrapping_add(carry);
    self.registers.set(Register8::A as u8, result);

    self.registers.set_sub(false);
    self.registers.update_zero_and_carries_flags_3(a, other, carry);
  } 

  pub(super) fn sub(&mut self, reg: u8) {
    let (a, other) = self.get_alu_op_operands(reg);

    let other = other.wrapping_neg();
    let result = a.wrapping_add(other);
    self.registers.set(Register8::A as u8, result);

    self.registers.set_sub(false);
    self.registers.update_zero_and_carries_flags(a, other);
  } 

  pub(super) fn sbc(&mut self, reg: u8) {
    let (a, other) = self.get_alu_op_operands(reg);

    let carry = (self.registers.carry() as u8).wrapping_neg();
    let other = other.wrapping_neg();
    let result = a.wrapping_add(other).wrapping_add(carry);
    self.registers.set(Register8::A as u8, result);

    self.registers.set_sub(false);
    self.registers.update_zero_and_carries_flags_3(a, other, carry);
  } 

  pub(super) fn and(&mut self, reg: u8) {
    let (a, other) = self.get_alu_op_operands(reg);

    let result = a & other;
    self.registers.set(Register8::A as u8, result);

    self.registers.set_zero(result == 0);
    self.registers.set_sub(false);
    self.registers.set_carry(true);
    self.registers.set_hcarry(false);
  } 

  pub(super) fn xor(&mut self, reg: u8) {
    let (a, other) = self.get_alu_op_operands(reg);

    let result = a ^ other;
    self.registers.set(Register8::A as u8, result);

    self.registers.set_zero(result == 0);
    self.registers.set_sub(false);
    self.registers.set_carry(false);
    self.registers.set_hcarry(false);
  } 

  pub(super) fn or(&mut self, reg: u8) {
    let (a, other) = self.get_alu_op_operands(reg);

    let result = a | other;
    self.registers.set(Register8::A as u8, result);

    self.registers.set_zero(result == 0);
    self.registers.set_sub(false);
    self.registers.set_carry(false);
    self.registers.set_hcarry(false);
  } 

  pub(super) fn cp(&mut self, reg: u8) {
    let (a, other) = self.get_alu_op_operands(reg);
    let other = other.wrapping_neg();

    self.registers.set_sub(true);
    self.registers.update_zero_and_carries_flags(a, other);
  }

  pub(super) fn add_rr_to_hl(&mut self, reg: u8) {
    let data = if reg == 3 {
      self.sp
    } else { self.registers.get_16(reg) };

    let hl = self.registers.get_16(Register16::HL as u8);

    let result = hl.wrapping_add(data);
    self.registers.set_16(Register16::HL as u8, result);

    self.registers.set_sub(false);
    self.registers.update_carry_16(hl, data);
    self.registers.update_hcarry_16(hl, data);
  }
}