import { useState, useEffect, useContext } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { Metaplex, walletAdapterIdentity } from "@metaplex-foundation/js";
import { Connection, clusterApiUrl, PublicKey } from "@solana/web3.js";
import { faker } from "@faker-js/faker";
import { toast } from "react-hot-toast";
import { useFreeBondeBalance } from "../../FreeBondeBalanceContext";
import PLANT_IMAGE_URLS from "../../assets/plant_images/plantImageUrls";

const stages = ["mature_plant", "sprout", "seed", "flowering", "fruiting"];

const getRandomInt = (min, max) => {
    return Math.floor(Math.random() * (max - min + 1)) + min;
};
const getRandomFloat = (min, max, decimals = 2) => {
    return (Math.random() * (max - min) + min).toFixed(decimals);
};

const Dashboard = () => {
    // States
    const [plantList, setPlantList] = useState([]);
    const [loading, setLoading] = useState(false);
    const [globalCity, setGlobalCity] = useState("");

    // Hooks
    const { freeBondeBalance, setFreeBondeBalance } = useFreeBondeBalance();
    const { publicKey } = useWallet();
    const wallet = useWallet();

    const generateFreeBonde = () => {
        const randomAmount = Math.floor(Math.random() * 51) + 50;
        setFreeBondeBalance((prevBalance) => prevBalance + randomAmount);
    };

    useEffect(() => {
        handleGenerate();
    }, []);

    const handleGenerate = () => {
        setLoading(true);
        setTimeout(() => {
            // City list
            const cities = [
                "London",
                "Paris",
                "Berlin",
                "Rome",
                "Madrid",
                "Ottawa",
                "Washington",
                "Tokyo",
                "Canberra",
                "Moscow",
                "Brasilia",
                "Beijing",
                "Seoul",
                "Bangkok",
                "New Delhi",
                "Cairo",
                "Buenos Aires",
                "Wellington",
                "Oslo",
                "Stockholm",
            ];
            // Randomly select a city, all plants use this city
            const city = cities[getRandomInt(0, cities.length - 1)];
            setGlobalCity(city);
            // Generate 5 plants, all use the same city
            const newPlants = Array.from({ length: 5 }, () =>
                generatePlantData(city)
            );
            setPlantList(newPlants);
            setLoading(false);
        }, 1000);
    };

    // generatePlantData now accepts city parameter
    const generatePlantData = (city) => {
        // Common western supermarket vegetables
        const plantNames = [
            "Lettuce",
            "Tomato",
            "Spinach",
            "Kale",
            "Bell Pepper",
            "Celery",
            "Cauliflower",
        ];

        const randomIndex = Math.floor(Math.random() * PLANT_IMAGE_URLS.length);
        const selectedImageUrl = PLANT_IMAGE_URLS[randomIndex];

        const plantName = plantNames[getRandomInt(0, plantNames.length - 1)];
        const growDays = getRandomInt(1, 365);

        return {
            plant_id: faker.string.nanoid(),
            plant_type: getRandomInt(1, 10),
            stage: stages[getRandomInt(0, stages.length - 1)],
            city: city, // Use the passed city
            plant_name: plantName,
            grow_days: growDays,
            data: {
                temperature: getRandomInt(15, 35), // Temperature range 15-35
                ph: getRandomFloat(4, 8),
                ec: getRandomFloat(1, 2),
                plant_image: selectedImageUrl,
            },
        };
    };

    const mintNFT = async (plantData) => {
        let nft = null;
        const loadingMintId = toast.loading("Creating NFT...");

        try {
            if (!plantData) {
                toast.error("Please generate plant data first.");
                return;
            }

            if (!wallet.connected) {
                toast.error("Please connect your wallet first.");
                return;
            }

            let nickname = "";
            if (publicKey) {
                nickname =
                    localStorage.getItem(`nickname_${publicKey}`) ||
                    `User_${publicKey.toString().slice(0, 6)}`;
            }

            // Generate name consistent with frontend parsing
            const shortNick = nickname.slice(0, 4);
            const shortCity = (plantData.city || "").slice(0, 3);
            const shortDays = String(plantData.grow_days)
                .padStart(2, "0")
                .slice(-2);
            const shortPlant = (plantData.plant_name || "").slice(0, 2);
            const shortEC = String(Number(plantData.data.ec).toFixed(1)).slice(
                0,
                3
            );
            const shortPH = String(Number(plantData.data.ph).toFixed(1)).slice(
                0,
                3
            );
            const shortTemp = String(plantData.data.temperature)
                .padStart(2, "0")
                .slice(-2);
            const now = new Date();
            const MM = String(now.getMonth() + 1).padStart(2, "0");
            const DD = String(now.getDate()).padStart(2, "0");
            const HH = String(now.getHours()).padStart(2, "0");
            const timeTag = `${MM}${DD}${HH}`;
            const nftName =
                `${shortNick}-${shortCity}-${shortDays}-${shortPlant}-${shortEC}-${shortPH}-${shortTemp}-${timeTag}`.slice(
                    0,
                    32
                );

            const connection = new Connection(clusterApiUrl("devnet"));
            const metaplex = new Metaplex(connection).use(
                walletAdapterIdentity(wallet)
            );

            const imageUrl = plantData.data.plant_image;

            const result = await metaplex.nfts().create({
                name: nftName,
                symbol: "FreeBonde",
                collection: new PublicKey(
                    "CjUBBjARbAP3zJMr97inXwfYa9uDfMC4QmFJ3QwUhMPj"
                ),
                uri: imageUrl,
                sellerFeeBasisPoints: 500,
                properties: {
                    files: [
                        {
                            uri: imageUrl,
                            type: "image/jpeg",
                        },
                    ],
                },
                attributes: [
                    {
                        trait_type: "Plant Type",
                        value: String(plantData.plant_type),
                    },
                    { trait_type: "Stage", value: plantData.stage },
                    {
                        trait_type: "Temperature",
                        value: String(plantData.data.temperature),
                    },
                    { trait_type: "pH", value: String(plantData.data.ph) },
                    { trait_type: "EC", value: String(plantData.data.ec) },
                    { trait_type: "Username", value: nickname },
                ],
            });

            nft = result.nft;

            if (nft) {
                toast.success(
                    `NFT minted successfully! Mint address: ${nft.address.toString()}`,
                    { id: loadingMintId }
                );
            }

            const loadingVerifyId = toast.loading("Verifying NFT...");

            // Now verify the collection
            const nftVerifyResponse = await metaplex.nfts().verifyCollection({
                mintAddress: nft.address,
                collectionMintAddress: new PublicKey(
                    "CjUBBjARbAP3zJMr97inXwfYa9uDfMC4QmFJ3QwUhMPj"
                ),
                isSizedCollection: true,
            });

            if (nftVerifyResponse) {
                toast.success("NFT Verified", { id: loadingVerifyId });
            }

            return nft;
        } catch (error) {
            // As long as nft exists, report success; otherwise, log error only
            if (nft && nft.address) {
                toast.success(
                    `NFT minted successfully! Mint address: ${nft.address.toString()}`,
                    { id: loadingMintId }
                );
            } else {
                toast.error("Error minting NFT", { id: loadingMintId });
            }
        }
    };

    return (
        <div className="dashboard-section-container">
            <div
                className="dashboard-buttons-container"
                style={{
                    display: "flex",
                    flexDirection: "column",
                    alignItems: "center",
                }}
            >
                <button onClick={generateFreeBonde} className={`flex items-center gap-2 mt-2 bg-amber-600 hover:bg-amber-700
                         px-4 py-2 rounded text-white font-semibold cursor-pointer`}>
                    Generate FreeBonde (TEST)
                </button>
                {/* Place the Update Plants Info button here */}
                <button
                    onClick={handleGenerate}
                    className={`flex items-center gap-2 mt-2 bg-green-600 hover:bg-green-700
                         px-4 py-2 rounded text-white font-semibold cursor-pointer`}
                >
                    Update Plants Info
                </button>
                {loading && <p style={{ color: "#888" }}>Loading...</p>}
                {/* Center all cards */}
                {!loading && plantList.length > 0 && (
                    <div
                        style={{
                            display: "flex",
                            flexWrap: "wrap",
                            gap: "24px",
                            justifyContent: "center",
                            margin: "24px 0",
                        }}
                    >
                        {plantList.map((plant, idx) => {
                            // Get nickname
                            let nickname = "";
                            if (publicKey) {
                                nickname =
                                    localStorage.getItem(
                                        `nickname_${publicKey}`
                                    ) || `N/A`;
                            } else {
                                nickname = "N/A";
                            }
                            return (
                                <div
                                    key={plant.plant_id}
                                    style={{
                                        margin: "8px",
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
                                    {/* Only show image */}
                                    <img
                                        src={plant.data.plant_image}
                                        alt={plant.plant_name}
                                        style={{
                                            width: "180px",
                                            height: "180px",
                                            objectFit: "cover",
                                            borderRadius: "8px",
                                            marginTop: "16px",
                                        }}
                                    />
                                    {/* New field info */}
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
                                            <strong>Grower:</strong> {nickname}
                                        </div>
                                        <div>
                                            <strong>City:</strong> {plant.city}
                                        </div>
                                        <div>
                                            <strong>Grow Days:</strong>{" "}
                                            {plant.grow_days}
                                        </div>
                                        <div>
                                            <strong>Plant Name:</strong>{" "}
                                            {plant.plant_name}
                                        </div>
                                        <div>
                                            <strong>EC:</strong> {plant.data.ec}
                                        </div>
                                        <div>
                                            <strong>pH:</strong> {plant.data.ph}
                                        </div>
                                        <div>
                                            <strong>Temperature:</strong>{" "}
                                            {plant.data.temperature}°C
                                        </div>
                                    </div>
                                    {/* Center the button */}
                                    <div
                                        style={{
                                            width: "100%",
                                            display: "flex",
                                            justifyContent: "center",
                                            // marginTop: "8px",
                                            marginBottom: "16px",
                                        }}
                                    >
                                        <button
                                            className="relative inline-block mt-4 px-6 py-3 border cursor-pointer border-neonGreen rounded-md uppercase font-bold tracking-widest 
  bg-yellow-400 hover:shadow-[0_0_15px_#39ff14] transition-all duration-300 
  before:absolute before:inset-0 before:rounded-md before:border before:border-neonGreen before:animate-pulse before:opacity-30 before:pointer-events-none"
                                            onClick={() => mintNFT(plant)}
                                            hidden={!wallet.connected}
                                        >
                                            Create Plant NFT
                                        </button>
                                    </div>
                                </div>
                            );
                        })}
                    </div>
                )}
                {!wallet.connected && (
                    <p className="connect-wallet-message">
                        Please connect your wallet first to mint the NFT.
                    </p>
                )}
            </div>
        </div>
    );
};

export default Dashboard;
