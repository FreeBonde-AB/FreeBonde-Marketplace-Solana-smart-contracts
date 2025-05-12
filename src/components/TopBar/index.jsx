import { useState, useEffect } from 'react';
import { FaWallet } from 'react-icons/fa';
import { useWallet } from "@solana/wallet-adapter-react";
import { useWalletModal } from "@solana/wallet-adapter-react-ui";
import { useFreeBondeBalance } from '../../FreeBondeBalanceContext'; // Import useFreeBondeBalance

const TopBar = () => {
    const { freeBondeBalance, setFreeBondeBalance } = useFreeBondeBalance(); // Use the hook to get the balance

    const { connected } = useWallet();
    const { setVisible } = useWalletModal();

    useEffect(() => {
        const storedBalance = localStorage.getItem('freeBondeBalance');
        console.log('Initial freeBondeBalance from localStorage:', storedBalance);
        if (storedBalance) {
            setFreeBondeBalance(parseInt(storedBalance, 10));
        }

        const handleStorageChange = (event) => {
            if (event.key === 'freeBondeBalance' && event.newValue !== null) {
                console.log('Storage event for freeBondeBalance:', {
                    key: event.key,
 parseIntResult: parseInt(event.newValue, 10), // Add this line to log the parsed value
                    newValue: event.newValue,
                });
                setFreeBondeBalance(parseInt(event.newValue, 10)); // This line was already here
            }
        };

        window.addEventListener('storage', handleStorageChange);

        return () => {
            window.removeEventListener('storage', handleStorageChange);
        };
    }, [setFreeBondeBalance]);
    return (
        <header className="flex items-center justify-between shadow h-16 bg-[#0b5345]">
            <h1 className="font-bold flex-1 px-4 text-white">FreeBonde NFT Minter</h1>
            <div className="flex items-center px-4 text-white">
                <span>FreeBonde Balance: {freeBondeBalance}</span>
            </div>
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
