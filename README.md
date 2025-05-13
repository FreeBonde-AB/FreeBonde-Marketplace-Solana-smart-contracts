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

**Happy Minting! 🌿**

## Recent Updates

- Unified NFT name format for consistent frontend/backend parsing.
- Improved mintNFT logic: even if metadata fetch fails, successful mints are always reported as successful to the user.
- MyNFTsPage now sorts NFTs by mint time (newest first).
- Codebase is now more standardized and international-friendly.



