const path = require("path");
/*const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require('extract-css-chunks-webpack-plugin');
const TerserPlugin = require('terser-webpack-plugin');
var webpack = require("webpack");

const isProduction = process.env.NODE_ENV === 'production';
const dist = path.resolve(__dirname, "dist");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const S3Plugin = require('webpack-s3-plugin')
*/
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development"
};
