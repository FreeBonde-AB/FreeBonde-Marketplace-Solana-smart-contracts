import React, { useEffect, useState } from "react";
import { useWallet, useConnection } from "@solana/wallet-adapter-react";
import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
import { PublicKey, LAMPORTS_PER_SOL, SystemProgram, Transaction } from "@solana/web3.js";

// Assume you have an API or global state to get all NFTs, here is mock data for demonstration
// Please replace with real NFT fetching logic in actual project
const mockNFTs = [
  // Example data structure
  // {
  //   address: "xxx",
  //   name: "User-Lon-12-Le-1.2-6.5-22-052312",
  //   image: "https://arweave.net/fake-image-url.jpg",
  //   owner: "User",
  //   city: "London",
  //   plant: "Lettuce",
  // }
];

const cityMap = {
  "Lon": "London", "Par": "Paris", "Ber": "Berlin", "Rom": "Rome", "Mad": "Madrid",
  "Ott": "Ottawa", "Was": "Washington", "Tok": "Tokyo", "Can": "Canberra", "Mos": "Moscow",
  "Bra": "Brasilia", "Bei": "Beijing", "Seo": "Seoul", "Ban": "Bangkok", "New": "New Delhi",
  "Cai": "Cairo", "Bue": "Buenos Aires", "Wel": "Wellington", "Osl": "Oslo", "Sto": "Stockholm"
};
const plantMap = {
  "Le": "Lettuce", "To": "Tomato", "Sp": "Spinach", "Ka": "Kale",
  "Be": "Bell Pepper", "Ce": "Celery", "Ca": "Cauliflower"
};

const MarketplacePage = () => {
  const [nfts, setNfts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [filterUser, setFilterUser] = useState("");
  const [filterCity, setFilterCity] = useState("");
  const [filterPlant, setFilterPlant] = useState("");

  // Replace with real API to fetch all NFTs on devnet
  useEffect(() => {
    setNfts(mockNFTs);
  }, []);

  const parseNFT = (nft) => {
    let grower = "N/A", city = "N/A", plant = "N/A";
    if (nft.name) {
      const parts = nft.name.split("-");
      if (parts.length >= 4) {
        grower = parts[0] || "N/A";
        city = cityMap[parts[1]] || parts[1] || "N/A";
        plant = plantMap[parts[3]] || parts[3] || "N/A";
      }
    }
    return { grower, city, plant };
  };

  const filteredNFTs = nfts.filter(nft => {
    const { grower, city, plant } = parseNFT(nft);
    return (
      (!filterUser || grower.toLowerCase().includes(filterUser.toLowerCase())) &&
      (!filterCity || city === filterCity) &&
      (!filterPlant || plant === filterPlant)
    );
  });

  // Simulate buy: replace with real Solana transaction and NFT transfer logic
  const { publicKey, sendTransaction } = useWallet();
  const { connection } = useConnection();

  useEffect(() => {
    const fetchAllNFTs = async () => {
      if (!connection) return;
      setLoading(true);
      try {
        // This is just a demo, in reality you may need to aggregate all NFTs from all users or a backend service
        // Here only fetch NFTs of current wallet for demonstration
        // You can adjust to fetch all or specific NFTs as needed
        const metaplex = Metaplex.make(connection);
        // Example: fetch all NFTs under a collection
        // const nfts = await metaplex.nfts().findAllByCreator({ creator: new PublicKey("xxx") });
        // Here only fetch current wallet's NFTs
        // const tokenAccounts = await connection.getParsedTokenAccountsByOwner(publicKey, { programId: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA') });
        // ...similar logic as MyNFTsPage.tsx...
        setNfts([]); // Replace with real NFT array here
      } catch (e) {
        setNfts([]);
      }
      setLoading(false);
    };
    fetchAllNFTs();
  }, [connection]);
  const allCities = Array.from(new Set(nfts.map(nft => parseNFT(nft).city))).filter(Boolean);
  const allPlants = Array.from(new Set(nfts.map(nft => parseNFT(nft).plant))).filter(Boolean);

  return (
    <div className="container mx-auto px-4 py-8">
      <h2 className="text-3xl font-bold text-center mb-8">Marketplace</h2>
      <div className="flex flex-wrap gap-4 mb-6 justify-center">
        <input
          type="text"
          placeholder="Filter by username"
          value={filterUser}
          onChange={e => setFilterUser(e.target.value)}
          className="border px-2 py-1 rounded"
        />
        <select
          value={filterCity}
          onChange={e => setFilterCity(e.target.value)}
          className="border px-2 py-1 rounded"
        >
          <option value="">All Cities</option>
          {allCities.map(city => (
            <option key={city} value={city}>{city}</option>
          ))}
        </select>
        <select
          value={filterPlant}
          onChange={e => setFilterPlant(e.target.value)}
          className="border px-2 py-1 rounded"
        >
          <option value="">All Plants</option>
          {allPlants.map(plant => (
            <option key={plant} value={plant}>{plant}</option>
          ))}
        </select>
      </div>
      <div className="flex flex-wrap justify-center gap-6">
        {loading && <p className="text-gray-600">Loading NFTs...</p>}
        {!loading && filteredNFTs.length === 0 && (
          <p className="text-gray-600">No NFTs match the filter.</p>
        )}
        {filteredNFTs.map((nft, idx) => {
          const { grower, city, plant } = parseNFT(nft);
          return (
            <div
              key={nft.address || idx}
              style={{
                margin: "8px",
                minWidth: "260px",
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                boxShadow: "0 4px 16px rgba(0,0,0,0.13), 0 1.5px 4px rgba(0,0,0,0.09)",
                borderRadius: "12px",
                background: "#fff"
              }}
            >
              <h3 className="text-xl font-semibold mb-2 mt-4">{nft.name}</h3>
              {nft.image && (
                <img
                  src={nft.image}
                  alt={nft.name}
                  style={{
                    width: "180px",
                    height: "180px",
                    objectFit: "cover",
                    borderRadius: "8px",
                    marginTop: "8px"
                  }}
                />
              )}
              <div style={{ marginTop: "8px", background: "#f8f8f8", borderRadius: "8px", padding: "10px", textAlign: "center", width: "100%" }}>
                <div><strong>Grower:</strong> {grower}</div>
                <div><strong>City:</strong> {city}</div>
                <div><strong>Plant:</strong> {plant}</div>
              </div>
              <button
                style={{
                  background: "#ff9800",
                  color: "#fff",
                  border: "none",
                  borderRadius: "6px",
                  padding: "6px 16px",
                  cursor: "pointer",
                  margin: "16px 0"
                }}
                onClick={() => handleBuy(nft)}
              >
                Buy (0.00001 SOL)
              </button>
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default MarketplacePage;

// Buy NFT and transfer ownership
const handleBuy = async (nft) => {
  if (!publicKey || !connection) {
    alert("Please connect your wallet.");
    return;
  }
  try {
    // 1. Pay 0.00001 SOL to the seller
    const seller = new PublicKey(nft.owner); // Assume nft.owner is the seller's address
    const tx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: publicKey,
        toPubkey: seller,
        lamports: 0.00001 * LAMPORTS_PER_SOL,
      })
    );
    const signature = await sendTransaction(tx, connection);
    await connection.confirmTransaction(signature, "confirmed");
  
    // 2. Transfer NFT
    const metaplex = Metaplex.make(connection).use(keypairIdentity(publicKey));
    await metaplex.nfts().transfer({
      mintAddress: new PublicKey(nft.address),
      toOwner: publicKey,
    });
  
    alert("Purchase and transfer successful!");
    // TODO: Refresh NFT list
  } catch (err) {
    alert("Transaction failed: " + err.message);
  }
};