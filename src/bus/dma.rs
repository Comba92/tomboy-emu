use crate::definitions::OAM_START;

use super::BUS;

const DMA_TRANSFER_SIZE: usize = 160;
const DMA_START_DELAY: usize = 4;

pub struct DMA {
  pub active: bool,
  pub source: u16,
  pub bytes: usize,
}

impl DMA {
  pub fn new() -> DMA {
    DMA { active: false, source: 0, bytes: 0 }
  }

  pub fn write(&mut self, data: u8) {
    self.source = u16::from_be_bytes([data, 0x00]);
    self.bytes = 0;
    self.active = true;
  }

  pub fn step(&mut self, mut cycles: usize) -> usize {
    if self.bytes < DMA_START_DELAY {
      cycles = if cycles < DMA_START_DELAY { 0 }
      else { cycles - DMA_START_DELAY }
    }
    
    if self.bytes + cycles >= DMA_TRANSFER_SIZE + DMA_START_DELAY {
      self.active = false;
      self.bytes + cycles - (DMA_TRANSFER_SIZE + DMA_START_DELAY) 
    } else { cycles }
  }
}