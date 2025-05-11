import { useState } from "react";
import { FaBars, FaHome } from "react-icons/fa";
import { useNavigate } from "react-router-dom";

const SideBar = () => {
    // States
    const [collapsed, setCollapsed] = useState(false);

    // Router
    const navigate = useNavigate();

    return (
        <div
            className={`h-screen bg-[#138d75] text-white transition-all duration-300 flex flex-col ${
                collapsed ? "w-16" : "w-56"
            }`}
        >
            {/* Collapse/Expand Button */}
            <button
                className="p-4 focus:outline-none hover:bg-gray-700 transition"
                onClick={() => setCollapsed((prev) => !prev)}
                aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
            >
                <FaBars />
            </button>

            {/* Sidebar Content */}
            <nav className="flex-1 flex flex-col items-center">
                <button
                    className={`flex items-center w-full px-4 py-3 text-lg font-medium transition hover:bg-gray-700 ${
                        collapsed ? "justify-center" : ""
                    }`}
                    onClick={() => navigate("/")}
                >
                    <FaHome className="text-xl" />
                    {!collapsed && <span className="ml-4">Dashboard</span>}
                </button>
            </nav>
        </div>
    );
};

export default SideBar;
