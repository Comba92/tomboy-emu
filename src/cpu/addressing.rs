use super::{CPU, Flags};

#[derive(Debug, Clone)]
pub struct Opcode {
  pub code: u8,
  pub name: &'static str,
  pub bytes: u8,
  pub cycles: u8,
  pub immediate: bool,
  pub operands: Vec<Operand>,
}

#[derive(Debug, Clone, Copy)]
pub struct Operand {
  pub kind: OperandType,
  pub immediate: bool
}

#[derive(Debug, Clone, Copy)]
pub enum OperandType {  
  Register(RegisterOperand),
  Condition(ConditionOperand),
  Literal(LiteralOperand),
  Interrupt(u8),
  Bit(u8)
}

impl Operand {
  pub fn is_value_16(&self) -> bool {
    // this is awlays 8bit
    if !self.immediate { return false }

    match self.kind {
      OperandType::Register(reg) => {
        match reg {
          RegisterOperand::AF | RegisterOperand::BC | RegisterOperand::DE |
          RegisterOperand::HL | RegisterOperand::SP => true,
          _ => false,
        }
      },
      OperandType::Literal(lit) => {
        match lit {
          LiteralOperand::a16 | LiteralOperand::n16 => true,
          _ => false,
        }
      },
      _ => false,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterOperand { A, B, C, D, E, F, H, L, AF, BC, DE, HL, SP }

#[derive(Debug, Clone, Copy)]
pub enum ConditionOperand { Z, NZ, CY, NC }

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum LiteralOperand { n8, n16, a8, a16, e8 }

impl CPU {
  pub(super) fn get_from_source(&self, src: &Operand) -> u16 {
    let data_to_get = match src.kind {
      OperandType::Register(reg) => {
        match reg {
          RegisterOperand::A => self.a as u16,
          RegisterOperand::B => self.b as u16,
          RegisterOperand::C => self.c as u16,
          RegisterOperand::D => self.d as u16,
          RegisterOperand::E => self.e as u16,
          RegisterOperand::F => self.f.bits() as u16,
          RegisterOperand::H => self.h as u16,
          RegisterOperand::L => self.l as u16,
          
          RegisterOperand::AF => self.get_af(),
          RegisterOperand::BC => self.get_bc(),
          RegisterOperand::DE => self.get_de(),
          RegisterOperand::HL => self.get_hl(),
          RegisterOperand::SP => self.sp,
        }
      },

      OperandType::Condition(cond) => {
        match cond {
          ConditionOperand::CY => self.f.contains(Flags::CARRY) as u16,
          ConditionOperand::NC => !self.f.contains(Flags::CARRY) as u16,
          ConditionOperand::Z  => self.f.contains(Flags::ZERO) as u16,
          ConditionOperand::NZ => !self.f.contains(Flags::ZERO) as u16,
        }
      },

      OperandType::Literal(lit) => {
        // when computing the instruction, the pc is pointing to the next instruction.
        // we have to decrease it to read the operand.
        match lit {
          LiteralOperand::n8 | LiteralOperand::e8 => 
            self.mem_read(self.pc.wrapping_sub(1)) as u16,

          LiteralOperand::a8 => 
            0xff00 + self.mem_read(self.pc.wrapping_sub(1)) as u16, 

          LiteralOperand::n16 | LiteralOperand::a16 => 
            self.mem_read_16(self.pc.wrapping_sub(2)),
        }
      },

      OperandType::Interrupt(int) => int as u16,
      OperandType::Bit(bit) => bit as u16
    };

    if src.immediate { data_to_get }
    else { self.mem_read(data_to_get) as u16 }
  }

  pub(super) fn set_to_destination(&mut self, dst: &Operand, data: u8) {
    if dst.immediate { self.set_to_destination_direct(dst, data); }
    else { self.set_to_destination_indirect(dst, data); }
  }

  fn set_to_destination_direct(&mut self, dst: &Operand, data: u8) {
    match dst.kind {
      OperandType::Register(reg) => {
        match reg {
          RegisterOperand::A => self.a = data as u8,
          RegisterOperand::B => self.b = data as u8,
          RegisterOperand::C => self.c = data as u8,
          RegisterOperand::D => self.d = data as u8,
          RegisterOperand::E => self.e = data as u8,
          RegisterOperand::F => self.f = Flags::from_bits_truncate(data as u8),
          RegisterOperand::H => self.h = data as u8,
          RegisterOperand::L => self.l = data as u8,
          _ => panic!("Impossible to set 8bit literal value in 16bit register.")
        }
      },

      OperandType::Literal(lit) => {
        // when computing the instruction, the pc is pointing to the next instruction.
        // we have to decrease it to read the operand.
        match lit {
          LiteralOperand::a16 => {
            let addr = self.mem_read_16(self.pc.wrapping_sub(2));
            self.mem_write(addr, data as u8);
          }
          _ => panic!("Impossible to address 8bit literal value.")
        }
      },

      _ => panic!("Impossible destination to set.")
    };
  }

  fn set_to_destination_indirect(&mut self, dst: &Operand, data: u8) {
    let addr = match dst.kind {
      OperandType::Register(reg) => {
        match reg {
          RegisterOperand::AF => self.get_af(),
          RegisterOperand::BC => self.get_bc(),
          RegisterOperand::DE => self.get_de(),
          RegisterOperand::HL => self.get_hl(),
          _ => panic!("Impossible to address 8bit register.")
        }
      },

      OperandType::Literal(lit) => {
        // when computing the instruction, the pc is pointing to the next instruction.
        // we have to decrease it to read the operand.
        match lit {
          LiteralOperand::a8 => 0xff00 + self.mem_read(self.pc.wrapping_sub(1)) as u16,
          LiteralOperand::a16 => self.mem_read_16(self.pc.wrapping_sub(2)),
          
          _ => panic!("Impossible to address 8bit literal value.")
        } 
      },

      _ => panic!("Impossible destination to set.")
    };

    self.mem_write(addr, data);
  }

  pub(super) fn set_to_destination_16(&mut self, dst: &Operand, data: u16) {
    match dst.kind {
      OperandType::Register(reg) => {
        match reg {
          RegisterOperand::AF => self.set_af(data),
          RegisterOperand::BC => self.set_bc(data),
          RegisterOperand::DE => self.set_de(data),
          RegisterOperand::HL => self.set_hl(data),
          RegisterOperand::SP => self.sp = data,
          _ => panic!("Impossible to set 16bit literal value in 8bit register.")
        }
      },

      OperandType::Literal(lit) => {
        // when computing the instruction, the pc is pointing to the next instruction.
        // we have to decrease it to read the operand.
        match lit {
          LiteralOperand::a16 => {
            let addr = self.mem_read_16(self.pc.wrapping_sub(2));
            self.mem_write_16(addr, data);
          },
          _ => panic!("Impossible to address 8bit literal value.")
        }
      },

      _ => panic!("Impossible destination to set.")
    }
  }
}