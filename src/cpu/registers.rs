use Register8::*;
use enum_primitive_derive::Primitive;
use num::ToPrimitive;


#[derive(Primitive)]
pub enum Register8 {
  B = 0,
  C = 1,
  D = 2,
  E = 3,
  H = 4,
  L = 5,
  F = 6,
  A = 7
}

#[derive(Primitive)]
pub enum Register16 {
  BC = 0,
  DE = 1,
  HL = 2,
  AF = 3
}

const ZERO: u8 = 1 << 7;
const SUB: u8 = 1 << 6;
const HALF_CARRY: u8 = 1 << 5;
const CARRY: u8 = 1 << 4;

pub struct Registers {
  regs: [u8; 8],
}

impl Registers {
  pub fn new() -> Self { Registers { regs: [0; 8] } }
  pub fn get<T>(&self, id: T) -> u8 where T: ToPrimitive { 
    self.regs[id.to_usize().unwrap()] 
  }
  pub fn set<T>(&mut self, id: T, data: u8) where T: ToPrimitive {
    self.regs[id.to_usize().unwrap()] = data;
  }

  pub fn get_zero(&self) -> bool { self.get(F as u8) & ZERO != 0 }
  pub fn get_sub(&self) -> bool { self.get(F as u8) & SUB != 0}
  pub fn get_hcarry(&self) -> bool { self.get(F as u8) & HALF_CARRY != 0}
  pub fn get_carry(&self) -> bool { self.get(F as u8) & CARRY != 0}

  pub fn set_zero(&mut self, cond: bool) { 
    if cond { self.regs[F as usize] |= ZERO; } 
    else { self.regs[F as usize] &= !ZERO }
  }
  pub fn set_sub(&mut self, cond: bool) { 
    if cond { self.regs[F as usize] |= SUB; } 
    else { self.regs[F as usize] &= !SUB }
  }
  pub fn set_hcarry(&mut self, cond: bool) { 
    if cond { self.regs[F as usize] |= HALF_CARRY; } 
    else { self.regs[F as usize] &= !HALF_CARRY }
  }
    pub fn set_carry(&mut self, cond: bool) { 
    if cond { self.regs[F as usize] |= CARRY; } 
    else { self.regs[F as usize] &= !CARRY }
  }

  pub(super) fn update_carry(&mut self, a: u8, b: u8) {
    if let Some(_) = a.checked_add(b) {
      self.set_carry(false);
    } else { self.set_carry(true) }
  }

  pub(super) fn update_carry_16(&mut self, a: u16, b: u16) {
    if let Some(_) = a.checked_add(b) {
      self.set_carry(false);
    } else { self.set_carry(true) }
  }

  pub(super) fn update_carry_3(&mut self, a: u8, b: u8, c: u8) {
    let (r, overflow1) = a.overflowing_add(b);
    let (_, overflow2) = r.overflowing_add(c);

    if overflow1 || overflow2 {
      self.set_carry(true);
    } else { self.set_carry(false); }
  }

  pub(super) fn update_hcarry(&mut self, a: u8, b: u8) {
    // 0xF is bit 3. 0x1 is bit 4 set to 1 
    let hc = (a & 0xf) + (b & 0xf) & 0x10;
    self.set_hcarry(hc == 0x10);
  }

  pub(super) fn update_hcarry_16(&mut self, a: u16, b: u16) {
    let hc = (a & 0xfff) + (b & 0xfff) & 0x1000;
    self.set_hcarry(hc == 0x1000);
  }
  
  pub(super) fn update_hcarry_3(&mut self, a: u8, b: u8, c: u8) {
    let hc = (a & 0xf) + (b & 0xf) + (c & 0xf) & 0x10;
    self.set_hcarry(hc == 0x10);
  }

  pub(super) fn update_zero_and_carries_flags(&mut self, a: u8, b: u8) {
    self.set_zero(a.wrapping_add(b) == 0);
    self.update_carry(a, b);
    self.update_hcarry(a, b);
  }

  pub(super) fn update_zero_and_carries_flags_3(&mut self, a: u8, b: u8, c: u8) {
    self.set_zero(a.wrapping_add(b).wrapping_add(c) == 0);
    self.update_carry_3(a, b, c);
    self.update_hcarry_3(a, b,c );
  }

  pub fn get_16<T>(&self, id: T) -> u16 where T: ToPrimitive{ 
    match id.to_usize().unwrap() {
      0 => self.get_bc(),
      1 => self.get_de(),
      2 => self.get_hl(),
      3 => self.get_af(),
      _ => panic!("Impossible register16 ID.")
    }
  }

  pub fn set_16<T>(&mut self, id: T, data: u16) where T: ToPrimitive { 
    match id.to_usize().unwrap() {
      0 => self.set_bc(data),
      1 => self.set_de(data),
      2 => self.set_hl(data),
      3 => self.set_af(data),
      _ => panic!("Impossible register16 ID.")
    }
  }

  pub fn get_af(&self) -> u16 { u16::from_be_bytes([self.get(A), self.get(F as u8)]) }
  pub fn get_bc(&self) -> u16 { u16::from_be_bytes([self.get(B), self.get(C as u8)]) }
  pub fn get_de(&self) -> u16 { u16::from_be_bytes([self.get(D), self.get(E as u8)]) }
  pub fn get_hl(&self) -> u16 { u16::from_be_bytes([self.get(H), self.get(L as u8)]) }

  pub fn set_af(&mut self, data: u16) { 
    let [high, low] = data.to_be_bytes();
    self.set(A, high);
    self.set(F, low);
  }
  pub fn set_bc(&mut self, data: u16) { 
    let [high, low] = data.to_be_bytes();
    self.set(B, high);
    self.set(C, low);
  }
    pub fn set_de(&mut self, data: u16) { 
    let [high, low] = data.to_be_bytes();
    self.set(D, high);
    self.set(E, low);
  }
    pub fn set_hl(&mut self, data: u16) { 
    let [high, low] = data.to_be_bytes();
    self.set(H, high);
    self.set(L, low);
  }
}