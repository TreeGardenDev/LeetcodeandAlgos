//arrays first example
//
//
//array buffer
//
const buffer = new ArrayBuffer(8);
console.log(buffer);

const a8 = new Uint8Array(buffer);
a8[0] = 45;
console.log(a8);

a8[2] = 45;

const a16 = new Uint16Array(buffer);

a16[2]=0x4545;

console.log(a16);


