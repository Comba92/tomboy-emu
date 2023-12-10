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

pub struct LCD {
  pub ctrl: LCDControl,
  pub stat: LCDStatus,
  
  pub scroll: (u8, u8),
  pub window: (u8, u8),
  
  pub ly: u8,
  pub lyc: u8,

  pub bg_palette: u8,
  pub obj_palette0: u8,
  pub obj_palette1: u8
}

impl LCD {
  pub fn new() -> LCD {
    LCD {
      ly: 0, lyc: 0, ctrl: LCDControl::new(0), stat: LCDStatus::new(0),
      scroll: (0,0), window: (0,0),
      bg_palette: 0, obj_palette0: 0, obj_palette1: 0,
    }
  }
}