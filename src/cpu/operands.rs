use OperandsType::*;
use crate::cpu::CPU;
use crate::cpu::optable::{OperandsType, RegisterOperand};

use super::Flags;
use super::optable::{ConditionOperand, ConstantOperand};

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
        ConstantOperand::Data8       => todo!(),
        ConstantOperand::Data16      => todo!(),
        ConstantOperand::AddressIO   => todo!(),
        ConstantOperand::Address     => todo!(),
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
        ConstantOperand::Address     => todo!(),

        _ => panic!("{:#?} operand setting not implemented.", operand)
      },

      _ => panic!("{:#?} operand setting not implemented.", operand)
    }
  }
}