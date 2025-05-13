import { useState, useEffect } from "react";
import { FaWallet } from "react-icons/fa";
import { useWallet } from "@solana/wallet-adapter-react";
import { useWalletModal } from "@solana/wallet-adapter-react-ui";
import { useFreeBondeBalance } from "../../FreeBondeBalanceContext";
import USER_IMAGE_URLS from "../../assets/plant_images/user_imageURL";

const getAvatarIndex = (publicKey) => {
    if (!publicKey) return 0;

    let hash = 0;
    for (let i = 0; i < publicKey.length; i++) {
        hash = publicKey.charCodeAt(i) + ((hash << 5) - hash);
    }
    return Math.abs(hash) % USER_IMAGE_URLS.length;
};

const TopBar = () => {
    // States
    const [nickname, setNickname] = useState("");
    const [editing, setEditing] = useState(false);

    // Hooks
    const { freeBondeBalance, setFreeBondeBalance } = useFreeBondeBalance();
    const { connected, publicKey } = useWallet();
    const { setVisible } = useWalletModal();

    useEffect(() => {
        if (publicKey) {
            const saved = localStorage.getItem(`nickname_${publicKey}`);
            setNickname(saved || `user_${publicKey.toString().slice(0, 6)}`);
        }
    }, [publicKey]);

    const handleNicknameChange = (e) => setNickname(e.target.value);

    const handleNicknameSave = () => {
        if (publicKey) {
            localStorage.setItem(`nickname_${publicKey}`, nickname);
        }
        setEditing(false);
    };

    const avatarUrl = publicKey
        ? USER_IMAGE_URLS[getAvatarIndex(publicKey.toString())]
        : USER_IMAGE_URLS[0];

    useEffect(() => {
        const storedBalance = localStorage.getItem("freeBondeBalance");
        if (storedBalance) {
            setFreeBondeBalance(parseInt(storedBalance, 10));
        }
        const handleStorageChange = (event) => {
            if (event.key === "freeBondeBalance" && event.newValue !== null) {
                setFreeBondeBalance(parseInt(event.newValue, 10));
            }
        };
        window.addEventListener("storage", handleStorageChange);
        return () => {
            window.removeEventListener("storage", handleStorageChange);
        };
    }, [setFreeBondeBalance]);

    return (
        <header className="flex items-center justify-between shadow h-16 bg-[#0b5345]">
            <h1 className="font-bold flex-1 px-4 text-white">
                FreeBonde NFT Minter
            </h1>
            <div className="flex items-center px-4 text-white">
                <span>FreeBonde Balance: {freeBondeBalance}</span>
            </div>
            <div className="flex items-center space-x-3 px-4 cursor-pointer">
                {/* Icon & Username */}
                {connected && (
                    <div className="flex items-center space-x-2">
                        <img
                            src={avatarUrl}
                            alt="avatar"
                            className="w-10 h-10 rounded-full border-2 border-white"
                        />
                        {editing ? (
                            <div className="flex items-center">
                                <input
                                    className="rounded px-2 py-1 text-black"
                                    value={nickname}
                                    onChange={handleNicknameChange}
                                    onBlur={handleNicknameSave}
                                    onKeyDown={(e) => {
                                        if (e.key === "Enter")
                                            handleNicknameSave();
                                    }}
                                    autoFocus
                                />
                                <button
                                    className="ml-1 text-xs text-yellow-300"
                                    onClick={handleNicknameSave}
                                >
                                    Save
                                </button>
                            </div>
                        ) : (
                            <span
                                className="font-semibold text-white"
                                onClick={() => setEditing(true)}
                                title="Change Nickname"
                                style={{ cursor: "pointer" }}
                            >
                                {nickname}
                            </span>
                        )}
                    </div>
                )}
                <button
                    className={`flex items-center gap-2 ${
                        connected
                            ? "bg-green-600 hover:bg-green-700"
                            : "bg-amber-600 hover:bg-amber-700"
                    } px-4 py-2 rounded text-white font-semibold cursor-pointer`}
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
