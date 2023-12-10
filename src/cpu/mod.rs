#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use crate::{definitions::*, bus::{BUS, InterruptRegister}};
use log::{debug, info, trace};
use optable::OPTABLE;
use addressing::Opcode;

mod addressing;
mod instructions;
mod decode;
pub mod optable;

bitflags::bitflags! {
  pub struct Flags: u8 {
    const ZERO   = 1 << 7;
    const SUB    = 1 << 6;
    const HCARRY = 1 << 5;
    const CARRY  = 1 << 4;
  }
}

impl Flags {
  pub fn new(value: u8) -> Self { Self::from_bits_truncate(value) }
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

  pub ime: bool,
  pub ime_to_set: bool,
  pub halted: bool,

  pub sp: u16,
  pub pc: u16,
  pub memory: Rc<RefCell<BUS>>,
}

// Boilerplate, constructor, getter, setter
impl CPU {
  pub fn new(memory: Rc<RefCell<BUS>>) -> Self {
    CPU {
      a: A_INIT,
      f: Flags::new(F_INIT),
      b: B_INIT,
      c: C_INIT,
      d: D_INIT,
      e: E_INIT,
      h: H_INIT,
      l: L_INIT,
      sp: SP_INIT,
      pc: PC_INIT,
      ime: false,
      ime_to_set: false,
      halted: false,
      memory,
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

  pub fn mem_read(&self, addr: u16) -> u8 {
    self.memory.borrow()
    .mem_read(addr)
  }
  pub fn mem_write(&mut self, addr: u16, data: u8) {
    self.memory.borrow_mut()
    .mem_write(addr, data);
  }

  pub fn tick(&mut self, cycles: usize) {
    self.memory.borrow_mut()
    .tick(cycles);
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

  pub fn get_ie(&self) -> InterruptRegister {
    InterruptRegister::new( self.mem_read(INTERRUPT_ENABLE) )
  }

  pub fn get_if(&self) -> InterruptRegister {
    InterruptRegister::new( self.mem_read(INTERRUPT_FLAG) )
  }

  pub fn set_if(&mut self, int: InterruptRegister) {
    self.mem_write(INTERRUPT_FLAG, int.bits());
    info!("[SET_IF] VAL = {:?} - NEW_IF = {:?}", int, self.get_if());
  }
}

// Important Stuff
impl CPU {
  pub fn stack_push(&mut self, data: u16) {
    self.mem_write_16(self.sp.wrapping_sub(2), data);
    self.sp = self.sp.wrapping_sub(2);
  }

  pub fn stack_pop(&mut self) -> u16 {
    let value = self.mem_read_16(self.sp);
    self.sp = self.sp.wrapping_add(2);
    value
  }

  pub fn halt(&mut self) {
    let if_reg = self.get_if();
    let ie_reg = self.get_ie();

    if if_reg.bits() & ie_reg.bits() == 0 {
      self.halted = true;
      self.tick(4);
    }
  }

  pub fn interrupts_handle(&mut self) {
    let mut if_reg = self.get_if();
    let ie_reg = self.get_ie();

    // if there aren't any requested interrupts
    if if_reg.bits() & ie_reg.bits() == 0 {
      return;
    }

    // The CPU wakes up as soon as an interrupt is pending, that is,
    // when the bitwise AND of IE and IF is non-zero.
    self.halted = false;

    // TODO: here the halt bug happens
    if !self.ime { return; }

    info!("[InterruptsHandler] Checking for interrupts...");
    for (_, interrupt) in if_reg.iter_names() {
      info!("[InterruptsHandler] {:?}", interrupt);
      if ie_reg.contains(interrupt) {
        self.ime = false;
        if_reg.remove(interrupt);
        self.set_if(if_reg);

        self.interrupt_call(interrupt);
        break;
      }
    }
  }

  pub fn interrupt_call(&mut self, int: InterruptRegister) {
    self.tick(2 * 4);
    self.stack_push(self.pc);
    self.tick(2 * 4);

    info!("[InterruptCall] PC pushed. Redirecting to interrupt vector...");
    match int {
      InterruptRegister::VBLANK => self.pc = 0x40,
      InterruptRegister::LCD    => self.pc = 0x48,
      InterruptRegister::TIMER  => self.pc = 0x50,
      InterruptRegister::SERIAL => self.pc = 0x58,
      InterruptRegister::JOYPAD => self.pc = 0x60,
      _ => { panic!("[InterruptCall] No redirection to interrupt vector!") }
    }

    info!("[InterruptCall] Interrupt redirected correctly to {:x}.", self.pc);
    self.tick(1 * 4);
  }

  pub fn is_blargg_test_finished(&self) -> bool {
    // debug termination check
    let code = self.mem_read(self.pc);
    if code == 0xe0 && self.mem_read(self.pc + 1) == 0x26
        && self.mem_read(self.pc + 2) == 0x18
        && self.mem_read(self.pc + 3) == 0xfe
    { true } else { false }
  }
  
  pub fn run(&mut self) {
    loop {
      let result = self.step();
      if result.is_err() {
        break;
      }
    }
  }

  pub fn step(&mut self) -> Result<(), &'static str>{
    if self.is_blargg_test_finished() {
      return Err("Blargg test done");
    }

    self.interrupts_handle();

    if self.ime_to_set {
      info!("[EI] IME Enabled - IF Flag: {:?}, IE Flag: {:?}", self.get_if(), self.get_ie());
      self.ime_to_set = false;
      self.ime = true;
    }
    
    self.log_debug();
    if self.halted {
      self.tick(4);
      return Ok(());
    }

    let code = self.mem_read(self.pc);
    let opcode = if code == 0xCB {
      let code = 0xCB00 | self
        .mem_read(self.pc.wrapping_add(1))
        as u16;
      OPTABLE.get(&code).unwrap()
    } else {
      OPTABLE.get(&(code as u16)).unwrap()
    };

    self.pc = self.pc.wrapping_add(opcode.bytes as u16);
    let pc_before_jpc = self.pc;
    
    self.decode(opcode);

    if opcode.cycles.1 != 0 && self.pc == pc_before_jpc {
      self.tick(opcode.cycles.1);
    } else {
      self.tick(opcode.cycles.0);
    }

    if self.mem_read(0xff02) == 0x81 {
      eprint!("{}", self.mem_read(0xff01) as char);
      self.mem_write(0xff02, 0);
    }
    
    Ok(())
  }

  #[deprecated]
  pub fn log_debug_old(&self) {
    println!(
      "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
      self.a, self.f.bits(), self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc,
      self.mem_read(self.pc), self.mem_read(self.pc+1), self.mem_read(self.pc+2), self.mem_read(self.pc+3),
    )
  }

  pub fn log_debug(&self) {
    trace!(
      "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
      self.a, self.f.bits(), self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc,
      self.mem_read(self.pc), self.mem_read(self.pc+1), self.mem_read(self.pc+2), self.mem_read(self.pc+3)
    )
  }

  pub fn log_op(&self, opcode: &Opcode) {
    let second = self.mem_read(self.pc.wrapping_add(1)); 
    let third =  self.mem_read(self.pc.wrapping_add(2));
    trace!("[Running] {:#06x}: {},\t({:#04x}, {:#04x}, {:#04x})", self.pc, opcode.name, opcode.code, second, third);
  }
}