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

    const owner_res = await elcContract.query.owner();
    console.log('owner: ', owner_res.output.toString());
    console.log('sender: ', signer.address.toString());

    const balance_res = await elcContract.query.balanceOf(signer.address);
    console.log('call balanceOf: ', balance_res.output.toHuman());

    const name = await elcContract.query.tokenName();
    console.log('name: ', name.output.toHuman());

    const testcaller = await elcContract.query.testcaller();
    console.log('name: ', testcaller.output.toHuman());

    const pool = api.createType('AccountId', "5DHmTkUFUvpSFS5f7Y5FXwGVVMUNbAfYiYqPA4ssHDKNgZSy");
    console.log('transferOwnership....................');
    const result = await elcContract.tx.transferOwnership(pool, {
        signer: signer
    });
    console.log('transferOwnership: ', result.output.toHuman());

    api.disconnect();
}

run().catch((err) => {
    console.log(err);
});
