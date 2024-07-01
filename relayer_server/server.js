const express = require('express');
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const dotenv = require('dotenv');

dotenv.config();

const app = express();
app.use(express.json());

const relayerAccount = process.env.RELAYER_ACCOUNT;
const relayerPrivateKey = process.env.RELAYER_PRIVATE_KEY;
const nodeUrl = process.env.NODE_URL;

const keyring = new Keyring({ type: 'sr25519' });

async function main() {
    const provider = new WsProvider(nodeUrl);
    const api = await ApiPromise.create({ provider });

    const relayer = keyring.addFromUri(relayerPrivateKey);

    app.post('/relay', async (req, res) => {
        const { signedPayload, signature, target, inputData } = req.body;

        // Verify signature and payload (simplified)
        // ...

        // Relay the transaction
        const call = api.tx[target](...inputData);
        const hash = await call.signAndSend(relayer);
        res.json({ status: 'success', hash });
    });

    app.listen(3000, () => console.log('Relayer server listening on port 3000'));
}

main().catch(console.error);
