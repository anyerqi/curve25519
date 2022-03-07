import test from 'ava'


import {generateKeyPair, hello, calculateSignature, verifySignature, createKeyPair, validatePubKeyFormat} from "../index"

const pubKeyBuffer = Buffer.from('BVZkUqe13emSJm2eJFg1w0fX4Uhdsw0jqJV0bzgbS8Jg', 'base64');
const messageBuffer = Buffer.from('uX9oWF6BHmLEtJrvYUyG5GGnZL8raP5s+nwQ4aKtO2c=', 'base64');
const signatureBuffer = Buffer.from('FzX9xkLvUWsc8ochqvNl9CAoKmnAPGblAOsZRYA8i5bn1sjkvRfLd58jvnrnfDxqGTFb9g/JdvS+/RPd0vMvBw==', 'base64');

test("verify signature", (t) => {
    t.is(verifySignature(pubKeyBuffer, messageBuffer, signatureBuffer), false);
})