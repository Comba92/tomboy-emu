const fs = require('fs');

const jsonFile = fs.readFileSync('opcodes.json');
const jsonData = JSON.parse(jsonFile.toString());

const conds = ["CY", "NC", "Z", "NZ"]
const bits = ["0", "1" , "2" , "3" , "4" , "5" , "6" , "7"]
const literals = ["n8", "n16", "a8", "a16", "e8"]

function parse_operand(data) {
  let type = '';

  if (data.name[0] == '$' || !isNaN(Number(data.name[0]))) { type = 'Literal(n8)' }
  else if (conds.includes(data.name)) { type = 'Condition(' + data.name + ')'}
  else if (bits.includes(data.name)) { type = 'Literal(n8)' }
  else if (literals.includes(data.name)) { type = 'Literal(' + data.name + ')' }
  else { type = 'Register(' + data.name + ')'}

  return `Operand {kind: ${type}, immediate: ${data.immediate}}`
}

function parse_obj(data) {
  return Object.entries(data).map( ([key, value]) => {
    if (["0xDC", "0xD8", "0xDA"].includes(key)) {
      value.operands[0].name = "CY"
    }

    let cycles = isNaN(value.cycles) ? 0 : value.cycles
    let operands = value.operands.map(e => parse_operand(e)).join(', ')
    let code = parseInt(key, 16)

    return `\tOpcode {code: ${code}, name: "${value.mnemonic}", bytes: ${value.bytes}, cycles: ${cycles}, immediate: ${value.immediate}, operands: vec!(${operands})}`
  }).join(', \n')
}

let unprefixed = parse_obj(jsonData.unprefixed)
let cbprefixed = parse_obj(jsonData.cbprefixed)
let header = `use super::addressing::*;
use super::addressing::OperandType::*;
use super::addressing::ConditionOperand::*;
use super::addressing::RegisterOperand::*;
use super::addressing::LiteralOperand::*;\n\n`


fs.writeFileSync('optable.txt', header + 'const opcodes: Vec<Opcode> = vec![\n' + unprefixed + ',\n\n' + cbprefixed + '\n];')

