import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import {
    ConnectionProvider,
    WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-wallets";
import { clusterApiUrl } from "@solana/web3.js";
import TopBar from "./components/TopBar";
import SideBar from "./components/SideBar";
import Dashboard from "./components/Dashboard";
import { FreeBondeBalanceProvider } from "./FreeBondeBalanceContext";
import MyNFTsPage from "./pages/MyNFTsPage";
import MarketplacePage from "./pages/MarketplacePage";
import "@solana/wallet-adapter-react-ui/styles.css";
import { Toaster } from "react-hot-toast";

const App = () => {
    const endpoint = clusterApiUrl("devnet");
    const wallets = [new PhantomWalletAdapter()];

 return (
    <>
      <Toaster position="top-right" />
      <Router>
        <ConnectionProvider endpoint={endpoint}>
          <WalletProvider wallets={wallets} autoConnect>
            <WalletModalProvider>
              <FreeBondeBalanceProvider>
                <div className="flex flex-col min-h-screen w-screen">
                  <TopBar />
                  <div className="flex flex-1 bg-gray-100">
                    <SideBar />
                    <main className="flex-1 p-6">
                      <Routes>
                        <Route path="/" element={<Dashboard />} />
                        <Route path="/my-nfts" element={<MyNFTsPage />} />
                        <Route path="/marketplace" element={<MarketplacePage />} />
                      </Routes>
                    </main>
                  </div>
                </div>
              </FreeBondeBalanceProvider>
            </WalletModalProvider>
          </WalletProvider>
        </ConnectionProvider>
      </Router>
        </>
    );
};

export default App;
