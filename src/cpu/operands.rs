use OperandsType::*;
use super::CPU;
use super::Flags;

#[derive(Debug)]
pub enum OperandsType {
  Register(RegisterOperand),
  Condition(ConditionOperand),
  Constant(ConstantOperand)
}

#[derive(Debug)]
pub enum RegisterOperand {
  A, B, C, D, E, F, H, L, AF, BC, DE, HL
}

#[derive(Debug)]
pub enum ConditionOperand {
  Z, NZ, C, NC
}

#[derive(Debug)]
pub enum ConstantOperand {
  Data8,
  Data16,
  DataSigned8,
  Address,
  AddressIO,
  Bit,
  Vector
}


// TODO: get should return an u8 or a u16, and set should set an u8 or an u16, figure out how
impl CPU {
  pub(super) fn get_operand(&self, operand: &OperandsType) -> u16 {
    match operand {
      Register(reg) => match reg {
        RegisterOperand::A  => self.reg_a as u16,
        RegisterOperand::B  => self.reg_b as u16,
        RegisterOperand::C  => self.reg_c as u16,
        RegisterOperand::D  => self.reg_d as u16,
        RegisterOperand::E  => self.reg_d as u16,
        RegisterOperand::F  => self.reg_f.bits() as u16,
        RegisterOperand::H  => self.reg_h as u16,
        RegisterOperand::L  => self.reg_l as u16,
        RegisterOperand::AF => self.reg_af(),
        RegisterOperand::BC => self.reg_bc(),
        RegisterOperand::DE => self.reg_de(),
        RegisterOperand::HL => self.reg_hl(),
      },

      Condition(cond) => match cond {
        ConditionOperand::Z  => self.reg_f.contains(Flags::ZERO) as u16,
        ConditionOperand::NZ => (!self.reg_f.contains(Flags::ZERO)) as u16,
        ConditionOperand::C  => self.reg_f.contains(Flags::CARRY) as u16,
        ConditionOperand::NC => (!self.reg_f.contains(Flags::CARRY)) as u16,
      },

      Constant(c) => match c {
        ConstantOperand::Data8       => self.mem_read(self.pc) as u16,
        ConstantOperand::Data16      => self.mem_read16(self.pc),
        ConstantOperand::AddressIO   => todo!(),
        ConstantOperand::Address     => self.mem_read16(self.pc),
        ConstantOperand::DataSigned8 => todo!(),
        ConstantOperand::Bit         => todo!(),
        ConstantOperand::Vector      => todo!(),
      }
    }
  }

  pub(super) fn set_operand(&mut self, operand: &OperandsType, data: u16) {
    match operand {
      Register(reg) => match reg {
        RegisterOperand::A  => self.reg_a = data as u8,
        RegisterOperand::B  => self.reg_b = data as u8,
        RegisterOperand::C  => self.reg_c = data as u8,
        RegisterOperand::D  => self.reg_d = data as u8,
        RegisterOperand::E  => self.reg_d = data as u8,
        RegisterOperand::F  => 
          self.reg_f = Flags::from_bits_truncate(data as u8),
        RegisterOperand::H  => self.reg_h = data as u8,
        RegisterOperand::L  => self.reg_l = data as u8,
        RegisterOperand::AF => self.set_reg_af(data),
        RegisterOperand::BC => self.set_reg_bc(data),
        RegisterOperand::DE => self.set_reg_de(data),
        RegisterOperand::HL => self.set_reg_hl(data),
      },

      Constant(c) => match c {
        ConstantOperand::AddressIO   => todo!(),
        ConstantOperand::Address     => {
          let addr = self.mem_read16(self.pc);
          self.mem_write(addr, data as u8);
        },

        _ => panic!("{:#?} operand setting not implemented.", operand)
      },

      _ => panic!("{:#?} operand setting not implemented.", operand)
    }
  }
}