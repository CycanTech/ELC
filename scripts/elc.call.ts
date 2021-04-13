import { patract, network } from 'redspot';

const { getContractAt } = patract;
const { createSigner, keyring, api } = network;

const uri =
    'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice';

// const elc = '5G8VknFrQSbE47FKTd5kFv5KbwnVLCyY94CmWSHLyoUoEWEh';
// const relp = '5GeJTi5fmhaQKUfpxTYKQGcaSEpWZ4grQVcng4ce5DyWwSrG';

async function run() {
    await api.isReady;
    const signer = createSigner(keyring.createFromUri(uri));
    const balance = await api.query.system.account(signer.address);
    console.log('Balance: ', balance.toHuman());

    const elc = api.createType('AccountId', "5G8VknFrQSbE47FKTd5kFv5KbwnVLCyY94CmWSHLyoUoEWEh");
    const relp = api.createType('AccountId', "5GeJTi5fmhaQKUfpxTYKQGcaSEpWZ4grQVcng4ce5DyWwSrG");
    const elcContract = await getContractAt('elc', elc, signer);
    const owner_res = await elcContract.query.owner();
    console.log('owner: ', owner_res.output.toString());
    console.log('sender: ', signer.address.toString());

    const balance_res = await elcContract.query.balanceOf(signer.address);
    console.log('call balanceOf: ', balance_res.output);

    const name = await elcContract.query.tokenName();
    console.log('name: ', name.output.toHuman());

    const relpContract = await getContractAt('relp', relp, signer);
    const owner_res2 = await relpContract.query.owner();
    console.log('owner: ', owner_res2.output.toString());

    const symbol = await relpContract.query.tokenSymbol();
    // console.log('symbol: ', symbol.output.isNone);
    // console.log('symbol: ', symbol.output.toJSON)
    console.log('symbol: ', symbol.output.toHuman());

    const pool = api.createType('AccountId', "5DHmTkUFUvpSFS5f7Y5FXwGVVMUNbAfYiYqPA4ssHDKNgZSy");
    console.log('transferOwnership....................');
    const result = await elcContract.tx.transferOwnership(pool, {
        signer: signer
    });
    console.log('transferOwnership: ', result);

    api.disconnect();
}

run().catch((err) => {
    console.log(err);
});
