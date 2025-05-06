import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useWallet } from "@solana/wallet-adapter-react";

const WalletConnection = () => {
    const { connected } = useWallet();

    return (
        <div>
            <WalletMultiButton />
            {connected && <p>Wallet Connected!</p>}
        </div>
    );
};

export default WalletConnection;
