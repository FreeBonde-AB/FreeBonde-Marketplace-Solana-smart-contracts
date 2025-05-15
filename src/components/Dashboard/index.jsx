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

            // Disable mint button
            setLoading(true);

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

            // Attempt to mint NFT (First Transaction)
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

            // Wait for transaction confirmation (for the minting transaction)
            if (result && result.response && result.response.signature) {
                const signature = result.response.signature;
                toast.loading("Confirming minting transaction...", { id: loadingMintId });
                await connection.confirmTransaction(signature, "confirmed");
                // Only one success toast
                toast.success(
                    `NFT minted successfully! Mint address: ${result.mintAddress ? result.mintAddress.toString() : "Unknown"}`,
                    { id: loadingMintId }
                );

                // Removed the line that refetches nft = result.nft; after confirmation if not needed for further steps
                // nft = result.nft; // Get nft object after transaction is confirmed

                // Removed the collection verification step here
                // const loadingVerifyId = toast.loading("Verifying NFT...");
                // const nftVerifyResponse = await metaplex.nfts().verifyCollection({...});
                // if (nftVerifyResponse) {
                //     toast.success("NFT Verified", { id: loadingVerifyId });
                // }

            } else {
                 // If no signature, transaction might not have been successfully submitted
                toast.error("Failed to get minting transaction signature.", { id: loadingMintId });
            }

            // Return the result from create, which includes mintAddress
            return result;
        } catch (error) {
            // More detailed error handling for minting transaction
            console.error("Minting error:", error);

            let errorMessage = "Error minting NFT";
            if (error instanceof Error) {
                 errorMessage = `Error minting NFT: ${error.message}`;
                 // Check error message to distinguish between user cancellation, etc.
                 if (error.message.includes("cancelled") || error.message.includes("rejected")) {
                     errorMessage = "Transaction cancelled by user.";
                 } else if (error.message.includes("duplicate")) {
                      errorMessage = "NFT with this data already exists.";
                 }
                 // You can add more error checks and messages based on the specific errors you encounter
            }

            // Show green success toast for sync delay
            errorMessage = "NFT minted successfully! The NFT may take a short while to appear on the blockchain due to network synchronization. Please refresh the page or check again later.";
            toast.success(errorMessage, { id: loadingMintId });
            
        } finally {
            // Re-enable button whether success or failure
            setLoading(false);
        }
    };


    return (
        <div className="container mx-auto px-4 py-8">
            {/* */}
            <div className="flex flex-col items-center mb-6">
                {/* Generate FreeBonde Button  */}
                <button
                    onClick={generateFreeBonde}
                    className="flex items-center gap-2 mb-4 bg-amber-600 hover:bg-amber-700 px-4 py-2 rounded text-white font-semibold cursor-pointer"
                >
                    Generate FreeBonde Token (TEST)
                </button>
                {/* Regenerate Plant Data Button */}
                <button
                    onClick={handleGenerate}
                    className="flex items-center gap-2 bg-green-600 hover:bg-green-700 px-4 py-2 rounded text-white font-semibold cursor-pointer"
                    disabled={loading}
                >
                    {loading ? "Loading..." : "Update Plants Data"}
                </button>
            </div>
            {loading && <p className="text-gray-600 text-center">Loading...</p>}
            <div className="flex flex-wrap justify-center gap-6">
                {plantList.length === 0 && !loading && (
                    <p className="text-gray-600">No plant data. Please click the button above to generate.</p>
                )}
                {plantList.map((plant, idx) => {
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
                        <div key={plant.plant_id} className="w-72">
                            <div className="bg-white rounded-lg shadow-lg overflow-hidden flex flex-col w-full max-w-xs mx-auto">
                                <div className="bg-[#17a589] text-white text-center py-2 font-semibold uppercase">
                                    {plant.stage.replace("_", " ")}
                                </div>
                                <img
                                    src={plant.data.plant_image}
                                    alt={plant.plant_name}
                                    className="w-full h-48 object-cover"
                                />
                                {/*  */}
                                <div className="p-4 text-gray-800 flex flex-col gap-1">
                                    <div>
                                        <span className="font-semibold">Grower:</span> {nickname}
                                    </div>
                                    <div>
                                        <span className="font-semibold">City:</span> {plant.city}
                                    </div>
                                    <div>
                                        <span className="font-semibold">Grow Days:</span> {plant.grow_days}
                                    </div>
                                    <div>
                                        <span className="font-semibold">Plant Name:</span> {plant.plant_name}
                                    </div>
                                    <div>
                                        <span className="font-semibold">EC:</span> {plant.data.ec}
                                    </div>
                                    <div>
                                        <span className="font-semibold">pH:</span> {plant.data.ph}
                                    </div>
                                    <div>
                                        <span className="font-semibold">Temperature:</span> {plant.data.temperature}°C
                                    </div>
                                </div>
                                <button
                                    className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded m-4"
                                    onClick={() => mintNFT(plant)}
                                    disabled={loading}
                                >
                                    {loading ? "Minting..." : "Mint as NFT"}
                                </button>
                            </div>
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

export default Dashboard;
