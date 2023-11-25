use crate::{definitions::*, ppu::PPU, timer::Timer};
use bitflags::bitflags;

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct InterruptRegister: u8 {
    const VBLANK = 1 << 0;
    const LCD    = 1 << 1;
    const TIMER = 1 << 2;
    const SERIAL = 1 << 3;
    const JOYPAD = 1 << 4;
  }

  pub struct SerialControl: u8 {
    const CLOCK_SELECT = 1 << 0;
    const CLOCK_SPEED  = 1 << 1;
    const TRANSFER_ENABLE = 1 << 7;
  }
}

impl InterruptRegister {
  pub fn new(value: u8) -> Self { Self::from_bits_truncate(value) }
}
impl SerialControl {
  pub fn new(value: u8) -> Self { Self::from_bits_truncate(value) }
}

pub struct BUS {
  rom: Vec<u8>,
  pub ram: [u8; 1024 * 8],
  hram: [u8; 128],
  ie_reg: InterruptRegister,
  if_reg: InterruptRegister,

  ppu: PPU,
  timer: Timer,
  io_regs: [u8; 128],
  serial_transfer: [u8; 2],
}

impl BUS {
  pub fn new(mut rom: Vec<u8>) -> Self {
    rom.resize(0x8000, 0);

    BUS {
      ram: [0; 1024 * 8],
      hram: [0; 128],
      rom,
      ie_reg: InterruptRegister::new(0),
      if_reg: InterruptRegister::new(0),

      ppu: PPU::new(),
      timer: Timer::new(),
      io_regs: [0; 128],
      serial_transfer: [0; 2],
    }
  }

  pub fn tick(&mut self, cycles: usize) {
    let tima_overflow = self.timer.tick(cycles);
    if tima_overflow {
      self.if_reg.insert(InterruptRegister::TIMER);
    }
  }

  pub fn mem_read(&self, addr: u16) -> u8 {
    match addr {
      // required by blargg tests
      0xff44 => 0x90,

      ROM_START ..= ROM_END => self.rom[addr as usize],
      VRAM_START ..= VRAM_END => {
        // eprintln!("[READ] {:?}", self.ppu.vram);
        self.ppu.vram[(addr - VRAM_START) as usize]
      }
      EXT_RAM_START ..= EXT_RAM_END => { eprintln!("EXT RAM address range not implemented."); 0 },
      WRAM_START ..= WRAM_END => self.ram[(addr - WRAM_START) as usize],

      0xff04 => self.timer.div.to_be_bytes()[0],
      0xff05 => self.timer.tima,
      0xff06 => self.timer.tma,
      0xff07 => self.timer.tac,

      0xff0f => self.if_reg.bits(),
      0xffff => self.ie_reg.bits(),

      IO_REGISTERS_START ..= IO_REGISTERS_END => self.io_regs[(addr -  IO_REGISTERS_START) as usize],
      HRAM_START ..= HRAM_END => self.hram[(addr - HRAM_START) as usize],

      _ => { eprintln!("Addressing not implemented for address {addr:#04x}"); 0 }
    }
  }


  pub fn mem_write(&mut self, addr: u16, data: u8) {
    match addr {
      ROM_START ..= ROM_END => panic!("Trying to write ROM memory at {addr:#04x}."),
      VRAM_START ..= VRAM_END => {
        self.ppu.vram[(addr - VRAM_START) as usize] = data;
        // eprintln!("[WRITE]{:?}", self.ppu.vram);
      }
      EXT_RAM_START ..= EXT_RAM_END => eprintln!("EXT RAM address range not implemented."),
      WRAM_START ..= WRAM_END => self.ram[(addr - WRAM_START) as usize] = data,

      0xff04 => self.timer.div = 0,
      0xff05 => self.timer.tima = data,
      0xff06 => self.timer.tma = data,
      0xff07 => self.timer.tac = data,

      0xff0f => self.if_reg = InterruptRegister::new(data),
      0xffff => self.ie_reg = InterruptRegister::new(data),

      IO_REGISTERS_START ..= IO_REGISTERS_END => self.io_regs[(addr -  IO_REGISTERS_START) as usize] = data,
      HRAM_START ..= HRAM_END => self.hram[(addr - HRAM_START) as usize] = data,

      _ => { eprintln!("Addressing not implemented for address {addr:#04x}"); }
    };
  }

  fn io_registers_read(&mut self, addr: u16) -> u8 {
    match addr {
      0xff00 => todo!("joypad input"),
      0xff01 | 0xff02 => self.serial_transfer[(0xff01 - addr) as usize],
      0xff04 ..= 0xff07 => todo!("timer and divder"),

      0xff10 ..= 0xff3f => todo!("audio and wave pattern"),

      0xff40 ..= 0xff4b => todo!("lcd registers"),
      0xff4f => todo!("vram bank select"),

      0xff50 => todo!("set to non-zero to disable boot rom"),
      
      _ => panic!("Addressing not implemented for address {addr:#04x}"),
    }
  }

  fn io_registers_write(&mut self, addr: u16, data: u8) {
    match addr {
      0xff00 => todo!("joypad input"),
      0xff01 | 0xff02 => self.serial_transfer[(0xff01 - addr) as usize] = data,
      0xff04 ..= 0xff07 => todo!("timer and divder"),

      0xff10 ..= 0xff3f => todo!("audio and wave pattern"),

      0xff40 ..= 0xff4b => todo!("lcd registers"),
      0xff4f => todo!("vram bank select"),

      0xff50 => todo!("set to non-zero to disable boot rom"),
      
      _ => panic!("Addressing not implemented for address {addr:#04x}"),
    }
  }
}

