import React, { useEffect, useState } from 'react';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { getTokenAccountsByOwner, AccountLayout } from '@solana/spl-token';
import { Metaplex } from '@metaplex-foundation/js';
import { Connection, PublicKey } from '@solana/web3.js';

const MyNFTsPage = () => {
  const { publicKey, connected } = useWallet();
  const { connection } = useConnection();
  const [nfts, setNfts] = useState([]);
  const [loading, setLoading] = useState(true);

  const fetchNFTs = async (ownerPublicKey, solanaConnection) => {
    if (!ownerPublicKey || !solanaConnection) return;

    try {
      setLoading(true);
      const tokenAccounts = await solanaConnection.getParsedTokenAccountsByOwner(
        ownerPublicKey,

        {
          programId: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'), // SPL Token Program ID
        }
      );

      const metaplex = Metaplex.make(solanaConnection);

      const nftPromises = tokenAccounts.value
        .filter(account => account.account.data.parsed.info.tokenAmount.amount === '1' && account.account.data.parsed.info.tokenAmount.decimals === 0)
        .map(async (account) => {
          try {
            const mintAddress = account.account.data.parsed.info.mint;
            const nft = await metaplex.nfts().findByMint({ mintAddress: new PublicKey(mintAddress) });
            console.log("NFT Metadata URI for", account.account.data.parsed.info.mint, ":", nft.uri);

            // Fetch and parse metadata JSON
            if (nft.uri) {
              // Check if the URI points directly to an image
              const isImageUrl = /\.(jpeg|jpg|png|gif)$/i.test(nft.uri);

              if (isImageUrl) {
                nft.json = { image: nft.uri }; // Set image directly if it's an image URL
              } else {
                const response = await fetch(nft.uri);
                const jsonMetadata = await response.json();
                nft.json = jsonMetadata; // Attach the parsed JSON metadata to the nft object
              }
            }

            return nft;
          } catch (innerError) {
            console.error(`Error fetching NFT metadata for mint ${account.account.data.parsed.info.mint}:`, innerError);
            return null; // Return null for failed fetches
          }
        });

      let fetchedNfts = await Promise.all(nftPromises);

      // Generate simulated plant data for each NFT
      fetchedNfts = fetchedNfts.map(nft => {
        if (nft) {
          nft.simulatedData = {
            temperature: Math.floor(Math.random() * 30) + 15, // 15-45
            humidity: Math.floor(Math.random() * 50) + 30,   // 30-80
            ph: parseFloat((Math.random() * 2 + 6).toFixed(1)), // 6.0-8.0
            ec: parseFloat((Math.random() * 0.5 + 1).toFixed(1)), // 1.0-1.5
            plant_type: Math.floor(Math.random() * 5) + 1, // 1-5
          };
        }
        return nft;
      });
      setNfts(fetchedNfts.filter(nft => nft !== null && nft.json?.image)); // Filter out null results and NFTs without an image

    } catch (error) {
      console.error("Error fetching NFTs:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (connected && publicKey && connection) {
      console.log("Wallet Public Key:", publicKey?.toBase58());
      fetchNFTs(publicKey, connection);
    } else {
      setNfts([]); // Clear NFTs if wallet is disconnected
      setLoading(false);
    }
  }, [publicKey, connection, connected]);

  return (
    <div className="container mx-auto px-4 py-8">
      <h2 className="text-3xl font-bold text-center mb-8">My NFTs</h2>
      <div className="flex flex-wrap justify-center gap-4">
        {loading && <p className="text-gray-600">Loading NFTs...</p>}
        {!loading && nfts.length === 0 && publicKey && (
          <p className="text-gray-600">No NFTs found for this wallet.</p>
        )}
         {!loading && nfts.length === 0 && !publicKey && (
          <p className="text-gray-600">Connect your wallet to see your NFTs.</p>
        )}
        {!loading && nfts.length > 0 && (
          nfts.map(nft => (
            <div key={nft.address.toBase58()} className="border p-4 rounded shadow-md">
              <h3 className="text-xl font-semibold mb-2">{nft.name}</h3>
              {nft.json?.image && (
                <img src={nft.json.image} alt={nft.name} className="w-32 h-32 object-cover mb-2"/>
              )}
              <p className="text-sm text-gray-700">Mint: {nft.address.toBase58()}</p>
              {nft.simulatedData && (
                <div className="text-sm text-gray-700 mt-2">
                  <p>Temperature: {nft.simulatedData.temperature}°C</p>
                  <p>Humidity: {nft.simulatedData.humidity}%</p>
                  <p>PH: {nft.simulatedData.ph}</p>
                  <p>EC: {nft.simulatedData.ec} mS/cm</p>
                  {/* Plant Type display can be added here if you map plant_type to a name */}
                </div>
              )}
              {/* Add more NFT details here as needed */}
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default MyNFTsPage;