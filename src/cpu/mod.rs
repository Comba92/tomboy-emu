#![allow(dead_code)]

use bitflags::bitflags;
use crate::definitions::{PC_INIT, SP_INIT};

use self::optable::OPTABLE;

pub mod optable;
mod instructions;
mod operands;


bitflags! {
  pub struct Flags: u8 {
    const ZERO   = 1 << 7;
    const NEG    = 1 << 6;
    const HCARRY = 1 << 5;
    const CARRY  = 1 << 4;
  }
}

pub struct CPU {
  pub reg_a: u8,
  pub reg_f: Flags,

  pub reg_b: u8,
  pub reg_c: u8,

  pub reg_d: u8,
  pub reg_e: u8,

  pub reg_h: u8,
  pub reg_l: u8,

  pub sp: u16,
  pub pc: u16,
  pub ram: [u8; 1024],
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      reg_a: 0,
      reg_f: Flags::from_bits_truncate(0),

      reg_b: 0,
      reg_c: 0,

      reg_d: 0,
      reg_e: 0,

      reg_h: 0,
      reg_l: 0,

      sp: PC_INIT,
      pc: SP_INIT,
      ram: [0; 1024],
    }
  }

  pub fn reg_af(&self) -> u16 { u16::from_be_bytes([self.reg_a, self.reg_f.bits()])}
  pub fn reg_bc(&self) -> u16 { u16::from_be_bytes([self.reg_b, self.reg_c])}
  pub fn reg_de(&self) -> u16 { u16::from_be_bytes([self.reg_d, self.reg_e])}
  pub fn reg_hl(&self) -> u16 { u16::from_be_bytes([self.reg_h, self.reg_l])}

  pub fn set_reg_af(&mut self, data: u16) {
    let [high, low] = data.to_be_bytes();
    self.reg_a = high;
    self.reg_f = Flags::from_bits_truncate(low);
  }
  pub fn set_reg_bc(&mut self, data: u16) { [self.reg_b, self.reg_c] = data.to_be_bytes(); }
  pub fn set_reg_de(&mut self, data: u16) { [self.reg_d, self.reg_e] = data.to_be_bytes(); }
  pub fn set_reg_hl(&mut self, data: u16) { [self.reg_h, self.reg_l] = data.to_be_bytes(); }

  pub fn mem_read(&self, addr: u16) -> u8 {
    self.ram[addr as usize]
  }
  pub fn mem_write(&mut self, addr: u16, data: u8) {
    self.ram[addr as usize] = data;
  }
  
  pub fn mem_read16(&self, addr: u16) -> u16 {
    let high = self.mem_read(addr);
    let low = self.mem_read(addr + 1);
    println!("[MEM_READ16]: {:x}", u16::from_be_bytes([high, low]));
    u16::from_be_bytes([high, low]) 
  }

  pub fn mem_write16(&mut self, addr: u16, data: u16) {
    let [high, low] = data.to_be_bytes();
    println!("[MEM_WRITE16]: {:x}", u16::from_be_bytes([high, low]));
    self.mem_write(addr, high);
    self.mem_write(addr + 1, low);
  }


  pub fn load(&mut self, program: Vec<u8>) {
    self.ram[0 .. program.len()].copy_from_slice(&program);
    self.pc = 0;
  }

  pub fn run(&mut self) {
    loop {
      let code = self.ram[self.pc as usize];
      let opcode = OPTABLE.unprefixed.get(&code).unwrap();
      println!("[Running]: {:x}, {:?}", code, opcode);
      self.pc += 1;

      match code {
        0x00 => return,
        0x02 | 0xea => self.ld(&opcode.operands),
        _ => panic!("Opcode {:#?} not implemented.", opcode)
      }

      self.pc += opcode.bytes as u16;
    }
  }
}