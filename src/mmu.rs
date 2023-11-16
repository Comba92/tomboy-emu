use crate::definitions::*;
use bitflags::bitflags;

bitflags! {
  pub struct InterruptRegister: u8 {
    const VBLANK = 1 << 0;
    const LCD    = 1 << 1;
    const TIMTER = 1 << 2;
    const SERIAL = 1 << 3;
    const JOYPAD = 1 << 4;
  }
}

impl InterruptRegister {
  pub fn new(value: u8) -> Self { Self::from_bits_truncate(value) }
}

pub struct MMU {
  rom: Vec<u8>,
  pub ram: [u8; 1024 * 4],
  hram: [u8; 128],
  ie_reg: InterruptRegister,
  if_reg: InterruptRegister,
}

impl MMU {
  pub fn new(rom: Vec<u8>) -> Self {
    MMU {
      ram: [0; 1024 * 4],
      hram: [0; 128],
      rom,
      ie_reg: InterruptRegister::new(0),
      if_reg: InterruptRegister::new(0),
    }
  }

  pub fn mem_read(&self, addr: u16) -> u8 {
    match addr {
      0xff0f => self.if_reg.bits(),
      0xffff => self.ie_reg.bits(),

      ROM_START ..= ROM_END => self.rom[addr as usize],
      VRAM_START ..= VRAM_END => { eprintln!("VRAM address range not implemented."); 0 },
      EXT_RAM_START ..= EXT_RAM_END => { eprintln!("EXT RAM address range not implemented."); 0 },
      WRAM_START ..= WRAM_END => self.ram[(addr - WRAM_START) as usize],
      IO_REGISTERS_START ..= IO_REGISTERS_END => { eprintln!("IO Registers address range not implemented."); 0 },
      HRAM_START ..= HRAM_END => self.hram[(addr - HRAM_START) as usize],

      _ => { eprintln!("Addressing not implemented for address {addr:#04x}"); 0 }
    }
  }


  pub fn mem_write(&mut self, addr: u16, data: u8) {
    match addr {
      0xff0f => self.if_reg = InterruptRegister::new(data),
      0xffff => self.ie_reg = InterruptRegister::new(data),

      ROM_START ..= ROM_END => panic!("Trying to write ROM memory at {addr:#04x}."),
      VRAM_START ..= VRAM_END => eprintln!("VRAM address range not implemented."),
      EXT_RAM_START ..= EXT_RAM_END => eprintln!("EXT RAM address range not implemented."),
      WRAM_START ..= WRAM_END => self.ram[(addr - WRAM_START) as usize] = data,
      IO_REGISTERS_START ..= IO_REGISTERS_END => eprintln!("IO Registers address range not implemented."),
      HRAM_START ..= HRAM_END => self.hram[(addr - HRAM_START) as usize] = data,

      _ => { eprintln!("Addressing not implemented for address {addr:#04x}"); }
    };
  }
}

