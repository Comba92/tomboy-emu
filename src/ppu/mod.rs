mod tiles;

bitflags::bitflags! {
  pub struct LCDControl: u8 {
    const BG_N_WINDOW_ENABLE = 1 << 0;
    const OBJ_ENABLE         = 1 << 1;
    const OBJ_SIZE           = 1 << 2;
    const BG_TILE_MAP        = 1 << 3;
    const BG_N_WINDOW_TILES  = 1 << 4;
    const WINDOW_ENABLE      = 1 << 5;
    const WINDOW_TILE_MAP    = 1 << 6;
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
  vram: [u8; 1024 * 8],
}

impl PPU {

}