var web3 = require('web3');

var address = '0xf15090c01bec877a122b567e5552504e5fd22b79';
var account = '0x9f23fedfa2ce3a321f20f6a95d0c2cbabb5876dd';
var privateKey = '0x77173c4b349c6342ae695f86c5610688606de77361769bd8919301fc55823f1b';

var instance = new web3.Web3(new web3.Web3.providers.HttpProvider('http://localhost:8080'));
console.log('Downloading', account);
instance.eth.getTransactionCount(account, function (err, nonce) {
    console.log('Tranny', err, nonce);
    var data = instance.eth.contract(abi).at(address).increment.getData();
    var tx = new web3.ethereumjs.Tx({
        //nonce: nonce,
        gasPrice: web3.toHex(web3.toWei('20', 'gwei')),
        gasLimit: 210000,
        to: address,
        value: web3.toHex(web3.toWei('0.1', 'eth')),
        data: '0x0',
    });
    tx.sign(ethereumjs.Buffer.Buffer.from(privateKey, 'hex'));
    var raw = '0x' + tx.serialize().toString('hex');
    instance.eth.sendRawTransaction(raw, function (err, transactionHash) {
      console.log(transactionHash);
    });
});
