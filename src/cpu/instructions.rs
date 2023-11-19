use super::{CPU, addressing::{Operand, OperandType, RegisterOperand}, Flags};

const REG_A_OPERAND: Operand = Operand { 
  kind: OperandType::Register(RegisterOperand::A), 
  immediate: true,
};

// Flags Management
impl CPU {
  pub fn update_zero(&mut self, result: u8) { self.f.set(Flags::ZERO, result == 0); }

  pub fn update_carry(&mut self, a: u8, b: u8, c: u8) {
    let result = (a as u16).wrapping_add(b as u16).wrapping_add(c as u16);
    self.f.set(Flags::CARRY, result > 0xff);
  }

  pub fn update_carry_sub(&mut self, a: u8, b: u8, c: u8) {
    let result = (a as u16).wrapping_sub(b as u16).wrapping_sub(c as u16);
    self.f.set(Flags::CARRY, result > 0xff);
  }

  pub fn update_hcarry(&mut self, a: u8, b: u8, c: u8) {
    let result = (a & 0xf).wrapping_add(b & 0xf).wrapping_add(c & 0xf);
    self.f.set(Flags::HCARRY, result > 0xf);
  }

  pub fn update_hcarry_sub(&mut self, a: u8, b: u8, c: u8) {
    let result = (a & 0xf).wrapping_sub(b & 0xf).wrapping_sub(c & 0xf);
    self.f.set(Flags::HCARRY, result > 0xf);
  }

  pub fn update_carry_16(&mut self, a: u16, b: u16) {
    let result = (a as u32).wrapping_add(b as u32);
    self.f.set(Flags::CARRY, result > 0xffff);
  }

  pub fn update_hcarry_16(&mut self, a: u16, b: u16) {
    let result = (a & 0x0fff).wrapping_add(b & 0x0fff);
    self.f.set(Flags::HCARRY, result > 0x0fff);
  }

  pub fn update_zero_and_carries(&mut self, a: u8, b: u8, c: u8) {
    self.update_zero(a.wrapping_add(b).wrapping_add(c));
    self.update_carry(a, b, c);
    self.update_hcarry(a, b, c);
  }

  pub fn update_zero_and_carries_sub(&mut self, a: u8, b: u8, c: u8) {
    self.update_zero(a.wrapping_sub(b).wrapping_sub(c));
    self.update_carry_sub(a, b, c);
    self.update_hcarry_sub(a, b, c);
  }

  pub fn set_hcarry_and_unset_carry(&mut self) {
    self.f.insert(Flags::HCARRY);
    self.f.remove(Flags::CARRY);
  }
  pub fn unset_hcarry_and_carry(&mut self) {
    self.f.remove(Flags::HCARRY);
    self.f.remove(Flags::CARRY);
  }
  pub fn update_flags_after_rotation(&mut self, result: u8, bit: u8) {
    self.update_zero(result);
    self.f.remove(Flags::SUB);
    self.f.remove(Flags::HCARRY);
    self.f.set(Flags::CARRY, bit != 0);
  }
}


// Instructions
impl CPU {
  pub fn ld(&mut self, dst: &Operand, src: &Operand) {
    let data = self.get_from_source(src);

    if src.is_value_16() {
      self.set_to_destination_16(dst, data);
    } else {
      self.set_to_destination(dst, data as u8);
    }
  }

  pub fn ld_io_in_c_reg_to_a(&mut self) {
    let addr = 0xFF00 + self.c as u16;
    self.a = self.mem_read(addr);
  }

  pub fn ld_a_to_io_in_c_reg(&mut self) {
    let addr = 0xFF00 + self.c as u16;
    self.mem_write(addr, self.a);
  }

  pub fn ldi(&mut self, dst: &Operand, src: &Operand) {
    self.ld(dst, src);
    let hl = self.get_hl();
    self.set_hl(hl.wrapping_add(1));
  }

  pub fn ldd(&mut self, dst: &Operand, src: &Operand) {
    self.ld(dst, src);
    let hl = self.get_hl();
    self.set_hl(hl.wrapping_sub(1));
  }

  pub fn push(&mut self, src: &Operand) {
    let data = self.get_from_source(src);
    self.stack_push(data);
  }

  pub fn pop(&mut self, dst: &Operand) {
    let data = self.stack_pop();
    self.set_to_destination_16(dst, data);
  }

  pub fn add(&mut self, dst: &Operand) {
    let data = self.get_from_source(dst) as u8;

    self.f.remove(Flags::SUB);
    self.update_zero_and_carries(self.a, data, 0);

    self.a = self.a.wrapping_add(data);
  }

  pub fn adc(&mut self, dst: &Operand) {
    let data = self.get_from_source(dst) as u8;
    let carry = self.carry();

    self.f.remove(Flags::SUB);
    self.update_zero_and_carries(self.a, data, carry);

    self.a = self.a.wrapping_add(data).wrapping_add(carry);
  }

  pub fn sub(&mut self, dst: &Operand) {
    let data = self.get_from_source(dst) as u8;

    self.f.insert(Flags::SUB);
    self.update_zero_and_carries_sub(self.a, data, 0);

    self.a = self.a.wrapping_sub(data);
  }

  pub fn sbc(&mut self, dst: &Operand) {
    let data = self.get_from_source(dst) as u8;
    let carry = self.carry();

    self.f.insert(Flags::SUB);
    self.update_zero_and_carries_sub(self.a, data, carry);

    self.a = self.a.wrapping_sub(data).wrapping_sub(carry);
  }

  pub fn and(&mut self, dst: &Operand) {
    let data = self.get_from_source(dst) as u8;
    let result = self.a & data;

    self.update_zero(result);
    self.f.remove(Flags::SUB);
    self.set_hcarry_and_unset_carry();

    self.a = result;
  }

  pub fn xor(&mut self, dst: &Operand) {
    let data = self.get_from_source(dst) as u8;
    let result = self.a ^ data;

    self.update_zero(result);
    self.f.remove(Flags::SUB);
    self.unset_hcarry_and_carry();

    self.a = result;
  }

  pub fn or(&mut self, dst: &Operand) {
    let data = self.get_from_source(dst) as u8;
    let result = self.a | data;

    self.update_zero(result);
    self.f.remove(Flags::SUB);
    self.unset_hcarry_and_carry();

    self.a = result;
  }

  pub fn cp(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    
    self.f.insert(Flags::SUB);
    self.update_zero_and_carries_sub(self.a, data, 0);
  }

  pub fn inc(&mut self, dst: &Operand) {
    let data = self.get_from_source(dst);
    let result = data.wrapping_add(1);

    if dst.is_value_16() {
      self.set_to_destination_16(dst, result);
    } else {
      self.f.remove(Flags::SUB);
      self.update_zero(result as u8);
      self.update_hcarry(data as u8, 1, 0);

      self.set_to_destination(dst, result as u8);
    }
  }

  pub fn dec(&mut self, dst: &Operand) {
    let data = self.get_from_source(dst);
    let result = data.wrapping_sub(1);

    if dst.is_value_16() {
      self.set_to_destination_16(dst, result);
    } else {
      self.f.insert(Flags::SUB);
      self.update_zero(result as u8);
      self.update_hcarry_sub(data as u8, 1, 0);

      self.set_to_destination(dst, result as u8);
    }
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

  pub fn add_sp_sign(&mut self, offset: &Operand) {
    let data = self.get_from_source(offset);
    let signed = (self.get_from_source(offset) as i8) as i16;
    let subtraction = signed < 0;
    let result = self.sp.wrapping_add_signed(signed);

    self.f.remove(Flags::ZERO);
    self.f.remove(Flags::SUB);

    // Not documented anywhere!! This add gets treated as an 8bit operation
    // So both the carry flag and half carry get set as if it was an 8 bit operation!
    // https://stackoverflow.com/questions/57958631/game-boy-half-carry-flag-and-16-bit-instructions-especially-opcode-0xe8
    // https://stackoverflow.com/questions/5159603/gbz80-how-does-ld-hl-spe-affect-h-and-c-flags
    if subtraction {
      self.f.set(Flags::HCARRY, result & 0xf <= self.sp & 0xf);
      self.f.set(Flags::CARRY, result & 0xff <= self.sp & 0xff);
    } else {
      self.update_hcarry(self.sp as u8, data as u8, 0);
      self.update_carry(self.sp as u8, data as u8, 0);
    }

    self.sp = result;
  }


  pub fn ld_sp_sign(&mut self, offset: &Operand) {
    let data = self.get_from_source(offset);
    let signed = (self.get_from_source(offset) as i8) as i16;
    let subtraction = signed < 0;
    let result = self.sp.wrapping_add_signed(signed);

    self.f.remove(Flags::ZERO);
    self.f.remove(Flags::SUB);
    
    // Same as add_sp_sign()
    if subtraction {
      self.f.set(Flags::CARRY, result & 0xff <= self.sp & 0xff);
      self.f.set(Flags::HCARRY, result & 0xf <= self.sp & 0xf);
    } else {
      self.update_hcarry(self.sp as u8, data as u8, 0);
      self.update_carry(self.sp as u8, data as u8, 0);
    }

    self.set_hl(result);
  }

  pub fn daa(&mut self) {
    let a = self.a;

    let mut correction: u8 = 0;
    let mut carry = false;

    if self.f.contains(Flags::HCARRY) || 
      (!self.f.contains(Flags::SUB) && (a & 0xf) > 0x9) {
        correction += 0x6;
    }

    if self.f.contains(Flags::CARRY) || 
      (!self.f.contains(Flags::SUB) && a > 0x99) {
        correction += 0x60;
        carry = true;
    }
    
    let correction =
      if self.f.contains(Flags::SUB) { correction.wrapping_neg() } 
      else { correction };
    let result = self.a.wrapping_add(correction);

    self.update_zero(result);
    self.f.remove(Flags::HCARRY);
    self.f.set(Flags::CARRY, carry);

    self.a = result;
  }

  pub fn rlc(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let carry = data >> 7;
    let result = (data << 1) | carry;
    self.set_to_destination(src, result);

    self.update_flags_after_rotation(result, carry);
  }

  pub fn rlca(&mut self) {
    self.rlc(&REG_A_OPERAND);
    self.f.remove(Flags::ZERO);
  }
  
  pub fn rrc(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let carry = data & 1;
    let result = (data >> 1) | (carry << 7);
    self.set_to_destination(src, result);

    self.update_flags_after_rotation(result, carry);
  }

  pub fn rrca(&mut self) {
    self.rrc(&REG_A_OPERAND);
    self.f.remove(Flags::ZERO);
  }

  pub fn rl(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let carry = self.carry();
    let bit = data >> 7;
    let result = (data << 1) | carry;
    self.set_to_destination(src, result);

    self.update_flags_after_rotation(result, bit); 
  }

  pub fn rla(&mut self) {
    self.rl(&REG_A_OPERAND);
    self.f.remove(Flags::ZERO);
  }

  pub fn rr(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let carry = self.carry();
    let bit = data & 1;
    let result = (data >> 1) | (carry << 7);
    self.set_to_destination(src, result);

    self.update_flags_after_rotation(result, bit); 
  }

  pub fn rra(&mut self) {
    self.rr(&REG_A_OPERAND);
    self.f.remove(Flags::ZERO);
  }

  pub fn sla(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let bit = data >> 7;
    let result = data << 1;
    self.set_to_destination(src, result);

    self.update_flags_after_rotation(result, bit); 
  }

  pub fn sra(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let bit = data & 1;
    let last = data & 0b1000_0000;
    let result = data >> 1 | last;
    self.set_to_destination(src, result);

    self.update_flags_after_rotation(result, bit); 
  }

  pub fn srl(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8; 
    let bit = data & 1;
    let result = data >> 1;
    self.set_to_destination(src, result);

    self.update_flags_after_rotation(result, bit); 
  }

  pub fn swap(&mut self, src: &Operand) {
    let data = self.get_from_source(src) as u8;
    let low = data & 0x0f;
    let high = data >> 4;
    let result = (low << 4) | high;
    self.set_to_destination(src, result);

    self.update_flags_after_rotation(result, 0);
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
    self.pc = self.pc
      // The program counter points to the next instruction before the current instruction is evaluated. 
      .wrapping_add(1)
      .wrapping_add_signed(addr as i16);
  }

  pub fn jrc(&mut self, cond: &Operand, dst: &Operand) {
    let cond = self.get_from_source(cond);
    if cond != 0 {
      self.jr(dst);
    }
  }

  pub fn call(&mut self, dst: &Operand) {
    // The program counter points to the next instruction before the current instruction is evaluated. 
    self.stack_push(self.pc.wrapping_add(2));
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

  pub fn rst(&mut self, dst: &Operand) {
    let addr = self.get_from_source(dst);
    // The program counter points to the next instruction before the current instruction is evaluated. 
    self.stack_push(self.pc);
    self.pc = addr;
  }

  pub fn reti(&mut self) {
    self.ime = true;
    self.ret();
  }

   // The effect of ei is delayed by one instruction. This means that ei followed immediately by di does not allow any interrupts between them. This interacts with the halt bug in an interesting way.
   pub fn di(&mut self) { self.ime = false; }
   pub fn ei(&mut self) { self.ime_to_set = true; }


  pub fn bit(&mut self, bit: &Operand, src: &Operand) {
    let pos = self.get_from_source(bit) as u8;
    let data = self.get_from_source(src) as u8;

    let result = data & (1 << pos);
    self.update_zero(result);
    self.f.remove(Flags::SUB);
    self.f.insert(Flags::HCARRY);
  }

  pub fn set(&mut self, bit: &Operand, src: &Operand) {
    let pos = self.get_from_source(bit) as u8;
    let data = self.get_from_source(src) as u8;

    let result = data | (1 << pos);
    self.set_to_destination(src, result);
  }

  pub fn res(&mut self, bit: &Operand, src: &Operand) {
    let pos = self.get_from_source(bit) as u8;
    let data = self.get_from_source(src) as u8;

    let result = data & !(1 << pos);
    self.set_to_destination(src, result);
  }
}