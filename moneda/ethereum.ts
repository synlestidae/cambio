import { BlockTransaction } from './block_transaction';

export class EthereumService {
    private web3: any;

    constructor(web3: any) {
        if (!web3) {
            throw new Error('Cannot create EthereumService without web3 instance');
        }
        this.web3 = web3;
    }

    async sendTransaction(ethTransaction: any): Promise<string> {
        let web3 = this.web3;
        return new Promise(async function (res: (r: string) => void, rej: (r: any) => void) {
            let rawTx: string = ethTransaction.serialize().toString('hex');
            try {
                web3.eth.sendRawTransaction('0x' + rawTx, function(err: any, hash: string) {
                    if (err) {
                        rej(err);
                    }
                    res(hash);
                });
            } catch (e) {
                rej(e);
            }
        });
    }

    async getBlockTransaction(ethTransaction: any): Promise<BlockTransaction> {
        let tx: any = await this.getTransaction(ethTransaction.hash);
        let blockNumber: number = await this.getBlockNumber();
        let confirmations: number = blockNumber - tx.blockNumber;
        return new BlockTransaction(tx, blockNumber);
    }

    async getTransaction(hash: string): Promise<any> {
        return new Promise(function(res, rej) {
            this.web3.getTransaction(hash, function(err: any, hash: string) {
                if (err) {
                    rej(err);
                }
                res(hash);
            });
        });
    }

    getBlockNumber(): Promise<number> {
        return new Promise(function(res, rej) {
            this.web3.eth.getBlockNumber(function(err: any, blockNum: number) {
                if (err) {
                    rej(err);
                }
                res(blockNum);
            });
        });
    }
}
