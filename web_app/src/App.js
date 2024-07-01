import React, { useState } from 'react';
import axios from 'axios';
import { Keyring } from '@polkadot/keyring';

function App() {
  const [password, setPassword] = useState('');
  const [account, setAccount] = useState(null);

  const createAccount = () => {
    const keyring = new Keyring({ type: 'sr25519' });
    const newAccount = keyring.addFromUri(password);
    setAccount(newAccount);
    // Encrypt and store seed
  };

  const mintNFT = async () => {
    const payload = /* create payload */;
    const signature = account.sign(payload);

    await axios.post(`${process.env.REACT_APP_RELAY_SERVER_URL}/relay`, {
      signedPayload: payload,
      signature,
      target: /* NFT contract address */,
      inputData: /* mint function call data */,
    });
  };

  return (
    <div className="App">
      <h1>Gasless NFT Minting</h1>
      <input
        type="password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        placeholder="Enter a password"
      />
      <button onClick={createAccount}>Create Account</button>
      {account && <button onClick={mintNFT}>Mint NFT</button>}
    </div>
  );
}

export default App;
