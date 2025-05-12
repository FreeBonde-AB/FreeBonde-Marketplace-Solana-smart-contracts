import { useState, useEffect } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { Metaplex, walletAdapterIdentity } from "@metaplex-foundation/js";
import { Connection, clusterApiUrl } from "@solana/web3.js";
import { faker } from "@faker-js/faker";
import { toast } from "react-hot-toast";
import PlantCard from "../PlantCard";
import plantImage1 from "../../assets/plant_images/sample_plant_1.jpg";
import plantImage2 from "../../assets/plant_images/sample_plant_2.jpg";
import plantImage3 from "../../assets/plant_images/sample_plant_3.png";
import plantImage4 from "../../assets/plant_images/sample_plant_4.jpg";
import plantImage5 from "../../assets/plant_images/sample_plant_5.jpeg";
import "./Dashboard.css"; // Import the CSS file for Dashboard styles

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


const Dashboard = () => {
    // States
    const [plantData, setPlantData] = useState(null);

    // Hooks
    const { publicKey } = useWallet();
    const wallet = useWallet();

    // Automatically generate data on component mount
    useEffect(() => {
        handleGenerate();
    }, []); // Empty dependency array means this effect runs only once after the initial render

    const handleGenerate = () => {
        setPlantData(generatePlantData());
    };

    const uploadToArweave = (image) => {
        return image;
    };

    const uploadMetadataToArweave = async (metadata) => {};

    const LEONARDO_IMAGES = [
        "https://cdn.leonardo.ai/users/577d5d05-5f18-4951-b4b9-ad7cfaa975f9/generations/6d147b2a-36a3-4043-a58b-92c15ec8a669/segments/1:4:1/Flux_Schnell_exquisite_high_fashion_photography_of_Kale_Sprout_0.jpeg",
        "https://cdn.leonardo.ai/users/577d5d05-5f18-4951-b4b9-ad7cfaa975f9/generations/2d54ecf1-046a-49fc-bf03-e3964432a685/segments/3:4:1/Flux_Schnell_exquisite_high_fashion_photography_of_Baby_Bok_Ch_2.jpeg",
        "https://cdn.leonardo.ai/users/577d5d05-5f18-4951-b4b9-ad7cfaa975f9/generations/eacbae2a-4208-480c-8b72-0efbad1cf0b8/segments/2:4:1/Flux_Schnell_A_sleek_highfashion_photograph_showcasing_a_beaut_1.jpeg",
        "https://cdn.leonardo.ai/users/577d5d05-5f18-4951-b4b9-ad7cfaa975f9/generations/c25b91ef-c127-4ffc-8521-d87c7988258c/segments/2:4:1/Flux_Dev_Macro_photograph_capturing_the_intricacies_of_sunflow_1.jpeg",
        "https://cdn.leonardo.ai/users/577d5d05-5f18-4951-b4b9-ad7cfaa975f9/generations/c25b91ef-c127-4ffc-8521-d87c7988258c/segments/4:4:1/Flux_Dev_Macro_photograph_capturing_the_intricacies_of_sunflow_3.jpeg",
        "https://cdn.leonardo.ai/users/577d5d05-5f18-4951-b4b9-ad7cfaa975f9/generations/7556628d-6aba-4891-ba0d-c453348ae07d/Leonardo_Phoenix_Please_provide_a_photo_of_a_sleek_blackgray_v_3.jpg",
        "https://cdn.leonardo.ai/users/577d5d05-5f18-4951-b4b9-ad7cfaa975f9/generations/7c31d5e0-14c5-494b-91d6-dc8bc2661f20/Leonardo_Phoenix_Please_provide_a_photo_of_a_sleek_blackgray_v_1.jpg"
    ];

    const getRandomLeonardoImage = () => {
        const idx = Math.floor(Math.random() * LEONARDO_IMAGES.length);
        return LEONARDO_IMAGES[idx];
    };

    const generatePlantData = () => {
        const selectedImageUrl = getRandomLeonardoImage(); // Select image here
        return {
            plant_id: faker.string.nanoid(),
            plant_type: getRandomInt(1, 10),
            stage: stages[getRandomInt(0, stages.length - 1)],
            data: {
                temperature: getRandomInt(20, 90),
                humidity: getRandomInt(50, 80),
                ph: getRandomFloat(1, 10),
                ec: getRandomFloat(1, 2),
                plant_image: selectedImageUrl, // Use the selected image URL
            },
        };
    };

    const mintNFT = async () => {
        try {
            if (!plantData) {
                toast.error("Please generate plant data first.");
                return;
            }

            if (!wallet.connected) {
                toast.error("Please connect your wallet first.");
                return;
            }

            const connection = new Connection(clusterApiUrl("devnet"));
            const metaplex = new Metaplex(connection).use(
                walletAdapterIdentity(wallet)
            );

            // Use the image URL generated with plant data
            const imageUrl = plantData.data.plant_image;

            // 创建 NFT
            console.log("Creating NFT...");
            const { nft } = await metaplex.nfts().create({
                name: `Plant #${plantData.plant_id}`,
                // Change the symbol to "FreeBonde"
                symbol: "FreeBonde",
                uri: imageUrl, // Use the selected image URL for URI
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
                    {
                        trait_type: "Plant Type",
                        value: String(plantData.plant_type)
                    },
                    {
                        trait_type: "Stage",
                        value: plantData.stage
                    },
                    {
                        trait_type: "Temperature",
                        value: String(plantData.data.temperature)
                    },
                    {
                        trait_type: "Humidity",
                        value: String(plantData.data.humidity)
                    },
                    {
                        trait_type: "pH",
                        value: String(plantData.data.ph)
                    },
                    {
                        trait_type: "EC",
                        value: String(plantData.data.ec)
                    }
                ]
            });

            console.log("NFT created successfully!");
            console.log("Mint Address:", nft.address.toString());

            toast.success(`NFT minted successfully! Mint address: ${nft.address.toString()}`);
            return nft;
        } catch (error) {
            toast.error(
                `Error minting NFT: ${error.message || error.toString()}`
            );
            console.error("Error minting NFT:", error);
        }
    };

 return (
    <div className="dashboard-section-container"> {/* Container for the Dashboard section */}
      <div className="dashboard-buttons-container"> {/* Container for all content */}
          {plantData && <PlantCard data={plantData} />}

          <button onClick={handleGenerate} className="generate-button">
              Generate Plant Data
            </button>
          {plantData && (
              <button className="mint-button" onClick={mintNFT} hidden={!wallet.connected}>
                  Create Plant NFT
              </button>
          )}
          {!wallet.connected && (
              <p className="connect-wallet-message">Please connect your wallet first to mint the NFT.</p>
          )}
      </div>
    </div>
  );
};

export default Dashboard;

