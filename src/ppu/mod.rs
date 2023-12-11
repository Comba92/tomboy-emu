use std::{rc::Rc, cell::RefCell};

use crate::{bus::{BUS, lcd::{LCD, LCDStatus, LCDControl}, InterruptRegister}, definitions::{LCD_WIDTH, LCD_HEIGHT, INTERRUPT_FLAG}};

mod fifo;

pub struct PPU {
  pub memory: Rc<RefCell<BUS>>,
  pub framebuffer: [u8; LCD_WIDTH * LCD_HEIGHT],
  pub mode: PPUMode,
  pub scanline_cycles: usize,
  pub scanline_pixels: usize,
}

pub enum PPUMode {
  HBlank, VBlank, OAMScan, Drawing
}

use PPUMode::*;

impl PPU {
  pub fn new(memory: Rc<RefCell<BUS>>) -> Self {
    PPU {
      memory, framebuffer: [0; LCD_WIDTH * LCD_HEIGHT],
      scanline_cycles: 0, scanline_pixels: 0, 
      mode: OAMScan,
    }
  }

  pub fn step(&mut self) {
    self.scanline_cycles += 1;

    match self.mode {
      OAMScan => {
        if self.scanline_cycles == 80 {
          self.scanline_pixels = 0;

          // TODO: implement sprites rendering

          self.mode = Drawing;
        }
      },
      Drawing => {
        self.scanline_pixels += 1;
        if self.scanline_pixels == 160 {
          self.mode = HBlank;
          if self.get_lcd_stat().contains(LCDStatus::HBLANK_INT) {
            self.send_stat_interrupt();
          }
        }
      },
      HBlank => {
        if self.scanline_cycles == 456 {
          self.scanline_cycles = 0;
          self.inc_ly();

          if self.get_ly() == 144 {
            self.mode = VBlank;
            self.send_vblank_interrupt();
            if self.get_lcd_stat().contains(LCDStatus::VBLANK_INT) {
              self.send_stat_interrupt();
            }

          } else { self.mode = OAMScan; }
        }
      },
      VBlank => {
        if self.scanline_cycles == 456 {
          self.inc_ly();

          if self.get_ly() == 153 {
            self.mode = OAMScan;
            self.reset_ly();
            self.scanline_cycles = 0;
          }
        }
      }
    };
  }

  pub fn get_ly(&self) -> u8 {
    self.memory.borrow().lcd.ly
  }

  pub fn inc_ly(&self) {
    self.memory.borrow_mut().lcd.ly += 1;
  }

  pub fn reset_ly(&self) {
    self.memory.borrow_mut().lcd.ly = 0;
  }

  pub fn get_lcd_stat(&self) -> LCDStatus {
    self.memory.borrow().lcd.stat
  } 

  pub fn get_lcd_ctrl(&self) -> LCDControl {
    self.memory.borrow().lcd.ctrl
  }

  fn send_vblank_interrupt(&self) {
    self.send_interrupt(InterruptRegister::VBLANK);
  }

  fn send_stat_interrupt(&self) {
    self.send_interrupt(InterruptRegister::LCD);
  }

  fn send_interrupt(&self, int: InterruptRegister) {
    let mut bus = self.memory.borrow_mut();
    let mut if_reg = InterruptRegister::new(bus.mem_read(INTERRUPT_FLAG));
    if_reg.insert(int);
    bus.mem_write(INTERRUPT_FLAG, if_reg.bits());
  }
}