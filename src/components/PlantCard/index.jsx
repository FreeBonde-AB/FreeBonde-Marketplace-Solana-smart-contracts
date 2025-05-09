import React from "react";

const PlantCard = ({ data }) => (
    <div className="bg-white rounded-lg shadow-lg overflow-hidden flex flex-col w-full max-w-xs mx-auto">
        <div className="bg-[#17a589] text-white text-center py-2 font-semibold uppercase">
            {data.stage.replace("_", " ")}
        </div>
        <img
            src={data.data.plant_image}
            alt="Plant"
            className="w-full h-48 object-cover"
        />
        <div className="p-4 grid grid-cols-2 gap-2 text-gray-800">
            <div>
                <span className="font-semibold">Temp:</span>{" "}
                {data.data.temperature}°F
            </div>
            <div>
                <span className="font-semibold">Humidity:</span>{" "}
                {data.data.humidity}%
            </div>
            <div>
                <span className="font-semibold">pH:</span> {data.data.ph}
            </div>
            <div>
                <span className="font-semibold">EC:</span> {data.data.ec}
            </div>
        </div>
    </div>
);

export default PlantCard;
