const fs = require('fs');
const data = fs.readFileSync('./inputs', { encoding: 'utf8' }).split('\n');

console.log(data);