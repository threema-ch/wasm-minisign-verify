const webpack = require('webpack');
const { merge } = require('webpack-merge');
const common = require('./webpack.common.js');

module.exports = merge(common, {
    mode: 'production',
    devtool: 'source-map',
    performance: {
        hints: 'warning'
    },
    output: {
        pathinfo: false
    },
    optimization: {
        moduleIds: 'deterministic',
        chunkIds: 'deterministic',
        nodeEnv: 'production',
        flagIncludedChunks: true,
        sideEffects: true,
        usedExports: true,
        concatenateModules: true,
        splitChunks: {
            hidePathInfo: true,
            minSize: 30000,
            maxAsyncRequests: 5,
            maxInitialRequests: 3,
        },
        emitOnErrors: false,
        checkWasmTypes: true,
        minimize: false,
    },
    plugins: [
        new webpack.DefinePlugin({ "process.env.NODE_ENV": JSON.stringify("production") })
    ]
});
