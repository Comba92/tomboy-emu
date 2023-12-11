use crate::definitions::*;
use bitflags::bitflags;
use log::{info, warn};

mod timer;
pub mod lcd;
mod dma;

use timer::Timer;
use lcd::{LCD, LCDControl, LCDStatus};
use dma::DMA;

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
  pub rom: Vec<u8>,
  pub vram: [u8; 1024 * 8],
  pub wram: [u8; 1024 * 8],
  pub oam: [u8; 160],
  pub hram: [u8; 128],

  pub timer: Timer,
  pub lcd: LCD,
  pub dma: DMA,
  ie_reg: InterruptRegister,
  if_reg: InterruptRegister,

  vram_lock: bool,
  oam_lock: bool,

  io_regs: [u8; 128],
  serial_transfer: [u8; 2],
}

impl BUS {
  pub fn new(mut rom: Vec<u8>) -> Self {
    rom.resize(ROM_END as usize + 1,0);

    BUS {
      rom,
      vram: [0; 1024 * 8],
      wram: [0; 1024 * 8],
      oam: [0; 160],
      hram: [0; 128],

      vram_lock: false,
      oam_lock: false,

      timer: Timer::new(),
      lcd: LCD::new(),
      dma: DMA::new(),
      ie_reg: InterruptRegister::new(0),
      if_reg: InterruptRegister::new(0),

      io_regs: [0; 128],
      serial_transfer: [0; 2],
    }
  }

  pub fn tick(&mut self, cycles: usize) {
    let tima_overflow = self.timer.step(cycles);
    if tima_overflow {
      self.if_reg.insert(InterruptRegister::TIMER);
      info!("[BUS] TIMA overflowed. Interrupt requested {:?}", self.if_reg);
    }

    if self.dma.active {
      let to_transfer = self.dma.step(cycles);
      for i in 0..to_transfer {
        let src = self.dma.source + (self.dma.bytes + i ) as u16;
        let dst = u16::from_be_bytes([0xFE, (self.dma.bytes + i) as u8]);

        let byte = self.mem_read(src);
        self.mem_write(dst, byte);
      }

      self.dma.bytes += to_transfer;
    }

    self.lcd.ly = self.lcd.ly.wrapping_add(1);
  }

  pub fn mem_read(&self, addr: u16) -> u8 {
    match addr {
      0x0000 ..= 0x7fff => self.rom[addr as usize],
      0x8000 ..= 0x9fff => self.vram[(addr - 0x8000) as usize],
      0xa000 ..= 0xbfff => { eprintln!("EXT RAM address range not implemented."); 0 },
      0xc000 ..= 0xdfff => self.wram[(addr - 0xc000) as usize],
      0xfe00 ..= 0xfe9f => self.oam[(addr - 0xfe00) as usize],

      0xff04 => self.timer.div.to_be_bytes()[0],
      0xff05 => self.timer.tima,
      0xff06 => self.timer.tma,
      0xff07 => self.timer.tac,

      0xff40 => self.lcd.ctrl.bits(),
      0xff41 => self.lcd.stat.bits(),
      0xff42 => self.lcd.scroll.1,
      0xff43 => self.lcd.scroll.0,
      0xff44 => self.lcd.ly,
      0xff45 => self.lcd.lyc,
      0xff46 => self.dma.source.to_be_bytes()[0],
      0xff47 => self.lcd.bg_palette,
      0xff48 => self.lcd.obj_palette0,
      0xff49 => self.lcd.obj_palette1,
      0xff4a => self.lcd.window.1,
      0xff4b => self.lcd.window.0,

      0xff0f => self.if_reg.bits(),
      0xffff => self.ie_reg.bits(),

      IO_REGISTERS_START ..= IO_REGISTERS_END => self.io_regs[(addr -  IO_REGISTERS_START) as usize],
      0xff80 ..= 0xfffe => self.hram[(addr - 0xff80) as usize],

      _ => { eprintln!("Addressing not implemented for address {addr:#04x}"); 0 }
    }
  }


  pub fn mem_write(&mut self, addr: u16, data: u8) {
    match addr {
      0x0000 ..= 0x7fff => eprintln!("Trying to write ROM memory at {addr:#04x}."),
      0x8000 ..= 0x9fff => self.vram[(addr - 0x8000) as usize] = data,
      0xa000 ..= 0xbfff => eprintln!("EXT RAM address range not implemented."),
      0xc000 ..= 0xdfff => self.wram[(addr - 0xc000) as usize] = data,
      0xfe00 ..= 0xfe9f => self.oam[(addr - 0xfe00) as usize] = data,


      0xff04 => self.timer.div = 0,
      0xff05 => self.timer.tima = data,
      0xff06 => self.timer.tma = data,
      0xff07 => self.timer.tac = data,
      
      0xff40 => self.lcd.ctrl = LCDControl::new(data),
      //0xff41 => { todo!("not all bits of lcdstate can be written") },
      0xff42 => self.lcd.scroll.1 = data,
      0xff43 => self.lcd.scroll.0 = data,
      0xff45 => self.lcd.lyc = data,
      0xff46 => self.dma.write(data),
      0xff47 => self.lcd.bg_palette = data,
      0xff48 => self.lcd.obj_palette0 = data,
      0xff49 => self.lcd.obj_palette1 = data,
      0xff4a => self.lcd.window.1 = data,
      0xff4b => self.lcd.window.0 = data,

      0xff0f => self.if_reg = InterruptRegister::new(data),
      0xffff => self.ie_reg = InterruptRegister::new(data),

      IO_REGISTERS_START ..= IO_REGISTERS_END => self.io_regs[(addr -  IO_REGISTERS_START) as usize] = data,
      0xff80 ..= 0xfffe => self.hram[(addr - 0xff80) as usize] = data,

      _ => { eprintln!("Addressing not implemented for address {addr:#04x}"); }
    };
  }
}

