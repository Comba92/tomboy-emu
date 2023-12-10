use log::info;

use crate::definitions::DIV_INIT;

pub struct Timer {
  pub div: u16,
  pub tima: u8,
  pub tma: u8,
  pub tac: u8,
  pub cycles: usize,
}

impl Timer {
  pub fn new() -> Self {
    Timer {
      div: DIV_INIT, tima: 0, tma: 0, tac: 0, cycles: 0
    }
  }

  pub fn step(&mut self, cycles: usize) -> bool {
    self.div = self.div.wrapping_add(cycles as u16);

    let mut overflowed: bool = false;
    
    if self.is_timer_enabled() {
      info!("[Timer] Increasing cycles");
      self.cycles = self.cycles.wrapping_add(cycles);

      if self.cycles > self.get_frequency() {
        info!("[Timer] Increasing registers");

        self.cycles = self.cycles
          .wrapping_sub(self.get_frequency());
        
        (self.tima, overflowed) = self.tima.overflowing_add(1);

        if overflowed {
          info!("[Timer] TIMA overflowed.");
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