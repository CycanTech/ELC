import { patract, network } from 'redspot';

const { getContractFactory } = patract;
const { createSigner, keyring, api } = network;

const uri =
    'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice';

async function run() {
  await api.isReady;

  const signer = createSigner(keyring.createFromUri(uri));
  console.log('signer: ', signer.address);
  const balance = await api.query.system.account(signer.address);
  console.log('Balance: ', balance.toHuman());

  function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  // deploy elc contract
  const elcFactory = await getContractFactory('elc', signer);

  const elcContract = await elcFactory.deployed('new', {
    gasLimit: '200000000000',
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
    salt: 'oracle',
  });
  console.log('');
  console.log(
      'Deploy oracleContract successfully. The contract address: ',
      oracleContract.address.toString()
  );

  // deploy relp contract
  const relpFactory = await getContractFactory('relp', signer);
  const relpContract = await relpFactory.deployed('new', elcContract.address,{
    gasLimit: '200000000000',
    salt: 'relp',
  });
  console.log('');
  console.log(
      'Deploy relpContract successfully. The contract address: ',
      relpContract.address.toString()
  );

// deploy lpt contract
  const lptFactory = await getContractFactory('lpt', signer);
  const lptContract = await lptFactory.deployed('new', '1000000000000000000', 'lpt', 'lp Token', '12',{
    gasLimit: '200000000000',
    value: '80000000000000000',
    salt: 'lpt',
	});
  console.log('');
  console.log(
      'Deploy lptContract successfully. The contract address: ',
      lptContract.address.toString()
  );
  //await delay(30000);
  
  // deploy exchange2 contract
  const exchange2Factory = await getContractFactory('exchange2', signer);
  const exchange2Contract = await exchange2Factory.deployed('new', elcContract.address, lptContract.address,{
    gasLimit: '200000000000',
    value: '80000000000000000',
    salt: 'exchange2',
	});
  console.log('');
  console.log(
    'Deploy exchange2Contract successfully. The contract address: ',
    exchange2Contract.address.toString()
  );
  //await delay(30000
  
  // deploy pool contract
  const poolFactory = await getContractFactory('pool', signer);
  const poolContract = await poolFactory.deployed('new',
      elcContract.address, relpContract.address, oracleContract.address, exchange2Contract.address,{
    gasLimit: '200000000000',
    value: '800000000000000000',
    salt: 'pool',
  });

  console.log('');
  console.log(
      'Deploy poolContract successfully. The contract address: ',
      poolContract.address.toString()
  );

  // update elp price & elc price
  const resultOracleUpdate = await oracleContract.update(1299, 1300, {
    signer: signer
  });
  console.log('init elp price & elc price ....................', resultOracleUpdate.output);
  
  // transfer ownerships
  const resultElcOldOwner = await elcContract.owner();
  console.log('elc old owner....................', resultElcOldOwner.output.toString());
  const result = await elcContract.transferOwnership(poolContract.address, {
    signer: signer
  });
  console.log('elc transferOwnership: ', result.output);
  const resultElcNowOwner = await elcContract.owner();
  console.log('elc new owner....................', resultElcNowOwner.output.toString());

  const resultRelpOldOwner = await relpContract.owner();
  console.log('relp old owner....................', resultRelpOldOwner.output.toString());
  const result2 = await relpContract.transferOwnership(poolContract.address, {
    signer: signer
  });
  console.log('relp transferOwnership: ', result2.output);
  const resultRelpNowOwner = await relpContract.owner();
  console.log('relp new owner....................', resultRelpNowOwner.output.toString());

  api.disconnect();
}

run().catch((err) => {
  console.log(err);
});
