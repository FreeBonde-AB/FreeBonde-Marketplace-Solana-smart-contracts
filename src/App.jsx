import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import TopBar from "./components/TopBar";
import SideBar from "./components/SideBar";
import Dashboard from "./components/Dashboard";

const App = () => (
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

export default App;
