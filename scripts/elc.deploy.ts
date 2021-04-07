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
    value: '0',
  });

  console.log('');
  console.log(
    'Deploy elcContract successfully. The contract address: ',
      elcContract.address.toString()
  );

  // deploy relp contract
  const relpFactory = await getContractFactory('relp', signer);
  const relpContract = await relpFactory.deployed('new', '1000000000000', elcContract.address,{
    gasLimit: '200000000000',
    value: '0',
  });
  console.log('');
  console.log(
      'Deploy relpContract successfully. The contract address: ',
      relpContract.address.toString()
  );

  // deploy oracle contract
  const oracleFactory = await getContractFactory('oracle', signer);
  const oracleContract = await oracleFactory.deployed('new', {
    gasLimit: '200000000000',
    value: '0',
  });
  console.log('');
  console.log(
      'Deploy oracleContract successfully. The contract address: ',
      oracleContract.address.toString()
  );

  // deploy pool contract
  const exchange_account =  api.createType('AccountId', "5EuWbAoT1gRjGxCT1NQV2TtZofoCBQUWvfwUCq3yBAwwc55S");
  const poolFactory = await getContractFactory('pool', signer);
  const poolContract = await poolFactory.deployed('new',
      '1000000000000000000', elcContract.address, relpContract.address, oracleContract.address, exchange_account,{
    gasLimit: '200000000000',
    value: '0',
  });
  console.log('');
  console.log(
      'Deploy poolContract successfully. The contract address: ',
      poolContract.address.toString()
  );

  // transfer ownerships
  await elcContract.tx['elc,transfer_ownership'](poolContract.address);
  await relpContract.tx['relp,transfer_ownership'](poolContract.address);

  api.disconnect();
}

run().catch((err) => {
  console.log(err);
});
