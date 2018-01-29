const http = require('http');
const web3 = require('web3');
const web3 = require('web3');
const tx = require('ethereumjs-tx')

const PROVIDER_ADDRESS = "http://localhost:8545";

if (typeof web3 !== 'undefined') {
    web3 = new Web3(web3.currentProvider);
} else {
    // set the provider you want from Web3.providers
    web3 = new Web3(new Web3.providers.HttpProvider(PROVIDER_ADDRESS));
}

function handleRequest(req, res) {
      
}
http.createServer(handleRequest).listen(8080); //the server object listens on port 8080
