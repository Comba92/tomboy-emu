use std::cell::RefCell;

use cartrdige::Cartridge;
use cpu::CPU;
use bus::BUS;
use timer::Timer;

pub mod cpu;
pub mod ppu;
pub mod bus;
pub mod timer;
pub mod cartrdige;
pub mod definitions;

pub struct Emulator {
  cpu: CPU,
  timer: Timer,
  cartridge: Cartridge,
  memory: RefCell<BUS>
}

impl Emulator {
}