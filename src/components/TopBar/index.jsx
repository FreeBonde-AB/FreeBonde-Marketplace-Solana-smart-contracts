import React from "react";
import { FaWallet } from "react-icons/fa";

const TopBar = () => (
    <header className="flex items-center justify-between shadow h-16 bg-[#0b5345]">
        <h1 className="font-bold flex-1 px-4">Solana Plant NFT Minter</h1>
        <div className="flex items-center space-x-3 px-4 cursor-pointer">
            <button className="flex items-center gap-2 bg-green-600 hover:bg-green-700 px-4 py-2 rounded text-white font-semibold">
                <FaWallet />
                Connect Wallet
            </button>
        </div>
    </header>
);

export default TopBar;
