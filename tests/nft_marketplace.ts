import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftMarketplace } from "../target/types/nft_marketplace";
import { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";

describe("nft_marketplace", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NftMarketplace as Program<NftMarketplace>;

  let buyer = anchor.web3.Keypair.generate();
  let seller = anchor.web3.Keypair.generate();
  let mint = null;
  let sellerNftAccount = null;
  let buyerNftAccount = null;
  const price = anchor.web3.LAMPORTS_PER_SOL / 100; // 0.01 SOL

  before(async () => {
    // Airdrop SOL to buyer and seller
    await provider.connection.requestAirdrop(buyer.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(seller.publicKey, anchor.web3.LAMPORTS_PER_SOL);

    // Create NFT mint
    mint = await createMint(
      provider.connection,
      seller,
      seller.publicKey,
      null,
      0 // decimals
    );

    // Create associated token accounts
    sellerNftAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      seller,
      mint,
      seller.publicKey
    );
    buyerNftAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      buyer,
      mint,
      buyer.publicKey
    );

    // Mint 1 NFT to seller
    await mintTo(
      provider.connection,
      seller,
      mint,
      sellerNftAccount.address,
      seller,
      1
    );
  });

  it("Buy NFT", async () => {
    await program.methods
      .buyNft(new anchor.BN(price))
      .accounts({
        buyer: buyer.publicKey,
        seller: seller.publicKey,
        sellerNftAccount: sellerNftAccount.address,
        buyerNftAccount: buyerNftAccount.address,
        tokenMint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([buyer, seller]) // 这里建议 buyer 和 seller 都作为 signer
      .rpc();

    const buyerAccountInfo = await provider.connection.getTokenAccountBalance(buyerNftAccount.address);
    console.log("Buyer NFT balance:", buyerAccountInfo.value.uiAmount);

    // 建议加断言，自动校验
    if (typeof expect !== "undefined") {
      expect(buyerAccountInfo.value.uiAmount).to.equal(1);
    }
  });
});