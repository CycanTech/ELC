import { expect } from 'chai';
import { patract, network, artifacts} from 'redspot';

const { getContractFactory,  getContractAt} = patract;
const { createSigner, keyring, api } = network;

const uriAlice =
  'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice';
const uriBob =
  'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob';
const uriCharlie =
  'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Charlie';
const uriDave =
  'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Dave';
const uriEve =
  'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Eve';
const uriFerdie =
  'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Ferdie';

describe('ELP', () => {
    after(() => {
      return api.disconnect();
    });

    async function setup() {
      await api.isReady;
      const signerA = createSigner(keyring.createFromUri(uriAlice));
      const signerB = createSigner(keyring.createFromUri(uriBob));
      const signerC = createSigner(keyring.createFromUri(uriCharlie));
      const signerD = createSigner(keyring.createFromUri(uriDave));
      const signerE = createSigner(keyring.createFromUri(uriEve));
      const signerF = createSigner(keyring.createFromUri(uriFerdie));

      /*
      Deploy elcContract successfully. The contract address:  5GMDNNDm7MMoZ78CGcvXTjecYDqmfBunSgbiA18jKoGhDpuJ
      Deploy oracleContract successfully. The contract address:  5Gk6eQrgrPavbAiBoS4ibe3UvQxbje5eFbu5DSyR9asx7Pim
      Deploy relpContract successfully. The contract address:  5H5YnhDJik3ftjtQEM7iueCbYB2mS1KS5LqoN1jQjbYn4NH7
      Deploy lptContract successfully. The contract address:  5Fc2yeeysxmfN7tVhevNSL1bMAsX5KPVbycfwDDVKvcoqGbC
      Deploy exchange2Contract successfully. The contract address:  5FqkzA4VKPTtianwNWfMfR41N7uYmGWko8GA5kNbADW7HMi6
      Deploy poolContract successfully. The contract address:  5GjCK2Z9nVFgnR82YU37UTFktM8EeuxWzccJfNUxmi8FuZK4
      elc old owner.................... 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
      */

      const elc = api.createType('AccountId', "5GMDNNDm7MMoZ78CGcvXTjecYDqmfBunSgbiA18jKoGhDpuJ");
      const oracle = api.createType('AccountId', "5Gk6eQrgrPavbAiBoS4ibe3UvQxbje5eFbu5DSyR9asx7Pim");
      const relp = api.createType('AccountId', "5H5YnhDJik3ftjtQEM7iueCbYB2mS1KS5LqoN1jQjbYn4NH7");
      const pool = api.createType('AccountId', "5GjCK2Z9nVFgnR82YU37UTFktM8EeuxWzccJfNUxmi8FuZK4");
      const exchange2 = api.createType('AccountId', "5FqkzA4VKPTtianwNWfMfR41N7uYmGWko8GA5kNbADW7HMi6");

      const elcContract = await getContractAt('elc', elc, signerA);
      const relpContract = await getContractAt('relp', relp, signerA);
      const oracleContract = await getContractAt('oracle', oracle, signerA);
      const poolContract = await getContractAt('pool', pool, signerA);
      const exchangeContract = await getContractAt('exchange2', exchange2, signerA);

      return { elcContract, relpContract, oracleContract, poolContract, exchangeContract, signerA,  signerB, signerC, signerD, signerE, signerF};
    }

  it('add_liquidity', async () => {
    const { elcContract, relpContract, poolContract, signerA, signerB } = await setup();

    const relp_totalSupply0  = await relpContract.query.totalSupply();
    console.log('relp_totalSupply0 : ', relp_totalSupply0.output.toString());
    const elc_totalSupply0  = await elcContract.query.totalSupply();
    console.log('elc_totalSupply0  : ', elc_totalSupply0.output.toString());

    const poollr  = await poolContract.query.liabilityRatio();
    console.log('liability_ratio  : ', poollr.output.toString());

    const result_r  = await poolContract.tx.addLiquidity( {
      gasLimit: '600000000000',
      value: '5000000000000',
      signer: signerB
    });

    const relp_totalSupply1  = await relpContract.query.totalSupply();
    console.log('relp_totalSupply1 : ', relp_totalSupply1.output.toString());
    const elc_totalSupply1  = await elcContract.query.totalSupply();
    console.log('elc_totalSupply1  : ', elc_totalSupply1.output.toString());

    expect(relp_totalSupply1.output).to.equal(500000000000);
  });

  it('add_liquidity result', async () => {
    const { elcContract, relpContract, poolContract, signerA, signerB } = await setup();

    const relp_totalSupply1  = await relpContract.query.totalSupply();
    console.log('relp_totalSupply1 : ', relp_totalSupply1.output.toString());
    const relpBalanceAlice = await relpContract.query.balanceOf(signerA.address);
    console.log('relpBalanceAlice : ', relpBalanceAlice.output.toString());
    const relpBalanceBob = await relpContract.query.balanceOf(signerB.address);
    console.log('relpBalanceBob : ', relpBalanceBob.output.toString());

    const elc_totalSupply1  = await elcContract.query.totalSupply();
    console.log('elc_totalSupply1  : ', elc_totalSupply1.output.toString());
    const elcBalanceAlice = await elcContract.query.balanceOf(signerA.address);
    console.log('elcBalanceAlice  : ', elcBalanceAlice.output.toString());
    const elcBalanceBob = await elcContract.query.balanceOf(signerB.address);
    console.log('elcBalanceBob  : ', elcBalanceBob.output.toString());

    const poollr  = await poolContract.query.liabilityRatio();
    console.log('liability_ratio  : ', poollr.output.toString());

    expect(relpBalanceBob.output).to.equal(500000000000);
  });
});