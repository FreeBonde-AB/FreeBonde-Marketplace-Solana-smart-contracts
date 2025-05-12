import { FaWallet } from "react-icons/fa";
import { useWallet } from "@solana/wallet-adapter-react";
import { useWalletModal } from "@solana/wallet-adapter-react-ui";

const TopBar = () => {
    // Hooks
    const { connected } = useWallet();
    const { setVisible } = useWalletModal();

    return (
        <header className="flex items-center justify-between shadow h-16 bg-[#0b5345]">
            <h1 className="font-bold flex-1 px-4 text-white">FreeBonde NFT Minter</h1>
            <div className="flex items-center space-x-3 px-4 cursor-pointer">
                <button
                    className={`flex items-center gap-2 ${connected ? "bg-yellow-500" : "bg-gray-200 hover:bg-gray-300"} px-4 py-2 rounded text-black font-semibold`}
                    onClick={() => setVisible(true)}
                >
                    <FaWallet />
                    {connected ? "Wallet Connected" : "Connect Wallet"}
                </button>
            </div>
        </header>
    );
};

export default TopBar;
