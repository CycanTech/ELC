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

  it('all state', async () => {
    const { elcContract, relpContract, poolContract, oracleContract, signerA, signerB } = await setup();

    const relpPrice  = await poolContract.query.relpPrice();
    console.log('relpPrice  : ', relpPrice.output);

    const elpPrice  = await oracleContract.query.elpPrice();
    console.log('elpPrice  : ', elpPrice.output);

    const elcPrice  = await oracleContract.query.elcPrice();
    console.log('elcPrice  : ', elcPrice.output);
    
    const elpReserve  = await poolContract.query.elpReserve();
    console.log('elpReserve  : ', elpReserve.output);

    const elpRiskReserve  = await poolContract.query.elpRiskReserve();
    console.log('elpRiskReserve  : ', elpRiskReserve.output);

    const relp_totalSupply  = await relpContract.query.totalSupply();
    console.log('relp_totalSupply : ', relp_totalSupply.output);

    const relpBalanceAlice = await relpContract.query.balanceOf(signerA.address);
    console.log('relpBalanceAlice : ', relpBalanceAlice.output);

    const relpBalanceBob = await relpContract.query.balanceOf(signerB.address);
    console.log('relpBalanceBob : ', relpBalanceBob.output);

    // const poollr  = await poolContract.query.liabilityRatio();
    // console.log('liability_ratio  : ', poollr.output);

    const elc_totalSupply  = await elcContract.query.totalSupply();
    console.log('elc_totalSupply  : ', elc_totalSupply.output);

    const elcBalanceAlice = await elcContract.query.balanceOf(signerA.address);
    console.log('elcBalanceAlice  : ', elcBalanceAlice.output);

    const elcBalanceBob = await elcContract.query.balanceOf(signerB.address);
    console.log('elcBalanceBob  : ', elcBalanceBob.output);

    //expect(relpBalanceAlice.output/1000000/1000000/1000000).to.equal(10000000000000000/1000000/1000000/1000000);
  });
});
