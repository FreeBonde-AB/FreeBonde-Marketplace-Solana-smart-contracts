import { useEffect, useState } from "react";
import { useWallet, useConnection } from "@solana/wallet-adapter-react";
import { Metaplex } from "@metaplex-foundation/js";
import { PublicKey } from "@solana/web3.js";
import toast from "react-hot-toast";
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

const MyNFTsPage = () => {
    // States
    const [nfts, setNfts] = useState([]);
    const [loading, setLoading] = useState(true);
    const [videoOpenIndex, setVideoOpenIndex] = useState(null);
    const [growthDataModalOpenIndex, setGrowthDataModalOpenIndex] = useState(null);
    const [detailModalIndex, setDetailModalIndex] = useState(null);
    
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


    // Hooks
    const { publicKey, connected } = useWallet();
    const { connection } = useConnection();

    const fetchNFTs = async (ownerPublicKey, solanaConnection) => {
        if (!ownerPublicKey || !solanaConnection) return;

        try {
            setLoading(true);
            const tokenAccounts =
                await solanaConnection.getParsedTokenAccountsByOwner(
                    ownerPublicKey,
                    {
                        programId: new PublicKey(
                            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                        ),
                    }
                );

            const metaplex = Metaplex.make(solanaConnection);

            const nftPromises = tokenAccounts.value
                .filter(
                    (account) =>
                        account.account.data.parsed.info.tokenAmount.amount ===
                            "1" &&
                        account.account.data.parsed.info.tokenAmount
                            .decimals === 0
                )
                .map(async (account) => {
                    try {
                        const mintAddress =
                            account.account.data.parsed.info.mint;
                        const nft = await metaplex
                            .nfts()
                            .findByMint({
                                mintAddress: new PublicKey(mintAddress),
                            });

                        if (nft.uri) {
                            const isImageUrl = /\.(jpeg|jpg|png|gif)$/i.test(
                                nft.uri
                            );

                            if (isImageUrl) {
                                nft.json = { image: nft.uri };
                            } else {
                                const response = await fetch(nft.uri);
                                const jsonMetadata = await response.json();
                                nft.json = jsonMetadata;
                            }
                        }

                        return nft;
                    } catch (innerError) {
                        toast.error(
                            `Error fetching NFT data: ${
                                innerError.message || "Unknown Error"
                            }`
                        );
                        setLoading(false);
                        return null;
                    }
                });

            let fetchedNfts = await Promise.all(nftPromises);

            fetchedNfts = fetchedNfts.map((nft) => {
                if (nft) {
                    nft.simulatedData = {
                        temperature: Math.floor(Math.random() * 30) + 15,
                        humidity: Math.floor(Math.random() * 50) + 30,
                        ph: parseFloat((Math.random() * 2 + 6).toFixed(1)),
                        ec: parseFloat((Math.random() * 0.5 + 1).toFixed(1)),
                        plant_type: Math.floor(Math.random() * 5) + 1,
                    };
                }
                return nft;
            });

            setNfts(
                fetchedNfts.filter((nft) => nft !== null && nft.json?.image)
            );
        } catch (error) {
            toast.error(
                `Error fetching NFTs: ${error.message || "Unknown Error"}`
            );
            setNfts([]);
            setLoading(false);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        if (connected && publicKey && connection) {
            fetchNFTs(publicKey, connection);
        } else {
            setNfts([]);
            setLoading(false);
        }
    }, [publicKey, connection, connected]);

    // Parse the last segment of name as time tag
    const parseTimeTag = (name) => {
        if (!name) return 0;
        const parts = name.split("-");
        if (parts.length < 8) return 0;
        // Convert MMDDHH format to number for sorting
        const tag = parts[7].replace(/\D/g, "");
        return Number(tag) || 0;
    };

    // Sort by time tag descending (newest NFT first)
    const sortedNfts = [...nfts].sort((a, b) => {
        const aTime = parseTimeTag(a.name);
        const bTime = parseTimeTag(b.name);
        return bTime - aTime;
    });

    return (
        <div className="container mx-auto px-4 py-8">
            <h2 className="text-3xl font-bold text-center mb-8">My NFTs</h2>
            <div className="flex flex-wrap justify-center gap-6">
                {loading && <p className="text-gray-600">Loading NFTs...</p>}
                {!loading && sortedNfts.length === 0 && publicKey && (
                    <p className="text-gray-600">
                        No NFTs found for this wallet.
                    </p>
                )}
                {!loading && sortedNfts.length === 0 && !publicKey && (
                    <p className="text-gray-600">
                        Connect your wallet to see your NFTs.
                    </p>
                )}
                {!loading &&
                    sortedNfts.length > 0 &&
                    sortedNfts.map((nft, idx) => {
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

                        let grower = "N/A",
                            city = "N/A",
                            days = "N/A",
                            plant = "N/A",
                            ec = "N/A",
                            ph = "N/A",
                            temp = "N/A";
                        if (nft.name) {
                            const parts = nft.name.split("-");
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
                        // Prefer local nickname if available
                        if (nft.address && publicKey) {
                            const localNick = localStorage.getItem(
                                `nickname_${publicKey}`
                            );
                            if (localNick) {
                                grower = localNick.slice(0, 8);
                            }
                        }
                        // Mapping city and plant
                        const cityFull = cityMap[city] || city;
                        const plantFull = plantMap[plant] || plant;

                        return (
                            <div
                                key={nft.address.toBase58()}
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
                                    {nft.name}
                                </h3>
                                {nft.json?.image && (
                                    <img
                                        src={nft.json.image}
                                        alt={nft.name}
                                        style={{
                                            width: "180px",
                                            height: "180px",
                                            objectFit: "cover",
                                            borderRadius: "8px",
                                            marginTop: "8px",
                                        }}
                                    />
                                )}
                                <div className="text-sm text-gray-700 mt-2">
                                    Mint:
                                </div>
                                <div className="text-sm text-gray-700 mt-2">
                                    {nft.address.toBase58()}
                                </div>

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
                                        <strong>Grower:</strong> {grower}
                                    </div>
                                    <div>
                                        <strong>City:</strong> {cityFull}
                                    </div>
                                    <div>
                                        <strong>Grow Days:</strong> {days}
                                    </div>
                                    <div>
                                        <strong>Plant Name:</strong> {plantFull}
                                    </div>
                                    <div>
                                        <strong>EC:</strong> {ec}
                                    </div>
                                    <div>
                                        <strong>pH:</strong> {ph}
                                    </div>
                                    <div>
                                        <strong>Temperature:</strong>{" "}
                                        {temp === "N/A" ? "N/A" : `${temp}°C`}
                                    </div>
                                </div>
                                <div style={{ height: "12px" }}></div>
 
                                {/* Growth Data Button */}
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
                                    onClick={() => setDetailModalIndex(idx)}
                                >
                                    View Growth Data
                                </button>
                                {detailModalIndex === idx && (
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
                                        onClick={() => setDetailModalIndex(null)}
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
                                                onClick={() => setDetailModalIndex(null)}
                                            >
                                                ×
                                            </button>
                                            <div style={{ width: "480px", maxWidth: "80vw" }}>
                                                <h3 style={{ marginBottom: "16px", fontWeight: "bold" }}>
                                                    Plant Growth Video
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
                                                    Growth Data Chart
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
        </div>
    );
};



export default MyNFTsPage;