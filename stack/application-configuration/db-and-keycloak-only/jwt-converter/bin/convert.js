#! /usr/bin/env node
const jwkToPem = require('jwk-to-pem');

process.stdin.on("data", line => {

    const jwk = JSON.parse(line);

    const pem = jwkToPem(jwk);

    console.log(pem);

})




