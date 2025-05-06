import Arweave from "arweave";

const arweave = Arweave.init({
    host: "arweave.net",
    port: 443,
    protocol: "https",
});

export const uploadToArweave = async (file) => {
    const reader = new FileReader();
    reader.readAsArrayBuffer(file);

    return new Promise((resolve, reject) => {
        reader.onloadend = async () => {
            try {
                const transaction = await arweave.createTransaction({
                    data: Buffer.from(reader.result),
                });

                // Add tags
                transaction.addTag("Content-Type", file.type);

                // Sign and submit transaction
                await arweave.transactions.sign(transaction);
                await arweave.transactions.post(transaction);

                const imageUrl = `https://arweave.net/${transaction?.id}`;
                resolve(imageUrl);
            } catch (error) {
                reject(error);
            }
        };
    });
};
