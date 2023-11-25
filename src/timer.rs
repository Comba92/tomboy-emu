use crate::definitions::DIV_INIT;

pub struct Timer {
  pub div: u16,
  pub tima: u8,
  pub tma: u8,
  pub tac: u8,
  pub div_cycles: usize,
  pub tima_cycles: usize
}

impl Timer {
  pub fn new() -> Self {
    Timer {
      div: DIV_INIT, tima: 0, tma: 0, tac: 0, div_cycles: 0, tima_cycles: 0
    }
  }

  pub fn tick(&mut self, cycles: usize) -> bool {
    self.div_cycles = self.div_cycles.wrapping_add(cycles);

    if self.div_cycles >= 256 {
      self.div_cycles = self.div_cycles.wrapping_sub(256);
      self.div = self.div.wrapping_add(1);
    }

    let mut overflowed: bool = false;
    
    if self.is_timer_enabled() {
      self.tima_cycles = self.tima_cycles.wrapping_add(cycles * 4);

      if self.tima_cycles > self.get_frequency() {
        self.tima_cycles = self.tima_cycles
          .wrapping_sub(self.get_frequency());
        
        (self.tima, overflowed) = self.tima.overflowing_add(1);

        if overflowed {
          self.tima = self.tma;
        }
      }
    }

    overflowed
  }

  pub fn is_timer_enabled(&self) -> bool {
    self.tac & 0b0100 != 0
  }

  pub fn get_frequency(&self) -> usize {
    match self.tac & 0b0011 {
      0b00 => 1024,
      0b01 => 16,
      0b10 => 64,
      0b11 => 256,
      _ => panic!("TAC register bad set")
    }
  }
}