use std::{rc::Rc, cell::RefCell};

use crate::{bus::BUS, definitions::{LCD_WIDTH, LCD_HEIGHT}};

mod tile;


pub struct PPU {
  pub memory: Rc<RefCell<BUS>>,
  pub framebuffer: [u8; LCD_WIDTH * LCD_HEIGHT],
  pub mode: PPUMode,
  pub pixel: usize,
  pub scanlines: usize,
}

pub enum PPUMode {
  HBlank, VBlank, OAMScan, Drawing
}

use PPUMode::*;

impl PPU {
  pub fn new(memory: Rc<RefCell<BUS>>) -> Self {
    PPU {
      memory, framebuffer: [0; LCD_WIDTH * LCD_HEIGHT], pixel: 0, scanlines: 0, mode: OAMScan,
    }
  }

  pub fn step(&mut self) {
    match self.mode {
      OAMScan => unimplemented!("OAM not implemented"),
      Drawing => {

      },
      HBlank => {},
      VBlank => {}
    };
  }
}