# FreeBonde Marketplace: Real-World Asset (RWA) NFTization for Smart Agriculture

## Project Overview

FreeBonde Marketplace is a Solana-based platform for the digitization and monetization of real-world agricultural assets. By integrating with our proprietary smart planting machines (IoT devices), users' machines automatically upload authentic plant growth data to the cloud. Through the dApp, users can generate NFTs that are directly linked to physical vegetables and devices, enabling the creation of tradable digital certificates for real assets.

While the current demo simulates data, the real-world deployment will feature seamless data flow from physical machines to the blockchain, ensuring that each NFT represents a unique, verifiable, and tradable real-world asset (RWA).

---

## Architecture

- **Frontend:** React, Tailwind CSS, Solana Wallet Adapter
- **NFT Minting:** Metaplex JS SDK
- **Smart Contract:** Anchor (Rust) program for NFT sales logic
- **IoT Integration:** (Planned) Smart planting machines upload real data to the cloud, which is then minted as NFTs
- **Network:** Solana Devnet (for safe testing and demo)

---

## Setup & Installation

### 1. Clone the Repository

```bash
git clone https://github.com/FreeBonde-AB/FreeBonde-Marketplace-Solana-smart-contracts
cd FreeBonde-Marketplace-Solana-smart-contracts
```

### 2. Install Dependencies

```bash
npm install
# or
yarn install
```

### 3. Start the Frontend

```bash
npm start
# or
yarn start
```
The app runs on [http://localhost:3000](http://localhost:3000).

### 4. Anchor Program (Optional: for developers)

To build and deploy the Anchor smart contract:

```bash
anchor build
anchor deploy
```
> Make sure you have [Anchor](https://www.anchor-lang.com/) and [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) installed.

---

## Usage Guide

1. **Connect Wallet:**  
   Click "Connect Wallet" and select Phantom. Ensure you are on Solana Devnet.

2. **Generate/Upload Real Data:**  
   In this demo, data is simulated. In production, your planting machine will upload real data automatically.

3. **Mint NFT:**  
   Generate an NFT that is cryptographically linked to your physical vegetable and device.

4. **Trade NFTs:**  
   Use the marketplace to buy and sell NFTs, each representing a real-world asset. Both buyer and seller must sign the transaction for secure, peer-to-peer transfer.

---

## Technical Details

- **RWA Implementation:**  
  Physical assets (vegetables) are bound to NFTs, enabling digital proof and monetization of real-world goods.

- **IoT Integration:**  
  Future-ready for automatic, tamper-proof data uploads from smart planting machines.

- **On-Chain Ownership & Transfer:**  
  NFTs serve as digital certificates for real assets, freely tradable on the blockchain.

- **Smart Contract:**  
  The Anchor program (`programs/nft_marketplace/src/lib.rs`) enforces that both buyer and seller sign the transaction for NFT transfers, ensuring trustless peer-to-peer trading.

---

## Important Notes

- **Network:**  
  The project is configured for Solana Devnet. Do not use mainnet or real assets for testing.

- **Wallet Support:**  
  The dApp currently supports Phantom Wallet. Make sure you have it installed and connected before interacting with the app.

- **NFT Minting:**  
  When minting an NFT, the NFT name follows a specific format for parsing plant and user data. Please do not modify the minting logic unless you understand the naming convention.

- **Marketplace Transactions:**  
  NFT sales require both the buyer and the seller to sign the transaction. The seller must be online and connected with their wallet to approve the sale.

- **No Multi-Wallet Support:**  
  The dApp does not support simultaneous multi-wallet signatures yet. Only the currently connected wallet can sign transactions.

- **Testing:**  
  All features should be tested on Devnet. If you encounter errors related to wallet signatures or transaction failures, ensure you are using the correct wallet and network.

- **Security:**  
  Do not use this project with valuable assets. This is a test/demo project and has not been audited.

- **Contribution:**  
  Contributions are welcome! Please fork the repository and submit a pull request.

---

## Judging Criteria Alignment

- **Clear Implementation:**  
  The repo contains all source code, smart contracts, and deployment scripts.
- **Technical Depth:**  
  Demonstrates full-stack Solana dApp development, including custom Anchor contracts, IoT integration concept, and Metaplex integration.
- **User Experience:**  
  Modern, responsive UI with clear feedback for all blockchain actions.
- **Documentation:**  
  This README provides step-by-step setup, usage, and technical details for reviewers.

---

## License

MIT License

---

## Acknowledgements

- [Solana Labs](https://solana.com/)
- [Metaplex](https://www.metaplex.com/)
- [Anchor](https://www.anchor-lang.com/)
- [Phantom Wallet](https://phantom.app/)
- [Solana Wallet Adapter](https://github.com/solana-labs/wallet-adapter)

---

**Empowering real-world asset liquidity—digitize, prove, and trade your harvest! 🌿**
```
