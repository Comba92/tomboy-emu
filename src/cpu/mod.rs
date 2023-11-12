#![allow(dead_code)]
use crate::definitions::{PC_INIT, SP_INIT};
use self::registers::Registers;

mod registers;
mod instructions;
mod decoding;

pub struct CPU {
  pub registers: Registers,

  pub sp: u16,
  pub pc: u16,
  pub ram: [u8; 1024],
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      registers: Registers::new(),
      sp: PC_INIT,
      pc: SP_INIT,
      ram: [0; 1024],
    }
  }

  pub fn mem_read(&self, addr: u16) -> u8 {
    self.ram[addr as usize]
  }
  pub fn mem_write(&mut self, addr: u16, data: u8) {
    self.ram[addr as usize] = data;
  }
  
  pub fn mem_read_16(&self, addr: u16) -> u16 {
    todo!("Don't know if it is little or big endian")
  }

  pub fn mem_write_16(&mut self, addr: u16, data: u16) {
    // first lower, then upper, so maybe little endian
    todo!("Don't know if it is little or big endian")
  }

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
      // let opcode = OPTABLE.unprefixed.get(&code).unwrap();
      // println!("[Running]: {:x}, {:?}", code, opcode);
      self.pc += 1;

      if let Err(s) = self.decode(code) {
        println!("Error {} decoding instruction {:x}", s, code);
        break;
      };
    }
  }
}