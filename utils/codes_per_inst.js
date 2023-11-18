const fs = require('fs');

const jsonFile = fs.readFileSync('opcodes.json');
const jsonData = JSON.parse(jsonFile.toString());

function get_names(data) {
  return [ ... new Set(Object.entries(data).map(([key, value]) => value.mnemonic)) ]
}

function get_codes_for_names(data) {
  let map = new Map()
  let names = get_names(data)
  names.forEach((name) => map.set(name, ''))

  Object.entries(data).forEach(([key, value]) => {
    let codes = map.get(value.mnemonic)
    codes += key + ' | '
    map.set(value.mnemonic, codes)
  })

  return Array.from(map)
}

const unprefixed = get_codes_for_names(jsonData.unprefixed)
const cbprefixed = get_codes_for_names(jsonData.cbprefixed)
let data = {unprefixed: unprefixed, cbprefixed: cbprefixed}
fs.writeFileSync('codes.txt', JSON.stringify(data, null, 2))