use std::collections::HashMap;
use std::hash::Hash;
use serde::{Serialize, Deserialize};
use serde_json;
use lazy_static::lazy_static;
use super::operands::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Optable<K, V> where K: Hash + Eq {
  pub unprefixed: HashMap<K, V>,
  pub cbprefixed: HashMap<K, V>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Opcode<T> {
  pub mnemonic: &'static str,
  pub bytes: u8,
  pub cycles: u8,
  pub operands: Vec<T>,
}

fn map_str_to_operand(op: &str) -> OperandsType {
  match op {
    "A" => OperandsType::Register(RegisterOperand::A),
    "B" => OperandsType::Register(RegisterOperand::B),
    "C" => OperandsType::Register(RegisterOperand::C),
    "D" => OperandsType::Register(RegisterOperand::D),
    "E" => OperandsType::Register(RegisterOperand::E),
    "F" => OperandsType::Register(RegisterOperand::F),
    "H" => OperandsType::Register(RegisterOperand::H),
    "L" => OperandsType::Register(RegisterOperand::L),
    "AF" => OperandsType::Register(RegisterOperand::AF),
    "BC" => OperandsType::Register(RegisterOperand::BC),
    "DE" => OperandsType::Register(RegisterOperand::DE),
    "HL" => OperandsType::Register(RegisterOperand::HL),
    "Z"     => OperandsType::Condition(ConditionOperand::Z),
    "NZ"    => OperandsType::Condition(ConditionOperand::NZ),
    "CARRY" => OperandsType::Condition(ConditionOperand::C),
    "NC"    => OperandsType::Condition(ConditionOperand::NC),
    "n8"  => OperandsType::Constant(ConstantOperand::Data8),
    "n16" => OperandsType::Constant(ConstantOperand::Data16),
    "a8"  => OperandsType::Constant(ConstantOperand::AddressIO),
    "a16" => OperandsType::Constant(ConstantOperand::Address),
    "e8"  => OperandsType::Constant(ConstantOperand::DataSigned8),
    "1" | "2" | "3" | "4" | "5" | "6" | "7" => 
      OperandsType::Constant(ConstantOperand::Bit),
    _ => OperandsType::Constant(ConstantOperand::Vector),
  }
}

fn from_json_to_rust_table(table: HashMap<String, Opcode<String>>) -> HashMap<u8, Opcode<OperandsType>> {
    let mut new_table = HashMap::new();
    
    for (key, value) in table {
      let opcode = Opcode {
        mnemonic: value.mnemonic,
        bytes: value.bytes,
        cycles: value.cycles,
        operands: value.operands.iter()
          .map(|o| map_str_to_operand(o.as_str())).collect()
      };

      new_table.insert(u8::from_str_radix(&key[2..], 16).unwrap(), opcode);
    }

    new_table
}

lazy_static! {
  pub static ref OPTABLE: Optable<u8, Opcode<OperandsType>> = {
    let json_str = include_str!("optable.json");
    let json_optable: Optable<String, Opcode<String>> = serde_json::from_str(json_str).unwrap();
    
    let unprefixed = from_json_to_rust_table(json_optable.unprefixed);
    let cbprefixed = from_json_to_rust_table(json_optable.cbprefixed);

    Optable { unprefixed, cbprefixed }
  };
}
