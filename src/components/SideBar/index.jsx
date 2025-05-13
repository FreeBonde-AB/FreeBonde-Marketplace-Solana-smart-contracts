import { useState } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import { FaBars } from "react-icons/fa";
import { MdOutlineSpaceDashboard } from "react-icons/md";
import { TbPlant } from "react-icons/tb";
import { MdOutlineShoppingCart } from "react-icons/md";

const SideBar = () => {
    // States
    const [collapsed, setCollapsed] = useState(false);

    // Hooks
    const navigate = useNavigate();
    const location = useLocation();

    // Helper to check if a path is active
    const isActive = (path) => location.pathname === path;

    return (
        <div
            className={`overflow-y-auto bg-[#138d75] text-white transition-all duration-300 flex flex-col ${
                collapsed ? "w-16" : "w-56"
            }`}
        >
            {/* Collapse/Expand Button */}
            <button
                className="p-4 focus:outline-none hover:bg-gray-700 transition cursor-pointer"
                onClick={() => setCollapsed((prev) => !prev)}
                aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
            >
                <FaBars />
            </button>

            {/* Sidebar Content */}
            <nav className="flex-1 flex flex-col items-center">
                <button
                    className={`flex items-center w-full px-4 py-3 cursor-pointer text-lg font-medium transition hover:bg-gray-700 ${
                        collapsed ? "justify-center" : ""
                    } ${isActive("/") ? "bg-gray-700" : ""}`}
                    onClick={() => navigate("/")}
                >
                    <MdOutlineSpaceDashboard className="text-xl" />
                    {!collapsed && <span className="ml-4">Dashboard</span>}
                </button>
                <button
                    className={`flex items-center w-full px-4 py-3 cursor-pointer text-lg font-medium transition hover:bg-gray-700 ${
                        collapsed ? "justify-center" : ""
                    } ${isActive("/my-nfts") ? "bg-gray-700" : ""}`}
                    onClick={() => navigate("/my-nfts")}
                >
                    <TbPlant className="text-xl" />
                    {!collapsed && <span className="ml-4">My NFTs</span>}
                </button>
                <button
                    className={`flex items-center w-full px-4 py-3 cursor-pointer text-lg font-medium transition hover:bg-gray-700 ${
                        collapsed ? "justify-center" : ""
                    } ${isActive("/marketplace") ? "bg-gray-700" : ""}`}
                    onClick={() => navigate("/marketplace")}
                >
                    <MdOutlineShoppingCart className="text-xl" />
                    {!collapsed && <span className="ml-4">Marketplace</span>}
                </button>
            </nav>
        </div>
    );
};

export default SideBar;
