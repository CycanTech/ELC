import { patract, network } from 'redspot';

const { getContractAt } = patract;
const { createSigner, keyring, api } = network;

const uri =
    'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice';

async function run() {
    await api.isReady;

    const signer = createSigner(keyring.createFromUri(uri));
    const balance = await api.query.system.account(signer.address);
    console.log('Balance: ', balance.toHuman());

    const elc = api.createType('AccountId', "5FKeMm5ux3Mw1ovoszhf1aTm7cvguWaVzMPA4Lu5yztQW3DW");
    const relp = api.createType('AccountId', "5GeJTi5fmhaQKUfpxTYKQGcaSEpWZ4grQVcng4ce5DyWwSrG");
    const elcContract = await getContractAt('elc', elc, signer);
    const owner_res = await elcContract.query.owner();
    console.log('owner: ', owner_res.output.toString());
    console.log('sender: ', signer.address.toString());

    const balance_res = await elcContract.query.balanceOf(signer.address);
    console.log('call balanceOf: ', balance_res.output.toHuman());

    const name = await elcContract.query.tokenName();
    console.log('name: ', name.output.toHuman());

    const testcaller = await elcContract.query.testcaller();
    console.log('name: ', testcaller.output.toHuman());

    api.disconnect();
}

run().catch((err) => {
    console.log(err);
});
