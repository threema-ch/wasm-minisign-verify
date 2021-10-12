const CopyWebpackPlugin = require('copy-webpack-plugin');
const path = require('path');

module.exports = {
    entry: {
        demo: './bootstrap.demo.js',
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bootstrap.[name].bundle.js',
    },
    plugins: [
        new CopyWebpackPlugin(['index.html']),
    ],
};
