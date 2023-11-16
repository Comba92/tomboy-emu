use super::{CPU, addressing::Operand, Flags};

impl CPU {
  pub fn ld(&mut self, dst: &Operand, src: &Operand) {
    let data = self.get_from_source(src);
    self.set_to_destination(dst, data);
  }

  pub fn ld_sp_rel(&mut self, offset: &Operand) {
    let data = (self.get_from_source(offset) as i8) as i16;
    let result = self.sp.wrapping_add_signed(data);

    self.f.remove(Flags::ZERO);
    self.f.remove(Flags::SUB);
    self.update_hcarry_16(self.sp, data as u16);
    self.update_carry_16(self.sp, data as u16);

    self.set_hl(result);
  }

  pub fn ldi(&mut self, dst: &Operand, src: &Operand) {
    let data = self.get_from_source(src).wrapping_add(1);
    self.set_to_destination(dst, data);
  }

  pub fn ldd(&mut self, dst: &Operand, src: &Operand) {
    let data = self.get_from_source(src).wrapping_sub(1);
    self.set_to_destination(dst, data);
  }

  pub fn push(&mut self, src: &Operand) {
    let data = self.get_from_source(src);
    self.stack_push(data);
  }

  pub fn pop(&mut self, dst: &Operand) {
    let data = self.stack_pop();
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
    
    self.f.insert(Flags::SUB);
    self.update_zero_and_carries(self.a, data, 0);
  }

  pub fn inc(&mut self, src: &Operand) {
    let data = self.get_from_source(src);
    let result = data.wrapping_add(1);

    if !src.kind.is_register_16() {
      self.f.remove(Flags::SUB);
      self.update_zero(result as u8);
      self.update_hcarry(data as u8, 1, 0);
    }

    self.set_to_destination(src, result);
  }

  pub fn dec(&mut self, src: &Operand) {
    let data = self.get_from_source(src);
    let result = data.wrapping_sub(1);

    if !src.kind.is_register_16() {
      self.f.insert(Flags::SUB);
      self.update_zero(result as u8);
      self.update_hcarry(data as u8, 1u8.wrapping_neg(), 0);
    }

    self.set_to_destination(src, result);
  }

  pub fn add_16(&mut self, src: &Operand) {
    let data = self.get_from_source(src);
    let hl = self.get_hl();
    let result = hl.wrapping_add(data);
    self.set_hl(result);

    self.f.remove(Flags::SUB);
    self.update_hcarry_16(hl, data);
    self.update_carry_16(hl, data);
  }

  pub fn add_sp_rel(&mut self, offset: &Operand) {
    let data = (self.get_from_source(offset) as i8) as i16;
    let result = self.sp.wrapping_add_signed(data);

    self.f.remove(Flags::ZERO);
    self.f.remove(Flags::SUB);
    self.update_hcarry_16(self.sp, data as u16);
    self.update_carry_16(self.sp, data as u16);

    self.sp = result;
  }

  pub fn daa(&mut self) {
    let a = self.a;

    let mut correction: u8 = 0;
    let mut carry = false;

    if self.f.contains(Flags::HCARRY) || 
      (!self.f.contains(Flags::SUB) && a & 0xf > 0x9) {
        correction |= 0x6;
    }

    if self.f.contains(Flags::CARRY) || 
      (!self.f.contains(Flags::SUB) && a > 0x99) {
        correction |= 0x60;
        carry = true;
    }
    
    let result = 
      if self.f.contains(Flags::SUB) { correction.wrapping_neg() } 
      else { correction };
    
    self.update_zero(result);
    self.f.remove(Flags::HCARRY);
    self.f.set(Flags::CARRY, carry);

    self.a = result;
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

  pub fn cpl(&mut self) {
    self.a = !self.a;
    self.f.insert(Flags::SUB);
    self.f.insert(Flags::HCARRY);
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
    self.stack_push(self.pc);
    let addr = self.get_from_source(dst);
    self.pc = addr;
  }

  pub fn callc(&mut self, cond: &Operand, dst: &Operand) {
    let cond = self.get_from_source(cond);
    if cond != 0 {
      self.call(dst);
    }
  }

  pub fn ret(&mut self) {
    self.pc = self.stack_pop();
  }

  pub fn retc(&mut self, cond: &Operand) {
    let cond = self.get_from_source(cond);
    if cond != 0 {
      self.ret();
    }
  }

  pub fn reti(&mut self) {
    self.ret();
    self.ime = true;
  }

  pub fn rst(&mut self, dst: &Operand) {
    let addr = self.get_from_source(dst);
    let data = self.mem_read(addr);
    self.stack_push(self.pc);
    self.pc = data as u16;
  }

   // The effect of ei is delayed by one instruction. This means that ei followed immediately by di does not allow any interrupts between them. This interacts with the halt bug in an interesting way.

   pub fn di(&mut self) { self.ime = false; }
   pub fn ei(&mut self) { self.ime_to_set = true; }
}