const webpack = require("webpack");

module.exports = function override(config) {
    // Polyfill Node.js core modules for browser
    const fallback = config.resolve.fallback || {};
    Object.assign(fallback, {
        crypto: require.resolve("crypto-browserify"),
        stream: require.resolve("stream-browserify"),
        assert: require.resolve("assert"),
        http: require.resolve("stream-http"),
        https: require.resolve("https-browserify"),
        os: require.resolve("os-browserify/browser"),
        url: require.resolve("url"),
        buffer: require.resolve("buffer"),
        process: require.resolve("process/browser.js"), // Note the .js extension
    });
    config.resolve.fallback = fallback;

    // Provide global variables for browser
    config.plugins = (config.plugins || []).concat([
        new webpack.ProvidePlugin({
            process: "process/browser.js", // Note the .js extension
            Buffer: ["buffer", "Buffer"],
        }),
    ]);

    // Allow imports without full extension in strict ESM
    config.module.rules.push({
        test: /\.m?js$/,
        resolve: { fullySpecified: false },
    });

    return config;
};
