use std::{cell::RefCell, rc::Rc};

use cpu::CPU;
use bus::BUS;
use ppu::PPU;

pub mod cpu;
pub mod ppu;
pub mod bus;
pub mod cartrdige;
pub mod definitions;

pub struct Emulator {
  pub cpu: CPU,
  pub ppu: PPU,
  pub memory: Rc<RefCell<BUS>>,
  
  // TODO
  // pub cartridge: CartridgeData,
}

impl Emulator {
  pub fn new(rom: Vec<u8>) -> Emulator {
    // let cartridge = CartridgeData::new(&rom);
    let memory = Rc::new(RefCell::new(BUS::new(rom)));
    
    let cpu = CPU::new(Rc::clone(&memory));
    let ppu = PPU::new(Rc::clone(&memory));

    Emulator { cpu, ppu, memory }
  }

  // to delete later
  pub fn run(&mut self) {
    self.cpu.run();
  }

  pub fn step(&mut self) -> Result<(), &str> {
    let res = self.cpu.step();
    for _ in 0..4 { self.ppu.step() }

    res
  }
}