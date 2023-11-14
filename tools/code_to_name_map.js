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

  return map
}

const data = get_codes_for_names(jsonData.unprefixed)
console.log(get_names(jsonData.unprefixed))
fs.writeFileSync('codes.txt', JSON.stringify(Array.from(data), null, 2))