const {generateKeyPair, hello, calculateSignature, verifySignature, createKeyPair, validatePubKeyFormat} = require("../index");

console.log(hello());

const { privKey, pubKey } = generateKeyPair();
console.log(privKey, pubKey);