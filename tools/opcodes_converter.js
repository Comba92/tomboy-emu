const fs = require('fs');

const jsonFile = fs.readFileSync('opcodes.json');
const jsonData = JSON.parse(jsonFile.toString());

let one = Object.assign({}, ...Object.entries(jsonData.unprefixed).map(([key, value]) => {
  if ( ["0xDC", "0xD8", "0xDA"].includes(key) ) {
    value.operands[0].name = "CARRY"
  }
  
  return { [`${key}`]: {
    mnemonic: value.mnemonic,
    bytes: value.bytes,
    cycles: value.cycles[0],
    operands: value.operands.map(op => op.name)
  }}
}))

let two = Object.assign({}, ...Object.entries(jsonData.cbprefixed).map(([key, value]) => {
  if (["0xDC", "0xD8", "0xDA"].includes(key)) {
    value.operands[0].name = "CARRY"
  }

  return { [`${key}`]: {
    mnemonic: value.mnemonic,
    bytes: value.bytes,
    cycles: value.cycles[0],
    operands: value.operands.map(op => op.name)
  }}
}))

fs.writeFileSync('../src/cpu/optable.json', JSON.stringify({ unprefixed: one, cbprefixed: two}, null, 2));

