import { useState, useEffect, useContext } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { Metaplex, walletAdapterIdentity } from "@metaplex-foundation/js";
import { Connection, clusterApiUrl } from "@solana/web3.js";
import { faker } from "@faker-js/faker";
import { toast } from "react-hot-toast";
import PlantCard from "../PlantCard";
import { useFreeBondeBalance } from "../../FreeBondeBalanceContext";
import "./Dashboard.css"; // Import the CSS file for Dashboard styles
import PLANT_IMAGE_URLS from "../../assets/plant_images/plantImageUrls";
import { uploadToArweave, uploadMetadataToArweave } from "../../utils/arweave";

const stages = ["mature_plant", "sprout", "seed", "flowering", "fruiting"];

const getRandomInt = (min, max) => {
    return Math.floor(Math.random() * (max - min + 1)) + min;
};
const getRandomFloat = (min, max, decimals = 2) => {
    return (Math.random() * (max - min) + min).toFixed(decimals);
};




const Dashboard = () => {
 
    const [plantList, setPlantList] = useState([]);
    const [loading, setLoading] = useState(false);
    const [globalCity, setGlobalCity] = useState(""); // Global city state
    const { freeBondeBalance, setFreeBondeBalance } = useFreeBondeBalance();

    const generateFreeBonde = () => {
        const randomAmount = Math.floor(Math.random() * 51) + 50;
        setFreeBondeBalance(prevBalance => prevBalance + randomAmount);
    };

    useEffect(() => {
        handleGenerate();
    }, []);

    const { publicKey } = useWallet();
    const wallet = useWallet();


    const handleGenerate = () => {
        setLoading(true);
        setTimeout(() => {
            // City list
            const cities = [
                "London", "Paris", "Berlin", "Rome", "Madrid", "Ottawa", "Washington", "Tokyo", "Canberra", "Moscow",
                "Brasilia", "Beijing", "Seoul", "Bangkok", "New Delhi", "Cairo", "Buenos Aires", "Wellington", "Oslo", "Stockholm"
            ];
            // Randomly select a city, all plants use this city
            const city = cities[getRandomInt(0, cities.length - 1)];
            setGlobalCity(city);
            // Generate 5 plants, all use the same city
            const newPlants = Array.from({ length: 5 }, () => generatePlantData(city));
            setPlantList(newPlants);
            setLoading(false);
        }, 1000);
    };

    const uploadToArweave = (image) => {

        return "https://arweave.net/fake-image-url.jpg";
    };

    const uploadMetadataToArweave = async (metadata) => {

        return "https://arweave.net/fake-metadata-url.json";
    };


    // generatePlantData now accepts city parameter
    const generatePlantData = (city) => {
        // Common western supermarket vegetables
        const plantNames = [
            "Lettuce", "Tomato", "Spinach", "Kale", "Bell Pepper", "Celery", "Cauliflower"
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
                nickname = localStorage.getItem(`nickname_${publicKey}`) || `User_${publicKey.toString().slice(0, 6)}`;
            }

            // Generate name consistent with frontend parsing
            const shortNick = nickname.slice(0, 4);
            const shortCity = (plantData.city || "").slice(0, 3);
            const shortDays = String(plantData.grow_days).padStart(2, "0").slice(-2);
            const shortPlant = (plantData.plant_name || "").slice(0, 2);
            const shortEC = String(Number(plantData.data.ec).toFixed(1)).slice(0, 3);
            const shortPH = String(Number(plantData.data.ph).toFixed(1)).slice(0, 3);
            const shortTemp = String(plantData.data.temperature).padStart(2, "0").slice(-2);
            const now = new Date();
            const MM = String(now.getMonth() + 1).padStart(2, "0");
            const DD = String(now.getDate()).padStart(2, "0");
            const HH = String(now.getHours()).padStart(2, "0");
            const timeTag = `${MM}${DD}${HH}`;
            const nftName = `${shortNick}-${shortCity}-${shortDays}-${shortPlant}-${shortEC}-${shortPH}-${shortTemp}-${timeTag}`.slice(0, 32);

            const connection = new Connection(clusterApiUrl("devnet"));
            const metaplex = new Metaplex(connection).use(
                walletAdapterIdentity(wallet)
            );

            const imageUrl = plantData.data.plant_image;

            console.log("Creating NFT...");
            const result = await metaplex.nfts().create({
                name: nftName,
                symbol: "FreeBonde",
                uri: imageUrl,
                sellerFeeBasisPoints: 500,
                properties: {
                    files: [
                        {
                            uri: imageUrl,
                            type: "image/jpeg"
                        }
                    ]
                },
                attributes: [
                    { trait_type: "Plant Type", value: String(plantData.plant_type) },
                    { trait_type: "Stage", value: plantData.stage },
                    { trait_type: "Temperature", value: String(plantData.data.temperature) },
                    { trait_type: "pH", value: String(plantData.data.ph) },
                    { trait_type: "EC", value: String(plantData.data.ec) },
                    { trait_type: "Username", value: nickname }
                ]
            });
            nft = result.nft;

            console.log("NFT created successfully!");
            console.log("Mint Address:", nft.address.toString());

            toast.success(`NFT minted successfully! Mint address: ${nft.address.toString()}`);
            return nft;
        } catch (error) {
            // As long as nft exists, report success; otherwise, log error only
            if (nft && nft.address) {
                toast.success(`NFT minted successfully! Mint address: ${nft.address.toString()}`);
            }
            console.error("Error minting NFT:", error);
        }
    };


 return (
    <div className="dashboard-section-container">
      <div className="dashboard-buttons-container" style={{ display: "flex", flexDirection: "column", alignItems: "center" }}>
        <p>FreeBonde Balance: {freeBondeBalance}</p>
        <button onClick={generateFreeBonde} className="generate-button">
            Generate FreeBonde (For test ONLY!)
        </button>
        {/* Place the Update Plants Info button here */}
        <button onClick={handleGenerate} className="generate-button" style={{ marginTop: "12px" }}>
            Update Plants Info
        </button>
        {loading && <p style={{color: "#888"}}>Loading...</p>}
        {/* Center all cards */}
        {!loading && plantList.length > 0 && (
            <div style={{ display: "flex", flexWrap: "wrap", gap: "24px", justifyContent: "center", margin: "24px 0" }}>
                {plantList.map((plant, idx) => {
                    // Get nickname
                    let nickname = "";
                    if (publicKey) {
                        nickname = localStorage.getItem(`nickname_${publicKey}`) || `N/A`;
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
                                boxShadow: "0 4px 16px rgba(0,0,0,0.13), 0 1.5px 4px rgba(0,0,0,0.09)",
                                borderRadius: "12px",
                                background: "#fff"
                            }}
                        >
                            {/* Only show image */}
                            <img src={plant.data.plant_image} alt={plant.plant_name} style={{ width: "180px", height: "180px", objectFit: "cover", borderRadius: "8px", marginTop: "16px" }} />
                            {/* New field info */}
                            <div style={{ marginTop: "8px", background: "#f8f8f8", borderRadius: "8px", padding: "10px", textAlign: "center", width: "100%" }}>
                                <div><strong>Grower:</strong> {nickname}</div>
                                <div><strong>City:</strong> {plant.city}</div>
                                <div><strong>Grow Days:</strong> {plant.grow_days}</div>
                                <div><strong>Plant Name:</strong> {plant.plant_name}</div>
                                <div><strong>EC:</strong> {plant.data.ec}</div>
                                <div><strong>pH:</strong> {plant.data.ph}</div>
                                <div><strong>Temperature:</strong> {plant.data.temperature}°C</div>
                            </div>
                            {/* Center the button */}
                            <div style={{ width: "100%", display: "flex", justifyContent: "center", marginTop: "8px", marginBottom: "16px" }}>
                                <button
                                    className="mint-button"
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
            <p className="connect-wallet-message">Please connect your wallet first to mint the NFT.</p>
        )}
      </div>
    </div>
);
};

export default Dashboard;