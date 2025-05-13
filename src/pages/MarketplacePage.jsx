import { useEffect, useState } from "react";
import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
import {
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    Transaction,
} from "@solana/web3.js";
import toast from "react-hot-toast";

const MarketplacePage = () => {
    // States
    const [nfts, setNfts] = useState([]);
    const [loading, setLoading] = useState(true);

    const parseNFT = (nft) => {
        const token_name = nft.content?.metadata?.name;
        const token_symbol = nft.content?.metadata?.symbol;
        const token_image = nft.content?.json_uri;

        return {
            token_name,
            token_symbol,
            token_image,
        };
    };

    useEffect(() => {
        fetchMarketplaceNFTs();
    }, []);

    const fetchMarketplaceNFTs = async () => {
        try {
            setLoading(true);

            // Fetch NFTs from Helius API
            const response = await fetch(
                `https://devnet.helius-rpc.com/?api-key=ec49d80c-6879-4d92-8d00-f485bf0901cc`,
                {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({
                        jsonrpc: "2.0",
                        id: "my-id",
                        method: "searchAssets",
                        params: {
                            grouping: [
                                "collection",
                                "CjUBBjARbAP3zJMr97inXwfYa9uDfMC4QmFJ3QwUhMPj",
                            ],
                            page: 1,
                            limit: 1000,
                        },
                    }),
                }
            );

            const data = await response.json();

            setNfts(data.result.items);

            setLoading(false);
        } catch (error) {
            setLoading(false);
            toast.error("Failed to get NFTs");
        }
    };

    return (
        <div className="container mx-auto px-4 py-8">
            <h2 className="text-3xl font-bold text-center mb-8">Marketplace</h2>
            <div className="flex flex-wrap justify-center gap-6">
                {loading && <p className="text-gray-600">Loading NFTs...</p>}
                {!loading && nfts.length === 0 && (
                    <p className="text-gray-600">No NFTs match the filter.</p>
                )}
                {nfts.map((nft, idx) => {
                    const { token_name, token_symbol, token_image } =
                        parseNFT(nft);
                    return (
                        <div
                            key={nft.address || idx}
                            style={{
                                margin: "8px",
                                padding: "16px",
                                minWidth: "260px",
                                display: "flex",
                                flexDirection: "column",
                                alignItems: "center",
                                boxShadow:
                                    "0 4px 16px rgba(0,0,0,0.13), 0 1.5px 4px rgba(0,0,0,0.09)",
                                borderRadius: "12px",
                                background: "#fff",
                            }}
                        >
                            <h3 className="text-xl font-semibold mb-2 mt-4">
                                {token_name || "No Name"}
                            </h3>
                            {token_image && (
                                <img
                                    src={token_image}
                                    alt={token_name}
                                    style={{
                                        width: "180px",
                                        height: "180px",
                                        objectFit: "cover",
                                        borderRadius: "8px",
                                        marginTop: "8px",
                                    }}
                                />
                            )}
                            <div
                                style={{
                                    marginTop: "8px",
                                    background: "#f8f8f8",
                                    borderRadius: "8px",
                                    padding: "10px",
                                    textAlign: "center",
                                    width: "100%",
                                }}
                            >
                                <div>
                                    <strong>Symbol:</strong> {token_symbol}
                                </div>
                            </div>
                            <button
                                style={{
                                    background: "#ff9800",
                                    color: "#fff",
                                    border: "none",
                                    borderRadius: "6px",
                                    padding: "6px 16px",
                                    cursor: "pointer",
                                    margin: "16px 0",
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
        const metaplex = Metaplex.make(connection).use(
            keypairIdentity(publicKey)
        );
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
