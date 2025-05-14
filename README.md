# Freebonde NFT Minter

Freebonde NFT Minter is a web application that allows users to create unique Plant NFTs on the Solana blockchain using the Metaplex protocol. The frontend is built with React and provides an interactive interface for generating plant data (such as humidity, growth, weather, and more) and minting it as an NFT.

---

## Features

- 🌱 **Generate Plant Data**: Randomly generate plant attributes like humidity, growth stage, weather, and more.
- 🖼️ **NFT Minting**: Mint unique Plant NFTs on Solana using Metaplex.
- 🔗 **Wallet Integration**: Connect your Solana wallet (Phantom and others supported).
- ⚡ **Modern UI**: Built with React, Tailwind CSS, and Solana Wallet Adapter for a seamless experience.

---

## Getting Started

# Installation 
At first clone the github repository to your local machine. 
```
git clone https://github.com/FreeBonde-AB/FreeBonde-Marketplace-Solana-smart-contracts
```

To run the app go to project directory and enter these commands serially
```
npm install
npm start
```
or if you are using Yarn,

```
yarn install
yarn start
```

* Port used 3000. App is displayed in localhost:3000. If the port 3000 is not free, it will prompt and ask to open in another port.

---

## Usage

1. **Connect your Solana wallet** using the "Connect Wallet" button connect your wallet. Phantom wallet provider is used here.
2. **Generate plant data** with the provided button.
3. **Mint your Plant NFT** and view the transaction on Solana Devnet.

---

## Tech Stack

- **Frontend:** React, Tailwind CSS, Webpack, HTML, CSS.
- **Blockchain:** Solana, Metaplex JS SDK.
- **Wallet Integration:** Phantom Wallet

---

## License

This project is licensed under the MIT License.

---

## Acknowledgements

- [Solana Labs](https://solana.com/)
- [Metaplex](https://www.metaplex.com/)
- [Solana Wallet Adapter](https://github.com/solana-labs/wallet-adapter)

---


## Important Notes for Visitors

- **Network:**  
  This project is configured for Solana Devnet by default. Please do not use real assets or mainnet wallets for testing.

- **Wallet Support:**  
  The dApp currently supports Phantom Wallet. Make sure you have it installed and connected before interacting with the app.

- **NFT Minting:**  
  When minting an NFT, the NFT name follows a specific format for parsing plant and user data. Please do not modify the minting logic unless you understand the naming convention.

- **Marketplace Transactions:**  
  NFT sales require both the buyer and the seller to sign the transaction. The seller must be online and connected with their wallet to approve the sale.

- **No Multi-Wallet Support:**  
  The dApp does not support simultaneous multi-wallet signatures. Only the currently connected wallet can sign transactions.

- **Testing:**  
  All features should be tested on Devnet. If you encounter errors related to wallet signatures or transaction failures, ensure you are using the correct wallet and network.

- **Security:**  
  Do not use this project with valuable assets. This is a test/demo project and has not been audited.

- **Contribution:**  
  Contributions are welcome! Please fork the repository and submit a pull request.


**Happy Minting! 🌿**

