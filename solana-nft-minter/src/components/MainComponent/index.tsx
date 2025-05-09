import WalletConnection from "../WalletConnection";
import NFTMinter from "../NFTMinter";

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
