const PlantCard = (props) => {
    // Props
    const { data, nickname } = props;

    return (
        <div key={data?.plant_id} className="bg-white rounded-lg shadow-lg overflow-hidden flex flex-col w-full max-w-xs mx-auto">
            <div className="bg-[#17a589] text-white text-center py-2 font-semibold uppercase">
                {data.stage.replace("_", " ")}
            </div>
            <img
                src={data?.plant_image}
                alt={data?.plant_name}
                className="w-full h-48 object-cover"
            />
            <div className="p-4 grid grid-cols-2 gap-2 text-gray-800">
                <div>
                    <span className="font-semibold">Grower:</span>{" "}
                    {nickname}
                </div>
                <div>
                    <span className="font-semibold">City:</span>{" "}
                    {data?.city}
                </div>
                <div>
                    <span className="font-semibold">Grow Days:</span> {data?.grow_days}
                </div>
                <div>
                    <span className="font-semibold">Plant Name:</span> {data?.plant_name}
                </div>
                <div>
                    <span className="font-semibold">EC:</span> {data?.ec}
                </div>
                <div>
                    <span className="font-semibold">pH:</span> {data?.ph}
                </div>
                <div>
                    <span className="font-semibold">Temperature:</span> {data?.temperature}°C
                </div>
            </div>
        </div>
    );
};

export default PlantCard;
