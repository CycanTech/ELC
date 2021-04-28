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

  // deploy patraswap on substrate node
  // depoly ELP-ELC pair on patraswap, assume address is 5GeJTi5fmhaQKUfpxTYKQGcaSEpWZ4grQVcng4ce5DyWwSrG
  /*
   const swap = api.createType('AccountId', "5GeJTi5fmhaQKUfpxTYKQGcaSEpWZ4grQVcng4ce5DyWwSrG")
   const swapFactory = await getContractAt('PatraFactory', swap, signer);
   const creatpair = await swapFactory.createExchangeWithDot(elcContract.address, {
      signer: signer
    });
    console.log('');
    console.log(
        'Creatpair successfully.
    );
  */

  const exchange_account =  api.createType('AccountId', "5GeJTi5fmhaQKUfpxTYKQGcaSEpWZ4grQVcng4ce5DyWwSrG");
  const poolFactory = await getContractFactory('pool', signer);
  const poolContract = await poolFactory.deployed('new', elcContract.address, relpContract.address, oracleContract.address, exchange_account,{
        gasLimit: '200000000000',
        value: '1000000000000000',
      });
  console.log('');
  console.log(
      'Deploy poolContract successfully. The contract address: ',
      poolContract.address.toString()
  );

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
