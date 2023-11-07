use RegisterID::*;
pub enum RegisterID {
  B = 0,
  C = 1,
  D = 2,
  E = 3,
  H = 4,
  L = 5,
  F = 6,
  A = 7
}

pub enum Register16ID {
  BC = 0,
  DE = 1,
  HL = 2,
  AF = 3
}

const ZERO: u8 = 1 << 7;
const NEGATIVE: u8 = 1 << 6;
const HALF_CARRY: u8 = 1 << 5;
const CARRY: u8 = 1 << 4;

pub struct Registers {
  regs: [u8; 8]
}

impl Registers {
  pub fn new() -> Self { Registers { regs: [0; 8] } }
  pub fn get(&self, id: u8) -> u8 { self.regs[id as usize] }
  pub fn set(&mut self, id: u8, data: u8) { self.regs[id as usize] = data; }

  pub fn zero(&self) -> bool { self.get(F as u8) & ZERO != 0 }
  pub fn negative(&self) -> bool { self.get(F as u8) & NEGATIVE != 0}
  pub fn hcarry(&self) -> bool { self.get(F as u8) & HALF_CARRY != 0}
  pub fn carry(&self) -> bool { self.get(F as u8) & CARRY != 0}

  pub fn set_zero(&mut self, cond: bool) { 
    if cond { self.regs[F as usize] |= ZERO; } 
    else { self.regs[F as usize] &= !ZERO }
  }
  pub fn set_negative(&mut self, cond: bool) { 
    if cond { self.regs[F as usize] |= NEGATIVE; } 
    else { self.regs[F as usize] &= !NEGATIVE }
  }
  pub fn set_hcarry(&mut self, cond: bool) { 
    if cond { self.regs[F as usize] |= HALF_CARRY; } 
    else { self.regs[F as usize] &= !HALF_CARRY }
  }
    pub fn set_carry(&mut self, cond: bool) { 
    if cond { self.regs[F as usize] |= CARRY; } 
    else { self.regs[F as usize] &= !CARRY }
  }

  pub(super) fn update_carry(&mut self, value: u8) {
    self.set_carry(value & 0b1000_0000 != 0);
  }

  pub(super) fn update_hcarry(&mut self, a: u8, b: u8) {
    let hc = (a & 0xf) + (b & 0xf) & 0x10;
    self.set_hcarry(hc == 0x10);
  }

  pub fn get_16(&self, id: u8) -> u16 { 
    match id {
      0 => self.get_bc(),
      1 => self.get_de(),
      2 => self.get_hl(),
      3 => self.get_af(),
      _ => panic!("Impossible register_16 ID.")
    }
  }

  pub fn set_16(&mut self, id: u8, data: u16) { 
    match id {
      0 => self.set_bc(data),
      1 => self.set_de(data),
      2 => self.set_hl(data),
      3 => self.set_af(data),
      _ => panic!("Impossible register_16 ID.")
    }
  }

  pub fn get_af(&self) -> u16 { u16::from_be_bytes([self.get(A as u8), self.get(F as u8)]) }
  pub fn get_bc(&self) -> u16 { u16::from_be_bytes([self.get(B as u8), self.get(C as u8)]) }
  pub fn get_de(&self) -> u16 { u16::from_be_bytes([self.get(D as u8), self.get(E as u8)]) }
  pub fn get_hl(&self) -> u16 { u16::from_be_bytes([self.get(H as u8), self.get(L as u8)]) }

  pub fn set_af(&mut self, data: u16) { 
    let [high, low] = data.to_be_bytes();
    self.set(A as u8, high);
    self.set(F as u8, low);
  }
  pub fn set_bc(&mut self, data: u16) { 
    let [high, low] = data.to_be_bytes();
    self.set(B as u8, high);
    self.set(C as u8, low);
  }
    pub fn set_de(&mut self, data: u16) { 
    let [high, low] = data.to_be_bytes();
    self.set(D as u8, high);
    self.set(E as u8, low);
  }
    pub fn set_hl(&mut self, data: u16) { 
    let [high, low] = data.to_be_bytes();
    self.set(H as u8, high);
    self.set(L as u8, low);
  }
}