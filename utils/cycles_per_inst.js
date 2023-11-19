const jsonFile = fs.readFileSync('opcodes.json');
const jsonData = JSON.parse(jsonFile.toString());

let cycles = Object.entries(jsonData.unprefixed).map(([key, value]) => {
  return {code: key, name: value.mnemonic, cycles: value.cycles}
})

console.log(cycles.filter(i => i.cycles.length > 1));
