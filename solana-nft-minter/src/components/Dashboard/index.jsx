import { useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { Metaplex } from "@metaplex-foundation/js";
import { Connection, clusterApiUrl } from "@solana/web3.js";
import { faker } from "@faker-js/faker";
import PlantCard from "../PlantCard";
import plantImage1 from "../../assets/plant_images/sample_plant_1.jpg";
import plantImage2 from "../../assets/plant_images/sample_plant_2.jpg";
import plantImage3 from "../../assets/plant_images/sample_plant_3.png";
import plantImage4 from "../../assets/plant_images/sample_plant_4.jpg";
import plantImage5 from "../../assets/plant_images/sample_plant_5.jpeg";

const stages = ["mature_plant", "sprout", "seed", "flowering", "fruiting"];

const getRandomInt = (min, max) => {
    return Math.floor(Math.random() * (max - min + 1)) + min;
};
const getRandomFloat = (min, max, decimals = 2) => {
    return (Math.random() * (max - min) + min).toFixed(decimals);
};

const PLANT_IMAGES = [
    plantImage1,
    plantImage2,
    plantImage3,
    plantImage4,
    plantImage5,
];

const getRandomPlantImage = () => {
    const idx = Math.floor(Math.random() * PLANT_IMAGES.length);
    return PLANT_IMAGES[idx];
};

const generatePlantData = () => ({
    plant_id: faker.string.nanoid(),
    plant_type: getRandomInt(1, 10),
    stage: stages[getRandomInt(0, stages.length - 1)],
    data: {
        temperature: getRandomInt(20, 90),
        humidity: getRandomInt(50, 80),
        ph: getRandomFloat(1, 10),
        ec: getRandomFloat(1, 2),
        plant_image: getRandomPlantImage(),
    },
});

const Dashboard = () => {
    // States
    const [plantData, setPlantData] = useState(null);

    // Hooks
    const { publicKey } = useWallet();

    const handleGenerate = () => {
        setPlantData(generatePlantData());
    };

    const uploadToArweave = (image) => {
        return image;
    };

    const uploadMetadataToArweave = async (metadata) => {};

    const mintNFT = async () => {
        try {
            if (!plantData) {
                throw new Error("No plant data available!");
            }

            const connection = new Connection(clusterApiUrl("devnet"));
            const metaplex = new Metaplex(connection);

            // Upload image to Arweave
            const imageUrl = uploadToArweave(plantData.data.plant_image);

            // 2. Prepare metadata object
            const metadata = {
                name: `Plant #${plantData.plant_id}`,
                symbol: "PLANT",
                description: `A unique plant NFT at the ${plantData.stage} stage.`,
                image: imageUrl,
                attributes: [
                    { trait_type: "Plant Type", value: plantData.plant_type },
                    { trait_type: "Stage", value: plantData.stage },
                    {
                        trait_type: "Temperature",
                        value: plantData.data.temperature,
                    },
                    { trait_type: "Humidity", value: plantData.data.humidity },
                    { trait_type: "pH", value: plantData.data.ph },
                    { trait_type: "EC", value: plantData.data.ec },
                ],
                properties: {
                    files: [
                        {
                            uri: imageUrl,
                            type: "image/jpeg", // or "image/png" depending on your image
                        },
                    ],
                    category: "image",
                    creators: [
                        {
                            address: publicKey?.toBase58() || "",
                            share: 100,
                        },
                    ],
                },
            };

            // 3. Upload metadata JSON to Arweave
            // const metadataUrl = await uploadMetadataToArweave(metadata);

            // Create NFT metadata
            const { nft } = await metaplex.nfts().create({
                uri: `${imageUrl}`,
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
        <div className="flex flex-col items-center w-full py-8">
            <button
                onClick={handleGenerate}
                className="mb-8 bg-green-600 hover:bg-green-700 text-white px-8 py-3 rounded-lg font-semibold text-lg shadow"
            >
                Generate Plant Data
            </button>
            {plantData && (
                <div className="w-full flex justify-center">
                    <PlantCard data={plantData} />
                </div>
            )}
            {plantData && (
                <button
                    className="relative inline-block mt-4 px-6 py-3 border border-neonGreen rounded-md uppercase font-bold tracking-widest 
  bg-black hover:shadow-[0_0_15px_#39ff14] transition-all duration-300 
  before:absolute before:inset-0 before:rounded-md before:border before:border-neonGreen before:animate-pulse before:opacity-30 before:pointer-events-none"
                    onClick={mintNFT}
                    disabled={!publicKey}
                >
                    Create Plant NFT
                </button>
            )}
        </div>
    );
};

export default Dashboard;
