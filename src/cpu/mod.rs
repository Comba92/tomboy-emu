#![allow(dead_code)]
use crate::definitions::{PC_INIT, SP_INIT};
use bitflags::bitflags;
use optable::OPTABLE;

mod addressing;
mod instructions;
mod decode;
pub mod optable;

bitflags! {
  pub struct Flags: u8 {
    const ZERO   = 1 << 7;
    const SUB    = 1 << 6;
    const HCARRY = 1 << 5;
    const CARRY  = 1 << 4;
  }
}

impl Flags {
  pub fn new(value: u8) -> Self { Flags::from_bits_truncate(value) }
}

pub struct CPU {
  pub a: u8,
  pub f: Flags,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,

  pub sp: u16,
  pub pc: u16,
  pub ram: [u8; 1024],
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      a: 0,
      f: Flags::new(0),
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      h: 0,
      l: 0,
      sp: PC_INIT,
      pc: SP_INIT,
      ram: [0; 1024],
    }
  }

  pub fn get_af(&self) -> u16 { u16::from_be_bytes([self.a, self.f.bits()]) }
  pub fn get_bc(&self) -> u16 { u16::from_be_bytes([self.b, self.c]) }
  pub fn get_de(&self) -> u16 { u16::from_be_bytes([self.d, self.e]) }
  pub fn get_hl(&self) -> u16 { u16::from_be_bytes([self.h, self.l]) }
  pub fn carry(&self) -> u8 { self.f.contains(Flags::CARRY) as u8 }
  pub fn hcarry(&self) -> u8 { self.f.contains(Flags::HCARRY) as u8 } 

  pub fn set_af(&mut self, data: u16) { let [high, low] = data.to_be_bytes(); self.a = high; self.f = Flags::new(low); }
  pub fn set_bc(&mut self, data: u16) { let [high, low] = data.to_be_bytes(); self.b = high; self.c = low; }
  pub fn set_de(&mut self, data: u16) { let [high, low] = data.to_be_bytes(); self.d = high; self.e = low; }
  pub fn set_hl(&mut self, data: u16) { let [high, low] = data.to_be_bytes(); self.h = high; self.l = low; }

  pub fn update_zero(&mut self, result: u8) { self.f.set(Flags::ZERO, result == 0); }

  pub fn update_carry(&mut self, a: u8, b: u8, c: u8) {
    let result = a as u16 + b as u16 + c as u16;
    self.f.set(Flags::CARRY, result > 0xff);
  }

  pub fn update_carry_16(&mut self, a: u16, b: u16) {
    let result = a as u32 + b as u32;
    self.f.set(Flags::CARRY, result > 0xffff);
  }

  pub fn update_hcarry(&mut self, a: u8, b: u8, c: u8) {
    let result = (a & 0x0f) + (b & 0x0f) + (c & 0x0f);
    self.f.set(Flags::HCARRY, result > 0x0f);
  }

  pub fn update_hcarry_16(&mut self, a: u16, b: u16) {
    let result = (a & 0x0fff) + (b & 0x0fff);
    self.f.set(Flags::HCARRY, result > 0x0fff);
  }

  pub fn update_zero_and_carries(&mut self, a: u8, b: u8, c: u8) {
    self.update_zero(a.wrapping_add(b).wrapping_add(c));
    self.update_carry(a, b, c);
    self.update_hcarry(a, b, c);
  }

  pub fn set_hcarry_and_unset_carry(&mut self) {
    self.f.insert(Flags::HCARRY);
    self.f.remove(Flags::CARRY);
  }
  pub fn unset_hcarry_and_carry(&mut self) {
    self.f.remove(Flags::HCARRY);
    self.f.remove(Flags::CARRY);
  }
  pub fn update_flags_after_rotation(&mut self, bit: u8) {
    self.f.remove(Flags::ZERO);
    self.f.remove(Flags::SUB);
    self.f.remove(Flags::HCARRY);
    self.f.set(Flags::CARRY, bit != 0);
  }

  pub fn mem_read(&self, addr: u16) -> u8 {
    self.ram[addr as usize]
  }
  pub fn mem_write(&mut self, addr: u16, data: u8) {
    self.ram[addr as usize] = data;
  }
  
  pub fn mem_read_16(&self, addr: u16) -> u16 {
    let low = self.mem_read(addr);
    let high = self.mem_read(addr + 1);

    u16::from_le_bytes([low, high])
  }

  pub fn mem_write_16(&mut self, addr: u16, data: u16) {
    let [low, high] = data.to_le_bytes();
    self.mem_write(addr, low);
    self.mem_write(addr + 1, high);
  }

  pub fn stack_push(&mut self, data: u16) {
    todo!("")
  }

  pub fn stack_pop(&mut self) -> u16 {
    todo!("")
  }

/*   pub fn stack_push(&mut self, data: u8) {}
  pub fn stack_pop(&mut self) -> u8 {}
  pub fn stack_push_16(&mut self, data: u16) {}
  pub fn stack_pop_16(&mut self) -> u16 {} */

  pub fn load_and_run(&mut self, program: Vec<u8>) {
    self.load(program);
    self.run();
  }

  pub fn load(&mut self, program: Vec<u8>) {
    self.ram[0 .. program.len()].copy_from_slice(&program);
    self.pc = 0;
  }

  pub fn run(&mut self) {
    loop {
      let code = self.ram[self.pc as usize];
      let opcode = OPTABLE.get(&code).unwrap();
      println!("[Running]: {:x}, {:?}", code, opcode);
      self.pc += 1;
      let pc_state = self.pc;

      if let Err(s) = self.decode(opcode) {
        println!("Error {} decoding instruction {:x}", s, code);
        break;
      };

      if pc_state == self.pc {
        self.pc += opcode.bytes as u16;
      }
    }
  }
}