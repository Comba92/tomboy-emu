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

#[derive(Serialize, Deserialize, Debug)]
struct OperandJSON {
  name: String, immediate: bool
}

fn map_str_to_operand(op: &OperandJSON) -> OperandType {
  let source = match op.name.as_str() {
    "A" => SourceType::Register(RegisterOperand::A),
    "B" => SourceType::Register(RegisterOperand::B),
    "C" => SourceType::Register(RegisterOperand::C),
    "D" => SourceType::Register(RegisterOperand::D),
    "E" => SourceType::Register(RegisterOperand::E),
    "F" => SourceType::Register(RegisterOperand::F),
    "H" => SourceType::Register(RegisterOperand::H),
    "L" => SourceType::Register(RegisterOperand::L),
    "AF" => SourceType::Register(RegisterOperand::AF),
    "BC" => SourceType::Register(RegisterOperand::BC),
    "DE" => SourceType::Register(RegisterOperand::DE),
    "HL" => SourceType::Register(RegisterOperand::HL),
    "Z"     => SourceType::Condition(ConditionOperand::Z),
    "NZ"    => SourceType::Condition(ConditionOperand::NZ),
    "CARRY" => SourceType::Condition(ConditionOperand::C),
    "NC"    => SourceType::Condition(ConditionOperand::NC),
    "n8"  => SourceType::Literal(LiteralOperand::Data8),
    "n16" => SourceType::Literal(LiteralOperand::Data16),
    "a8"  => SourceType::Literal(LiteralOperand::AddressIO),
    "a16" => SourceType::Literal(LiteralOperand::Address),
    "e8"  => SourceType::Literal(LiteralOperand::DataSigned8),
    "1" | "2" | "3" | "4" | "5" | "6" | "7" => 
      SourceType::Literal(LiteralOperand::Bit),
    _ => SourceType::Literal(LiteralOperand::Vector),
    };

    if op.immediate { OperandType::Immediate(source) } else { OperandType::Direct(source) }
}

fn from_json_to_rust_table(table: HashMap<String, Opcode<OperandJSON>>) -> HashMap<u8, Opcode<OperandType>> {
    let mut new_table = HashMap::new();
    
    for (key, value) in table {
      let opcode = Opcode {
        mnemonic: value.mnemonic,
        bytes: value.bytes,
        cycles: value.cycles,
        operands: value.operands.iter()
          .map(|o| map_str_to_operand(&o)).collect()
      };

      new_table.insert(u8::from_str_radix(&key[2..], 16).unwrap(), opcode);
    }

    new_table
}

lazy_static! {
  pub static ref OPTABLE: Optable<u8, Opcode<OperandType>> = {
    let json_str = include_str!("optable.json");
    let json_optable: Optable<String, Opcode<OperandJSON>> = serde_json::from_str(json_str).unwrap();
    
    let unprefixed = from_json_to_rust_table(json_optable.unprefixed);
    let cbprefixed = from_json_to_rust_table(json_optable.cbprefixed);

    Optable { unprefixed, cbprefixed }
  };
}
