const fs = require('fs');

const jsonFile = fs.readFileSync('opcodes.json');
const jsonData = JSON.parse(jsonFile.toString());

const conds = ["CY", "NC", "Z", "NZ"]
const bits = ["0", "1" , "2" , "3" , "4" , "5" , "6" , "7"]
const literals = ["n8", "n16", "a8", "a16", "e8"]

const jumps = [
  "0x18", "0x20", "0x28", "0x30", "0x38", "0xC2", "0xC3", "0xCA", "0xD2", "0xDA", "0xE9",
  "0xC4", "0xCC", "0xCD", "0xD4", "0xDC", "0xC0", "0xC8", "0xC9", "0xD0", "0xD8"
]

function parse_operand(data) {
  let type = '';

  if (data.name[0] === '$') {
    let name = data.name.replace("$", '')
    type = `Interrupt(0x${name})`
  }

  else if (!isNaN(data.name)) {
    type = `Bit(${data.name})` 
  }

  else if (conds.includes(data.name)) { type = 'Condition(' + data.name + ')'}
  else if (bits.includes(data.name)) { type = 'Literal(n8)' }
  else if (literals.includes(data.name)) { type = 'Literal(' + data.name + ')' }
  else { type = 'Register(' + data.name + ')'}
  
  return `Operand {kind: ${type}, immediate: ${data.immediate}}`
}

function parse_obj(data, prefixed) {
  return Object.entries(data).map( ([key, value]) => {
    if (jumps.includes(key)) {
      if (value?.operands[0]?.name === "C")
        value.operands[0].name = "CY"
    }

    let cycles = `(${value.cycles[0]}, ${value.cycles[1] ?? 0})`
    let operands = value.operands.map(e => parse_operand(e)).join(', ')
    let code = parseInt(key, 16)

    return `\tOpcode {code: ${key}, prefixed: ${prefixed}, name: "${value.mnemonic}", bytes: ${value.bytes}, cycles: ${cycles}, immediate: ${value.immediate}, operands: vec!(${operands})}`
  }).join(', \n')
}

let unprefixed = parse_obj(jsonData.unprefixed, false)
let cbprefixed = parse_obj(jsonData.cbprefixed, true)
let header = `use super::addressing::*;
use super::addressing::OperandType::*;
use super::addressing::ConditionOperand::*;
use super::addressing::RegisterOperand::*;
use super::addressing::LiteralOperand::*;\n\n`


fs.writeFileSync('optable.txt', 'const opcodes: Vec<Opcode> = vec![\n' + unprefixed + ',\n\n' + cbprefixed + '\n];')

