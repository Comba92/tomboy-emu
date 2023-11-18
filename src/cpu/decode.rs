use super::{addressing::Opcode, CPU};

impl CPU {
  pub fn decode(&mut self, opcode: &Opcode) {
    let operands = &opcode.operands;

    match opcode.code {
      0x00 => (),
      0x10 => panic!("STOP reached."), // STOP
      0x76 => panic!("HALT reached."), // HALT

      0x01 | 0x02 | 0x06 | 0x08 | 0x0A |
      0x0E | 0x11 | 0x12 | 0x16 | 0x1A |
      0x1E | 0x21 | 0x26 | 0x2E | 0x31 |
      0x36 | 0x3E | 0xEA | 0xFA | 0xF9 | 
      0xE0 | 0xF0 | 0x40 ..= 0x7F
      => self.ld(&operands[0], &operands[1]),

      0xF8 => self.ld_sp_sign(&operands[2]),
      0x22 | 0x2A => self.ldi(&operands[0], &operands[1]),
      0x32 | 0x3A => self.ldd(&operands[0], &operands[1]),
      0xE2 => self.ld_a_to_io_in_c_reg(),
      0xF2 => self.ld_io_in_c_reg_to_a(),

      0x03 | 0x04 | 0x0C | 0x13 | 0x14 | 0x1C | 
      0x23 | 0x24 | 0x2C | 0x33 | 0x34 | 0x3C => self.inc(&operands[0]),

      0x05 | 0x0B | 0x0D | 0x15 | 0x1B | 0x1D |
      0x25 | 0x2B | 0x2D | 0x35 | 0x3B | 0x3D => self.dec(&operands[0]),

      0x80 ..= 0x87 | 0xC6 => self.add(&operands[1]),
      0x09 | 0x19 | 0x29 | 0x39 => self.add_16(&operands[1]),
      0xE8 => self.add_sp_sign(&operands[1]),

      0x18 => self.jr(&operands[0]),
      0x20 | 0x28 | 0x30 | 0x38 => self.jrc(&operands[0], &operands[1]),

      0x07 => self.rlca(),
      0x17 => self.rla(),
      0x0F => self.rrca(), 
      0x1F => self.rra(),

      0x27 => self.daa(),
      0x2F => self.cpl(),
      0x37 => self.scf(),
      0x3F => self.ccf(),

      0x88 ..= 0x8F | 0xCE => self.adc(&operands[1]),
      0x90 ..= 0x97 | 0xD6 => self.sub(&operands[1]),
      0x98 ..= 0x9F | 0xDE => self.sbc(&operands[1]),
      0xA0 ..= 0xA7 | 0xE6 => self.and(&operands[1]),
      
      0xA8 ..= 0xAF | 0xEE => self.xor(&operands[1]),
      0xB0 ..= 0xB7 | 0xF6 => self.or(&operands[1]),
      0xB8 ..= 0xBF | 0xFE => self.cp(&operands[1]),
    

      0xC1 | 0xD1 | 0xE1 | 0xF1 => self.pop(&operands[0]),
      0xC5 | 0xD5 | 0xE5 | 0xF5 => self.push(&operands[0]),

      0xC3 | 0xE9 => self.jp(&operands[0]),
      0xC2  | 0xCA | 0xD2 | 0xDA  => self.jpc(&operands[0], &operands[1]),

      0xCD => self.call(&operands[0]),
      0xC4 | 0xCC | 0xD4 | 0xDC => self.callc(&operands[0], &operands[1]),
      0xC9 => self.ret(),
      0xC0 | 0xC8 | 0xD0 | 0xD8 => self.retc(&operands[0]),

      0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => self.rst(&operands[0]),

      0xD9 => self.reti(),
      
      0xF3 => self.di(),
      0xFB => self.ei(),

      _ => unimplemented!("Unimplemented instruction {:04x}.", opcode.code),
    }
  }
  pub fn cb_decode(&mut self, opcode: &Opcode) {
    let operands = &opcode.operands;

    match opcode.code {
      0x00 ..= 0x07 => self.rlc(&operands[0]),
      0x08 ..= 0x0f => self.rrc(&operands[0]),
      0x10 ..= 0x17 => self.rl(&operands[0]),
      0x18 ..= 0x1f => self.rr(&operands[0]),
      0x20 ..= 0x27 => self.sla(&operands[0]),
      0x28 ..= 0x2f => self.sra(&operands[0]),
      0x30 ..= 0x37 => self.swap(&operands[0]),
      0x38 ..= 0x3f => self.srl(&operands[0]),

      0x40 ..= 0x7f => self.bit(&operands[0], &operands[1]),
      0x80 ..= 0xbf => self.res(&operands[0], &operands[1]),
      0xc0 ..= 0xff => self.set(&operands[0], &operands[1]),
    }
  }
}