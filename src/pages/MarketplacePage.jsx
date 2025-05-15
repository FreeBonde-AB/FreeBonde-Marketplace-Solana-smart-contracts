import { useEffect, useState } from "react";
import { useWallet, useConnection } from "@solana/wallet-adapter-react";
import { Metaplex } from "@metaplex-foundation/js";
import {
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    Transaction,
} from "@solana/web3.js";
import toast from "react-hot-toast";
import {
    getAssociatedTokenAddress,
    createTransferInstruction,
    createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";
import idl from '../idl/nft_marketplace.json';
import * as anchor from "@project-serum/anchor"; 
import {
    ResponsiveContainer,
    LineChart,
    Line,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    Legend,
} from "recharts";

const programId = new PublicKey("GGmdwp7BVyBskiuQf6RQicozG8DojLTBDdd9HrLyqZSr"); // Program ID

const fixedGrowthData = [
    { day: 1, ec: 1.0, ph: 5.5, temp: 22 },
    { day: 2, ec: 1.2, ph: 5.8, temp: 22.2 },
    { day: 3, ec: 1.1, ph: 6.2, temp: 22.3 },
    { day: 4, ec: 1.3, ph: 6.8, temp: 22.5 },
    { day: 5, ec: 1.5, ph: 7.1, temp: 22.7 },
    { day: 6, ec: 1.4, ph: 6.5, temp: 22.9 },
    { day: 7, ec: 1.7, ph: 7.4, temp: 23.0 },
    { day: 8, ec: 1.6, ph: 7.8, temp: 23.1 },
    { day: 9, ec: 1.8, ph: 8.2, temp: 23.2 },
    { day: 10, ec: 1.5, ph: 8.5, temp: 23.3 },
    { day: 11, ec: 1.3, ph: 8.1, temp: 23.4 },
    { day: 12, ec: 1.2, ph: 7.7, temp: 23.5 },
    { day: 13, ec: 1.4, ph: 7.2, temp: 23.6 },
    { day: 14, ec: 1.6, ph: 6.9, temp: 23.7 },
    { day: 15, ec: 1.7, ph: 7.3, temp: 23.8 },
    { day: 16, ec: 1.5, ph: 7.8, temp: 23.9 },
    { day: 17, ec: 1.3, ph: 8.0, temp: 24.0 },
    { day: 18, ec: 1.2, ph: 8.3, temp: 24.1 },
    { day: 19, ec: 1.4, ph: 8.7, temp: 24.2 },
    { day: 20, ec: 1.6, ph: 8.9, temp: 24.3 },
    { day: 21, ec: 1.8, ph: 8.5, temp: 24.4 },
    { day: 22, ec: 1.7, ph: 8.1, temp: 24.5 },
    { day: 23, ec: 1.5, ph: 7.6, temp: 24.6 },
    { day: 24, ec: 1.3, ph: 7.2, temp: 24.7 },
    { day: 25, ec: 1.2, ph: 6.8, temp: 24.8 },
    { day: 26, ec: 1.4, ph: 6.5, temp: 24.9 },
    { day: 27, ec: 1.6, ph: 6.9, temp: 25.0 },
    { day: 28, ec: 1.7, ph: 7.3, temp: 25.1 },
    { day: 29, ec: 1.5, ph: 7.7, temp: 25.2 },
    { day: 30, ec: 1.3, ph: 8.0, temp: 25.3 },
];
const MarketplacePage = () => {
    const [nfts, setNfts] = useState([]);
    const [loading, setLoading] = useState(true);
    const [growthDataModalOpenIndex, setGrowthDataModalOpenIndex] = useState(null);
    const { publicKey, wallet, sendTransaction } = useWallet();
    const { connection } = useConnection();

    const provider = new anchor.AnchorProvider(connection, window.solana, { preflightCommitment: "confirmed" });
    const program = new anchor.Program(idl, programId, provider);

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

    const handleBuy = async (nft) => {
        if (!publicKey || !connection) {
            alert("Please connect your wallet.");
            return;
        }
        try {
            // get ownerAddress
            let ownerAddress = undefined;
            if (nft.ownership && nft.ownership.owner) {
                ownerAddress = nft.ownership.owner;
            } else if (nft.owner) {
                ownerAddress = nft.owner;
            } else if (
                Array.isArray(nft.authorities) &&
                nft.authorities.length > 0
            ) {
                const auth0 = nft.authorities[0];
                if (auth0 && typeof auth0 === "object" && auth0.address) {
                    ownerAddress = auth0.address;
                } else if (typeof auth0 === "string") {
                    ownerAddress = auth0;
                }
            } else if (
                Array.isArray(nft.creators) &&
                nft.creators.length > 0 &&
                nft.creators[0] &&
                typeof nft.creators[0] === "object" &&
                nft.creators[0].address
            ) {
                ownerAddress = nft.creators[0].address;
            }

            if (typeof ownerAddress !== "string" || !ownerAddress) {
                alert("NFT owner address not found. NFT: " + JSON.stringify(nft));
                return;
            }

            try {
                new PublicKey(ownerAddress);
            } catch (e) {
                alert("Invalid ownerAddress: " + e.message);
                return;
            }

            // get mintAddress
            const mintAddress = nft.id;
            if (!mintAddress) {
                alert("NFT mint address not found.");
                return;
            }
            try {
                new PublicKey(mintAddress);
            } catch (e) {
                alert("Invalid mint address: " + e.message);
                return;
            }

            const seller = new PublicKey(ownerAddress);
            const lamports = Math.floor(0.00001 * LAMPORTS_PER_SOL);

            // get token accounts
            const nftMintPublicKey = new PublicKey(mintAddress);
            const sellerTokenAccount = await getAssociatedTokenAddress(
                nftMintPublicKey,
                seller
            );
            const buyerTokenAccount = await getAssociatedTokenAddress(
                nftMintPublicKey,
                publicKey
            );

      
            const buyerTokenAccountInfo = await connection.getAccountInfo(buyerTokenAccount);
            if (!buyerTokenAccountInfo) {
                const ataIx = createAssociatedTokenAccountInstruction(
                    provider.wallet.publicKey, // payer
                    buyerTokenAccount, // ata
                    provider.wallet.publicKey, // owner
                    nftMintPublicKey // mint
                );
                const tx = new anchor.web3.Transaction().add(ataIx);
                await provider.sendAndConfirm(tx, []);
            }
            // Use Anchor program to buy NFT
            await program.methods
                .buyNft(new anchor.BN(lamports))
                .accounts({
                    buyer: provider.wallet.publicKey,
                    seller: seller,
                    sellerNftAccount: sellerTokenAccount,
                    buyerNftAccount: buyerTokenAccount,
                    tokenMint: nftMintPublicKey,
                    tokenProgram: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                    systemProgram: SystemProgram.programId,
                })
                .rpc();

            alert("Purchase and transfer successful!");
        } catch (err) {
            if (err.logs) {
                console.error("Transaction logs:", err.logs);
            }
            console.error("Transaction error:", err);
            alert("Transaction failed: " + err.message);
        }
    }; 

    const [filterUser, setFilterUser] = useState("");
    const [filterCity, setFilterCity] = useState("");
    const [filterPlant, setFilterPlant] = useState("");
    const [videoUrl, setVideoUrl] = useState("");
    const [showVideo, setShowVideo] = useState(false);

    // City and plant mapping
    const cityMap = {
        Lon: "London",
        Par: "Paris",
        Ber: "Berlin",
        Rom: "Rome",
        Mad: "Madrid",
        Ott: "Ottawa",
        Was: "Washington",
        Tok: "Tokyo",
        Can: "Canberra",
        Mos: "Moscow",
        Bra: "Brasilia",
        Bei: "Beijing",
        Seo: "Seoul",
        Ban: "Bangkok",
        New: "New Delhi",
        Cai: "Cairo",
        Bue: "Buenos Aires",
        Wel: "Wellington",
        Osl: "Oslo",
        Sto: "Stockholm",
    };
    const plantMap = {
        Le: "Lettuce",
        To: "Tomato",
        Sp: "Spinach",
        Ka: "Kale",
        Be: "Bell Pepper",
        Ce: "Celery",
        Ca: "Cauliflower",
    };


    const cityOptions = Object.values(cityMap);
    const plantOptions = Object.values(plantMap);

    // Parse plant data from NFT name
    const parsePlantDataFromName = (name, publicKey) => {
        let grower = "N/A",
            city = "N/A",
            days = "N/A",
            plant = "N/A",
            ec = "N/A",
            ph = "N/A",
            temp = "N/A";
        if (name) {
            const parts = name.split("-");
            if (parts.length >= 7) {
                grower = parts[0] || "N/A";
                city = cityMap[parts[1]] || parts[1] || "N/A";
                days = parts[2] || "N/A";
                plant = plantMap[parts[3]] || parts[3] || "N/A";
                ec = parts[4] || "N/A";
                ph = parts[5] || "N/A";
                temp = parts[6].replace(/\.$/, "") || "N/A";
            }
        }
        if (publicKey) {
            const localNick = localStorage.getItem(`nickname_${publicKey}`);
            if (localNick) {
                grower = localNick.slice(0, 8);
            }
        }
        return {
            grower,
            city,
            days,
            plant,
            ec,
            ph,
            temp,
        };
    };

    const filteredNfts = nfts.filter((nft) => {
        const name = nft.content?.metadata?.name || "";
        const ownerKey = nft.ownership?.owner || nft.owner || "";
        const plantData = parsePlantDataFromName(name, ownerKey);

        const userMatch = filterUser ? plantData.grower.includes(filterUser) : true;
        const cityMatch = filterCity ? plantData.city === filterCity : true;
        const plantMatch = filterPlant ? plantData.plant === filterPlant : true;
        return userMatch && cityMatch && plantMatch;
    });

    return (
        <div className="container mx-auto px-4 py-8">
            <h2 className="text-3xl font-bold text-center mb-8">Marketplace</h2>
            {/* Filter bar */}
            <div className="flex flex-wrap gap-4 justify-center mb-6">
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
                    {cityOptions.map(city => (
                        <option key={city} value={city}>{city}</option>
                    ))}
                </select>
                <select
                    value={filterPlant}
                    onChange={e => setFilterPlant(e.target.value)}
                    className="border px-2 py-1 rounded"
                >
                    <option value="">All Plants</option>
                    {plantOptions.map(plant => (
                        <option key={plant} value={plant}>{plant}</option>
                    ))}
                </select>
            </div>
            <div className="flex flex-wrap justify-center gap-6">
                {loading && <p className="text-gray-600">Loading NFTs...</p>}
                {!loading && filteredNfts.length === 0 && (
                    <p className="text-gray-600">No NFTs match the filter.</p>
                )}
                {filteredNfts.map((nft, idx) => {

                    const name = nft.content?.metadata?.name || "";
                    const ownerKey = nft.ownership?.owner || nft.owner || "";
                    const plantData = parsePlantDataFromName(name, ownerKey);
                    const meta = nft.content?.metadata || {};
                    const token_image = meta.image || nft.content?.json_uri || "";
                    const nftId = nft.id || "N/A";
                    let creator = "N/A";
                    if (Array.isArray(nft.creators) && nft.creators.length > 0) {
                        creator = nft.creators[0]?.address || "N/A";
                    } else if (nft.creators?.address) {
                        creator = nft.creators.address;
                    }
                    const video = "https://www.youtube.com/embed/8G0zxQHHMqc";
                    // Filter logic for city and plant
                    if (filterCity && plantData.city !== filterCity) return null;
                    if (filterPlant && plantData.plant !== filterPlant) return null;
                    // Filter logic for username
                    if (filterUser && !plantData.grower.includes(filterUser)) return null;
                    return (
                        <div
                            key={nftId + idx}
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
                                {name}
                            </h3>
                            {token_image && (
                                <img
                                    src={token_image}
                                    alt={plantData.plant}
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
                                <div><strong>ID:</strong> {nftId}</div>
                                <div><strong>Owner:</strong> {ownerKey}</div>
                                <div><strong>Creator:</strong> {creator}</div>
                                <div><strong>Grower:</strong> {plantData.grower}</div>
                                <div><strong>City:</strong> {plantData.city}</div>
                                <div><strong>Grow Days:</strong> {plantData.days}</div>
                                <div><strong>Plant Name:</strong> {plantData.plant}</div>
                                <div><strong>EC:</strong> {plantData.ec}</div>
                                <div><strong>pH:</strong> {plantData.ph}</div>
                                <div><strong>Temperature:</strong> {plantData.temp === "N/A" ? "N/A" : `${plantData.temp}°C`}</div>
                            </div>
                            <button
                                style={{
                                    background: "#ff9800",
                                    color: "#fff",
                                    border: "none",
                                    borderRadius: "6px",
                                    padding: "6px 16px",
                                    cursor: "pointer",
                                    margin: "16px 0 8px 0",
                                }}
                                onClick={() => handleBuy(nft)}
                            >
                                Buy (0.00001 SOL)
                            </button>

                                <button
                                style={{
                                    background: "#2196F3",
                                    color: "#fff",
                                    border: "none",
                                    borderRadius: "6px",
                                    padding: "6px 16px",
                                    cursor: "pointer",
                                    marginBottom: "12px",
                                    marginTop: "8px"
                                }}
                                onClick={() => setGrowthDataModalOpenIndex(idx)}
                            >
                                View Growth Data
                            </button>
                            {growthDataModalOpenIndex === idx && (
                                <div
                                    style={{
                                        position: "fixed",
                                        top: 0,
                                        left: 0,
                                        width: "100vw",
                                        height: "100vh",
                                        background: "rgba(0,0,0,0.5)",
                                        display: "flex",
                                        alignItems: "center",
                                        justifyContent: "center",
                                        zIndex: 9999,
                                    }}
                                    onClick={() => setGrowthDataModalOpenIndex(null)}
                                >
                                    <div
                                        style={{
                                            background: "#fff",
                                            borderRadius: "12px",
                                            padding: "16px",
                                            position: "relative",
                                            minWidth: "320px",
                                            maxWidth: "90vw"
                                        }}
                                        onClick={(e) => e.stopPropagation()}
                                    >
                                        <button
                                            style={{
                                                position: "absolute",
                                                top: "8px",
                                                right: "12px",
                                                background: "transparent",
                                                border: "none",
                                                fontSize: "20px",
                                                cursor: "pointer",
                                            }}
                                            onClick={() => setGrowthDataModalOpenIndex(null)}
                                        >
                                            ×
                                        </button>
                                        <div style={{ width: "480px", maxWidth: "80vw" }}>
                                            <h3 style={{ marginBottom: "16px", fontWeight: "bold" }}>
                                                Time-laps Video
                                            </h3>
                                            <iframe
                                                width="100%"
                                                height="270"
                                                src="https://www.youtube.com/embed/8G0zxQHHMqc"
                                                title="Plant Growth Video"
                                                frameBorder="0"
                                                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                                                allowFullScreen
                                            ></iframe>
                                            <h3 style={{ margin: "24px 0 16px 0", fontWeight: "bold" }}>
                                                Growth Data
                                            </h3>
                                            <ResponsiveContainer width="100%" height={350}>
                                                <LineChart
                                                    data={fixedGrowthData}
                                                    margin={{ top: 5, right: 20, left: 0, bottom: 20 }}
                                                >
                                                    <CartesianGrid strokeDasharray="3 3" />
                                                    <XAxis dataKey="day" label={{ value: 'Day', position: 'insideBottom', offset: -15 }} />
                                                    <YAxis label={{ value: 'Value', angle: -90, position: 'insideLeft', offset: 10 }} />
                                                    <Tooltip />
                                                    <Legend verticalAlign="top" wrapperStyle={{ paddingBottom: '10px' }}/>
                                                    <Line type="monotone" dataKey="ec" stroke="#8884d8" activeDot={{ r: 6 }} name="EC" />
                                                    <Line type="monotone" dataKey="ph" stroke="#82ca9d" activeDot={{ r: 6 }} name="pH" />
                                                    <Line type="monotone" dataKey="temp" stroke="#ffc658" activeDot={{ r: 6 }} name="Temperature (°C)" />
                                                </LineChart>
                                            </ResponsiveContainer>
                                        </div>
                                    </div>
                                </div>
                            )}
                        </div>
                    );
                })}
            </div>
            {/* Video modal */}
            {showVideo && (
                <div
                    style={{
                        position: "fixed",
                        top: 0,
                        left: 0,
                        width: "100vw",
                        height: "100vh",
                        background: "rgba(0,0,0,0.6)",
                        display: "flex",
                        alignItems: "center",
                        justifyContent: "center",
                        zIndex: 9999,
                    }}
                    onClick={() => setShowVideo(false)}
                >
                    <div
                        style={{
                            background: "#fff",
                            borderRadius: "12px",
                            padding: "24px",
                            position: "relative",
                        }}
                        onClick={e => e.stopPropagation()}
                    >
                        <iframe
                            width="560"
                            height="315"
                            src={videoUrl}
                            title="YouTube video player"
                            frameBorder="0"
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowFullScreen
                        ></iframe>
                        <button
                            style={{
                                position: "absolute",
                                top: "8px",
                                right: "8px",
                                background: "#f44336",
                                color: "#fff",
                                border: "none",
                                borderRadius: "50%",
                                width: "32px",
                                height: "32px",
                                fontSize: "18px",
                                cursor: "pointer",
                            }}
                            onClick={() => setShowVideo(false)}
                        >
                            ×
                        </button>
                    </div>
                </div>
            )}
        </div>
    );
};



export default MarketplacePage;

