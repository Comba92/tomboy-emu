use std::{rc::Rc, cell::RefCell};

use crate::bus::BUS;

mod tile;

bitflags::bitflags! {
  pub struct LCDControl: u8 {
    const BG_N_WINDOW_ENABLE = 1 << 0;
    const SPRITE_ENABLE      = 1 << 1;
    const SPRITE_SIZE        = 1 << 2;
    const BG_TILE_SELECT     = 1 << 3;
    const TILE_SELECT        = 1 << 4;
    const WINDOW_ENABLE      = 1 << 5;
    const WINDOW_SELECT      = 1 << 6;
    const LCD_ENABLE         = 1 << 7;
  }

  pub struct LCDStatus: u8 {
    const PPU_MODE1 = 1 << 0;
    const PPU_MODE2 = 1 << 1;
    const PPU_MODE  = 1 << 1 | 1 << 0;
    const LYC_EQ_LY = 1 << 2;
    const MODE_0    = 1 << 3;
    const MODE_1    = 1 << 4;
    const MODE_2    = 1 << 5;
    const LYC       = 1 << 6;
  }
}

impl LCDControl {
  pub fn new(value: u8) -> Self { Self::from_bits_truncate(value) }
}
impl LCDStatus {
  pub fn new(value: u8) -> Self { Self::from_bits_truncate(value) }
}

pub struct PPU {
  pub memory: Rc<RefCell<BUS>>,
}

impl PPU {
  pub fn new(memory: Rc<RefCell<BUS>>) -> Self {
    PPU {
      memory
    }
  }
}