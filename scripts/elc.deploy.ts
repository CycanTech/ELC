import { patract, network } from 'redspot';

const { getContractFactory } = patract;
const { createSigner, keyring, api } = network;

const uri =
  'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice';

async function run() {
  await api.isReady;

  const signer = createSigner(keyring.createFromUri(uri));
  const balance = await api.query.system.account(signer.address);
  console.log('Balance: ', balance.toHuman());

  // deploy elc contract
  const elcFactory = await getContractFactory('elc', signer);

  const elcContract = await elcFactory.deployed('new', '1000000000000', {
    gasLimit: '200000000000',
    value: '1000000000000000',
    salt: 'elc'
  });

  console.log('');
  console.log(
    'Deploy elcContract successfully. The contract address: ',
      elcContract.address.toString()
  );

  // deploy oracle contract
  const oracleFactory = await getContractFactory('oracle', signer);
  const oracleContract = await oracleFactory.deployed('new', {
    gasLimit: '200000000000',
    value: '1000000000000000',
    salt: 'oracle',
  });
  console.log('');
  console.log(
      'Deploy oracleContract successfully. The contract address: ',
      oracleContract.address.toString()
  );

  // deploy relp contract
  const relpFactory = await getContractFactory('relp', signer);
  const relpContract = await relpFactory.deployed('new', '1000000000000', elcContract.address,{
    gasLimit: '200000000000',
    value: '10000000000000000',
    salt: 'relp',
  });
  console.log('');
  console.log(
      'Deploy relpContract successfully. The contract address: ',
      relpContract.address.toString()
  );

  // deploy pool contract
  const exchange_account =  api.createType('AccountId', "5GeJTi5fmhaQKUfpxTYKQGcaSEpWZ4grQVcng4ce5DyWwSrG");
  const poolFactory = await getContractFactory('pool', signer);
  const poolContract = await poolFactory.deployed('new',
      '1000000000000000000', elcContract.address, relpContract.address, oracleContract.address, exchange_account,{
    gasLimit: '200000000000',
    value: '1000000000000000',
  });
  console.log('');
  console.log(
      'Deploy poolContract successfully. The contract address: ',
      poolContract.address.toString()
  );

  // transfer ownerships
  // await elcContract.tx['elc,transferOwnership'](poolContract.address);
  // await relpContract.tx['relp,transferOwnership'](poolContract.address);
  console.log('transferOwnership....................');
  const result = await elcContract.tx.transferOwnership(poolContract.address.toString());
  console.log('transferOwnership: ', result);
  
  console.log('transferOwnership....................');
  const result2 = await relpContract.tx.transferOwnership(poolContract.address.toString());
  console.log('transferOwnership: ', result2);

  api.disconnect();
}

run().catch((err) => {
  console.log(err);
});
