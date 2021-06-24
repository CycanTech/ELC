import { expect } from 'chai';
import { patract, network, artifacts} from 'redspot';
import { Config } from './pub.parameter.ts';
var cfg = new Config();

const { getContractFactory,  getContractAt} = patract;
const { createSigner, keyring, api } = network;

describe('ELP', () => {
    after(() => {
        return api.disconnect();
    });

    async function setup() {
        await api.isReady;
        const signerA = createSigner(keyring.createFromUri(cfg.uriAlice));
        const signerB = createSigner(keyring.createFromUri(cfg.uriBob));
        const signerC = createSigner(keyring.createFromUri(cfg.uriCharlie));
        const signerD = createSigner(keyring.createFromUri(cfg.uriDave));
        const signerE = createSigner(keyring.createFromUri(cfg.uriEve));
        const signerF = createSigner(keyring.createFromUri(cfg.uriFerdie));

        const elc       = api.createType('AccountId', cfg.elcAddress);
        const oracle    = api.createType('AccountId', cfg.oracleAddress);
        const relp      = api.createType('AccountId', cfg.relpAddress);
        const pool      = api.createType('AccountId', cfg.poolAddress);
        const exchange2 = api.createType('AccountId', cfg.exchange2Address);

        const elcContract = await getContractAt('elc', elc, signerA);
        const relpContract = await getContractAt('relp', relp, signerA);
        const oracleContract = await getContractAt('oracle', oracle, signerA);
        const poolContract = await getContractAt('pool', pool, signerA);
        const exchangeContract = await getContractAt('exchange2', exchange2, signerA);

        return { elcContract, relpContract, oracleContract, poolContract, exchangeContract, signerA,  signerB, signerC, signerD, signerE, signerF};
    }

    it('contract_elc', async () => {
        const { elcContract, relpContract, poolContract, signerA, signerB } = await setup();

        const relp_totalSupply0  = await relpContract.query.totalSupply();
        console.log('relp_totalSupply0 : ', relp_totalSupply0.output);
        const elc_totalSupply0  = await elcContract.query.totalSupply();
        console.log('elc_totalSupply0  : ', elc_totalSupply0.output);

        // const poollr  = await poolContract.query.liabilityRatio();
        // console.log('liability_ratio  : ', poollr.output);

        // const result_r  = await poolContract.tx.contractElc( {
        //     gasLimit: '600000000000',
        //     value: '5000000000000',
        //     signer: signerB
        // });

        const relp_totalSupply1  = await relpContract.query.totalSupply();
        console.log('relp_totalSupply1 : ', relp_totalSupply1.output);
        const elc_totalSupply1  = await elcContract.query.totalSupply();
        console.log('elc_totalSupply1  : ', elc_totalSupply1.output);

        // expect(relp_totalSupply1.output).to.equal(500000000000);
    });

    it('contract_elc result', async () => {
        const { elcContract, relpContract, poolContract, signerA, signerB } = await setup();

        const relp_totalSupply1  = await relpContract.query.totalSupply();
        console.log('relp_totalSupply1 : ', relp_totalSupply1.output);
        const relpBalanceAlice = await relpContract.query.balanceOf(signerA.address);
        console.log('relpBalanceAlice : ', relpBalanceAlice.output);
        const relpBalanceBob = await relpContract.query.balanceOf(signerB.address);
        console.log('relpBalanceBob : ', relpBalanceBob.output);

        const elc_totalSupply1  = await elcContract.query.totalSupply();
        console.log('elc_totalSupply1  : ', elc_totalSupply1.output);
        const elcBalanceAlice = await elcContract.query.balanceOf(signerA.address);
        console.log('elcBalanceAlice  : ', elcBalanceAlice.output);
        const elcBalanceBob = await elcContract.query.balanceOf(signerB.address);
        console.log('elcBalanceBob  : ', elcBalanceBob.output);

        // const poollr  = await poolContract.query.liabilityRatio();
        // console.log('liability_ratio  : ', poollr.output);

        // expect(relpBalanceBob.output).to.equal(500000000000);
    });
});
