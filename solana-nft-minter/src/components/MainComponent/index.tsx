import WalletConnection from "../WalletConnection";
import NFTMinter from "../NFTMinter";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import TopBar from "../TopBar";
import SideBar from "../SideBar";
import Dashboard from "../Dashboard";


const MainComponent = () => {
    return (
        <div>
            <h1>Solana NFT Minter</h1>
            <WalletConnection />
            <NFTMinter />
        </div>
    );
};

export default MainComponent;
