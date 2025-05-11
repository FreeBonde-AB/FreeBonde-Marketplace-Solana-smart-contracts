import React, { useState } from "react";
import PlantCard from "../PlantCard";
import { faker } from "@faker-js/faker";
import plantImage1 from "../../assets/plant_images/sample_plant_1.jpg";
import plantImage2 from "../../assets/plant_images/sample_plant_2.jpg";
import plantImage3 from "../../assets/plant_images/sample_plant_3.png";
import plantImage4 from "../../assets/plant_images/sample_plant_4.jpg";
import plantImage5 from "../../assets/plant_images/sample_plant_5.jpeg";

const stages = ["mature_plant", "sprout", "seed", "flowering", "fruiting"];

function getRandomInt(min, max) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
}
function getRandomFloat(min, max, decimals = 2) {
    return (Math.random() * (max - min) + min).toFixed(decimals);
}

const PLANT_IMAGES = [
    plantImage1,
    plantImage2,
    plantImage3,
    plantImage4,
    plantImage5,
];

function getRandomPlantImage() {
    const idx = Math.floor(Math.random() * PLANT_IMAGES.length);
    return PLANT_IMAGES[idx];
}

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
    const [plantData, setPlantData] = useState(null);

    const handleGenerate = () => {
        setPlantData(generatePlantData());
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
                >
                    Create Plant NFT
                </button>
            )}
        </div>
    );
};

export default Dashboard;
