use super::CPU;

impl CPU {
  pub(super) fn decode(&mut self, code: u8) -> Result<(), &'static str> {
    // https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html
    // x: bits 7, 6, y: bits 5, 4, 3, z: bits: 2, 1, 0
    let x = (code & 0b1100_0000) >> 5;
    let y = (code & 0b0011_1000) >> 3;
    let z = code & 0b0000_0111;
    // let q = (code & 0b0000_1000) >> 3;
    let p = (code & 0b0011_0000) >> 4;

    match (x, y, z) {
      (0, 0, 0) => return Err("NOP"),
      (0, 1, 0) => self.load_sp_to_mem(),
      (0, 2, 0) => todo!("stop"),
      (0, 3, 0) => todo!("jr d"),
      (0, 4..=7, 0) => todo!("jr cc d"),

      (0, 0, 1) => self.load_immediate_16_to_rr(p),
      (0, 2, 1) => self.load_immediate_16_to_rr(p),
      (0, 4, 1) => self.load_immediate_16_to_rr(p),
      (0, 6, 1) => self.load_immediate_16_to_rr(p),

      (0, 1, 1) => self.add_rr_to_hl(p),
      (0, 3, 1) => self.add_rr_to_hl(p),
      (0, 5, 1) => self.add_rr_to_hl(p),
      (0, 7, 1) => self.add_rr_to_hl(p),

      (0, 0..=3, 2) => self.load_rr_indirect_to_a(y),
      (0, 4..=7, 2) => self.load_a_to_indirect_rr(y - 4),

      (0, 0, 3) => self.inc_rr(p),
      (0, 2, 3) => self.inc_rr(p),
      (0, 4, 3) => self.inc_rr(p),
      (0, 6, 3) => self.sp = self.sp.wrapping_add(1),

      (0, 1, 3) => self.dec_rr(p),
      (0, 3, 3) => self.dec_rr(p),
      (0, 5, 3) => self.dec_rr(p),
      (0, 7, 3) => self.sp = self.sp.wrapping_sub(1),

      (0, _, 4) => self.inc_r(y),
      (0, _, 5) => self.dec_r(y),
      (0, _, 6) => self.load_immediate_to_r(y),

/*       (0, 0, 7) => todo!("whole z=7 block"),
      (0, 1, 7) => 
      (0, 2, 7) =>
      (0, 3, 7) =>
      (0, 4, 7) =>
      (0, 5, 7) =>
      (0, 6, 7) => self.registers.set_carry(true),
      (0, 7, 7) => self.registers.set_carry(!self.registers.carry()) 
      
      */

      (1, 6, 6) => todo!("halt"),                              
      (1, _, 6) => self.load_hl_indirect_to_r(y),
      (1, 6, _) => self.load_r_to_hl_indirect(z),
      (1, _, _) => self.load_r_to_r(y, z),

      
      (2, 0, _) => self.add(z),
      (2, 1, _) => self.adc(z),
      (2, 2, _) => self.sub(z),
      (2, 3, _) => self.sbc(z),
      (2, 4, _) => self.and(z),
      (2, 5, _) => self.xor(z),
      (2, 6, _) => self.or(z),
      (2, 7, _) => self.cp(z), 

      /* 
      (3, 0..=3, 0) => self.ret(y),
      (3, 4, 0)     => todo!("mem mapper reg load"),
      (3, 5, 0)     => todo!("add sp"),
      (3, 6, 0)     => todo!("ld a"),
      (3, 7, 0)     => todo!("ld hl, sp+d"),

      (3, 0, 1) => self.pop(p),
      (3, 2, 1) => self.pop(p)
      (3, 4, 1) => self.pop(p),
      (3, 6, 1) => self.pop(p),

      (3, 1, 1) => self.ret(),
      (3, 3, 1) => self.reti(),
      (3, 5, 1) => self.jp hl,
      (3, 7, 1) => self.ld sp hl,

      (3, 0..=3, 2) => self.jp(y),
      (3, 4, 2)     => todo("ld bla bla bla"),
      (3, 5, 2)     => 
      (3, 6, 2)     =>
      (3, 7, 2)     =>

      (3, 0, 3) => jp,
      (3, 1, 3) => ???,
      (3, 6, 3) => self.or(z),
      (3, 7, 3) => self.cp(z), 

      (3, 0..=3, 4) => self.call(y)
      
      (3, 0, 5) => self.push(p),
      (3, 2, 5) => self.push(p),
      (3, 4, 5) => self.push(p),
      (3, 6, 5) => self.push(p),

      (3, 1, 5) => self.call()


      (3, _, 6) => self.add(y),
      (3, _, 6) => self.adc(y),
      (3, _, 6) => self.sub(y),
      (3, _, 6) => self.sbc(y),
      (3, _, 6) => self.and(y),
      (3, _, 6) => self.xor(y),
      (3, _, 6) => self.or(y),
      (3, _, 6) => self.cp(y), 

      (3, _, 7) => self.rst(y),
      */
      _ => panic!("Opcode {:#?} not implemented.", code)
    }

    Ok(())
  }
}