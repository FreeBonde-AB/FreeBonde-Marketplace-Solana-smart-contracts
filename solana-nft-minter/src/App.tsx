import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import {
    ConnectionProvider,
    WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-wallets";
import { clusterApiUrl } from "@solana/web3.js";
import MainComponent from "./components/MainComponent";
import TopBar from "./components/TopBar";
import SideBar from "./components/SideBar";
import Dashboard from "./components/Dashboard";

const App = () => {
    // const endpoint = clusterApiUrl("devnet");
    // const wallets = [new PhantomWalletAdapter()];

    return (
        // <>
        //     <ConnectionProvider endpoint={endpoint}>
        //         <WalletProvider wallets={wallets} autoConnect>
        //             <WalletModalProvider>
        //                 <MainComponent />
        //             </WalletModalProvider>
        //         </WalletProvider>
        //     </ConnectionProvider>
        // </>

        <Router>
            <div className="flex flex-col min-h-screen w-screen">
                <TopBar />
                <div className="flex flex-1 bg-gray-100">
                    <SideBar />
                    <main className="flex-1 p-6">
                        <Routes>
                            <Route path="/" element={<Dashboard />} />
                        </Routes>
                    </main>
                </div>
            </div>
        </Router>
    );
};

export default App;
