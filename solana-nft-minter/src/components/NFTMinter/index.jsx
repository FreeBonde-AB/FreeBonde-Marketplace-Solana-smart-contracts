import { useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { Metaplex } from "@metaplex-foundation/js";
import { Connection, clusterApiUrl } from "@solana/web3.js";

const NFTMinter = () => {
    const [image, setImage] = useState(null);
    const [metadata, setMetadata] = useState({
        name: "",
        description: "",
        symbol: "",
    });
    const { publicKey } = useWallet();

    const handleImageUpload = (e) => {
        const file = e.target.files[0];
        setImage(file);
    };

    const uploadToArweave = async (file) => {
        console.log('arweave upload', file);
        
        // Implement Arweave upload logic
        // Return the Arweave URL
    };

    const mintNFT = async () => {
        try {
            const connection = new Connection(clusterApiUrl("devnet"));
            const metaplex = new Metaplex(connection);

            // Upload image to Arweave
            const imageUrl = await uploadToArweave(image);

            // Create NFT metadata
            const { nft } = await metaplex.nfts().create({
                uri: imageUrl,
                name: metadata.name,
                sellerFeeBasisPoints: 500, // 5%
                symbol: metadata.symbol,
            });

            console.log("NFT created:", nft);
        } catch (error) {
            console.error("Error minting NFT:", error);
        }
    };

    return (
        <div>
            <input type="file" onChange={handleImageUpload} />
            <input
                type="text"
                placeholder="NFT Name"
                onChange={(e) =>
                    setMetadata({ ...metadata, name: e.target.value })
                }
            />
            <input
                type="text"
                placeholder="Description"
                onChange={(e) =>
                    setMetadata({ ...metadata, description: e.target.value })
                }
            />
            <input
                type="text"
                placeholder="Symbol"
                onChange={(e) =>
                    setMetadata({ ...metadata, symbol: e.target.value })
                }
            />
            <button onClick={mintNFT} disabled={!publicKey || !image}>
                Mint NFT
            </button>
        </div>
    );
};

export default NFTMinter;
