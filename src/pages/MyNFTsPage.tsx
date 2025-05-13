import React, { useEffect, useState } from 'react';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { getTokenAccountsByOwner, AccountLayout } from '@solana/spl-token';
import { Metaplex } from '@metaplex-foundation/js';
import { Connection, PublicKey } from '@solana/web3.js';

const MyNFTsPage = () => {
  const { publicKey, connected } = useWallet();
  const { connection } = useConnection();
  const [nfts, setNfts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [videoOpenIndex, setVideoOpenIndex] = useState(null);

  const fetchNFTs = async (ownerPublicKey, solanaConnection) => {
    if (!ownerPublicKey || !solanaConnection) return;

    try {
      setLoading(true);
      const tokenAccounts = await solanaConnection.getParsedTokenAccountsByOwner(
        ownerPublicKey,
        {
          programId: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
        }
      );

      const metaplex = Metaplex.make(solanaConnection);

      const nftPromises = tokenAccounts.value
        .filter(account => account.account.data.parsed.info.tokenAmount.amount === '1' && account.account.data.parsed.info.tokenAmount.decimals === 0)
        .map(async (account) => {
          try {
            const mintAddress = account.account.data.parsed.info.mint;
            const nft = await metaplex.nfts().findByMint({ mintAddress: new PublicKey(mintAddress) });
            console.log("NFT Metadata URI for", account.account.data.parsed.info.mint, ":", nft.uri);

            if (nft.uri) {
              const isImageUrl = /\.(jpeg|jpg|png|gif)$/i.test(nft.uri);

              if (isImageUrl) {
                nft.json = { image: nft.uri };
              } else {
                const response = await fetch(nft.uri);
                const jsonMetadata = await response.json();
                nft.json = jsonMetadata;
              }
            }

            return nft;
          } catch (innerError) {
            console.error(`Error fetching NFT metadata for mint ${account.account.data.parsed.info.mint}:`, innerError);
            return null;
          }
        });

      let fetchedNfts = await Promise.all(nftPromises);

      fetchedNfts = fetchedNfts.map(nft => {
        if (nft) {
          nft.simulatedData = {
            temperature: Math.floor(Math.random() * 30) + 15,
            humidity: Math.floor(Math.random() * 50) + 30,
            ph: parseFloat((Math.random() * 2 + 6).toFixed(1)),
            ec: parseFloat((Math.random() * 0.5 + 1).toFixed(1)),
            plant_type: Math.floor(Math.random() * 5) + 1,
          };
        }
        return nft;
      });
      setNfts(fetchedNfts.filter(nft => nft !== null && nft.json?.image));

    } catch (error) {
      console.error("Error fetching NFTs:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (connected && publicKey && connection) {
      console.log("Wallet Public Key:", publicKey?.toBase58());
      fetchNFTs(publicKey, connection);
    } else {
      setNfts([]);
      setLoading(false);
    }
  }, [publicKey, connection, connected]);

  // Parse the last segment of name as time tag
  const parseTimeTag = (name) => {
    if (!name) return 0;
    const parts = name.split("-");
    if (parts.length < 8) return 0;
    // Convert MMDDHH format to number for sorting
    const tag = parts[7].replace(/\D/g, "");
    return Number(tag) || 0;
  };

  // Sort by time tag descending (newest NFT first)
  const sortedNfts = [...nfts].sort((a, b) => {
    const aTime = parseTimeTag(a.name);
    const bTime = parseTimeTag(b.name);
    return bTime - aTime;
  });

  return (
    <div className="container mx-auto px-4 py-8">
      <h2 className="text-3xl font-bold text-center mb-8">My NFTs</h2>
      <div className="flex flex-wrap justify-center gap-6">
        {loading && <p className="text-gray-600">Loading NFTs...</p>}
        {!loading && sortedNfts.length === 0 && publicKey && (
          <p className="text-gray-600">No NFTs found for this wallet.</p>
        )}
        {!loading && sortedNfts.length === 0 && !publicKey && (
          <p className="text-gray-600">Connect your wallet to see your NFTs.</p>
        )}
        {!loading && sortedNfts.length > 0 && (
          sortedNfts.map((nft, idx) => {

            const cityMap = {
              "Lon": "London", "Par": "Paris", "Ber": "Berlin", "Rom": "Rome", "Mad": "Madrid",
              "Ott": "Ottawa", "Was": "Washington", "Tok": "Tokyo", "Can": "Canberra", "Mos": "Moscow",
              "Bra": "Brasilia", "Bei": "Beijing", "Seo": "Seoul", "Ban": "Bangkok", "New": "New Delhi",
              "Cai": "Cairo", "Bue": "Buenos Aires", "Wel": "Wellington", "Osl": "Oslo", "Sto": "Stockholm"
            };
            const plantMap = {
              "Le": "Lettuce", "To": "Tomato", "Sp": "Spinach", "Ka": "Kale",
              "Be": "Bell Pepper", "Ce": "Celery", "Ca": "Cauliflower"
            };

            let grower = "N/A", city = "N/A", days = "N/A", plant = "N/A", ec = "N/A", ph = "N/A", temp = "N/A";
            if (nft.name) {

              const parts = nft.name.split("-");
              if (parts.length >= 7) {
                grower = parts[0] || "N/A";
                city = cityMap[parts[1]] || parts[1] || "N/A";
                days = parts[2] || "N/A";
                plant = plantMap[parts[3]] || parts[3] || "N/A";
                ec = parts[4] || "N/A";
                ph = parts[5] || "N/A";
                temp = parts[6].replace(/\.$/, "") || "N/A";
              }
            }
            // Prefer local nickname if available
            if (nft.address && publicKey) {
              const localNick = localStorage.getItem(`nickname_${publicKey}`);
              if (localNick) {
                grower = localNick.slice(0, 8);
              }
            }
            // Mapping city and plant
            const cityFull = cityMap[city] || city;
            const plantFull = plantMap[plant] || plant;

            return (
              <div
                key={nft.address.toBase58()}
                style={{
                  margin: "8px",
                  minWidth: "260px",
                  display: "flex",
                  flexDirection: "column",
                  alignItems: "center",
                  boxShadow: "0 4px 16px rgba(0,0,0,0.13), 0 1.5px 4px rgba(0,0,0,0.09)",
                  borderRadius: "12px",
                  background: "#fff"
                }}
              >
                <h3 className="text-xl font-semibold mb-2 mt-4">{nft.name}</h3>
                {nft.json?.image && (
                  <img
                    src={nft.json.image}
                    alt={nft.name}
                    style={{
                      width: "180px",
                      height: "180px",
                      objectFit: "cover",
                      borderRadius: "8px",
                      marginTop: "8px"
                    }}
                  />
                )}
                <p className="text-sm text-gray-700 mt-2">Mint: {nft.address.toBase58()}</p>
                <div style={{ marginTop: "8px", background: "#f8f8f8", borderRadius: "8px", padding: "10px", textAlign: "center", width: "100%" }}>
                  <div><strong>Grower:</strong> {grower}</div>
                  <div><strong>City:</strong> {cityFull}</div>
                  <div><strong>Grow Days:</strong> {days}</div>
                  <div><strong>Plant Name:</strong> {plantFull}</div>
                  <div><strong>EC:</strong> {ec}</div>
                  <div><strong>pH:</strong> {ph}</div>
                  <div><strong>Temperature:</strong> {temp === "N/A" ? "N/A" : `${temp}°C`}</div>
                </div>
                <div style={{ height: "12px" }}></div>
                {/* Growth Video Button */}
                <button
                  style={{
                    background: "#4CAF50",
                    color: "#fff",
                    border: "none",
                    borderRadius: "6px",
                    padding: "6px 16px",
                    cursor: "pointer",
                    marginBottom: "12px"
                  }}
                  onClick={() => setVideoOpenIndex(idx)}
                >
                  View Growth Video
                </button>
                {/* Modal for video */}
                {videoOpenIndex === idx && (
                  <div
                    style={{
                      position: "fixed",
                      top: 0,
                      left: 0,
                      width: "100vw",
                      height: "100vh",
                      background: "rgba(0,0,0,0.5)",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                      zIndex: 9999
                    }}
                    onClick={() => setVideoOpenIndex(null)}
                  >
                    <div
                      style={{
                        background: "#fff",
                        borderRadius: "12px",
                        padding: "16px",
                        position: "relative",
                        minWidth: "320px"
                      }}
                      onClick={e => e.stopPropagation()}
                    >
                      <button
                        style={{
                          position: "absolute",
                          top: "8px",
                          right: "12px",
                          background: "transparent",
                          border: "none",
                          fontSize: "20px",
                          cursor: "pointer"
                        }}
                        onClick={() => setVideoOpenIndex(null)}
                      >
                        ×
                      </button>
                      <div style={{ width: "480px", maxWidth: "80vw" }}>
                        <iframe
                          width="100%"
                          height="270"
                          src="https://www.youtube.com/embed/8G0zxQHHMqc"
                          title="Plant Growth Video"
                          frameBorder="0"
                          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                          allowFullScreen
                        ></iframe>
                      </div>
                    </div>
                  </div>
                )}
              </div>
            );
          })
        )}
      </div>
    </div>
  );
};

export default MyNFTsPage;