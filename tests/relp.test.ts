import BN from 'bn.js';
import { expect } from 'chai';
import { patract, network, artifacts } from 'redspot';

const { getContractFactory, getRandomSigner } = patract;

const { api, getSigners } = network;

describe('relp', () => {
  after(() => {
    return api.disconnect();
  });

  async function setup() {
    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]));
    const signers = await getSigners();
    const Alice = signers[0];
    const sender = await getRandomSigner(Alice, one.muln(10000));
    const contractFactory = await getContractFactory('relp', sender);
    const relpContract = await contractFactory.deploy('new', 100000000000000, Alice.address, {
      gasLimit: '200000000000',
      value: '1000000000000000',
      salt: 'relp'
    });

    const abi = artifacts.readArtifact('relp');
    const receiver = await getRandomSigner();

    return { sender, contractFactory, relpContract, abi, receiver, Alice, one };
  }

  it('get totalsupply', async () => {
    const { relpContract, sender } = await setup();
    const totalSupply = await relpContract.query.totalSupply();
    console.log('total_supply: ', totalSupply.output);
    expect(totalSupply.output).to.equal(100000000000000);
  });

  it('get balanceof', async () => {
    const { relpContract, sender } = await setup();
    const balanceof = await relpContract.query.balanceOf(sender.address);
    console.log('balanceof sender: ', balanceof.output);
    expect(balanceof.output).to.equal(100000000000000);
  });

  it('mint', async () => {
    const { relpContract, sender } = await setup();
    const result = await relpContract.tx.mint(sender.address, 1000000000000);
    console.log('mint: ', result.output);
    const totalSupply2 = await relpContract.query.totalSupply();
    console.log('total_supply: ', totalSupply2.output);
    expect(totalSupply2.output).to.equal(101000000000000);
  });

  it('burn', async () => {
    const { relpContract, sender } = await setup();
    const result = await relpContract.tx.burn(sender.address, 1000000000000);
    console.log('burn: ', result.output);
    const totalSupply2 = await relpContract.query.totalSupply();
    console.log('total_supply: ', totalSupply2.output);
    expect(totalSupply2.output).to.equal(99000000000000);
  });

});