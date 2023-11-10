const webpack = require('webpack');
const path = require('path');

module.exports = {
  entry: './src/index.tsx',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: './bundle.js',
    publicPath: '/'
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.(sa|sc|c)ss$/i,
        use: ["style-loader", "css-loader", "sass-loader"],
      },
    ],
  },
  plugins: [
    new webpack.EnvironmentPlugin({
      'BACKEND_HOST': '127.0.0.1:8001'
    })
  ],
  resolve: {
    extensions: ['.tsx', '.ts', '.jsx', '.js', '.css'],
  },
  experiments: {
    syncWebAssembly: true,
    asyncWebAssembly: true,
  },
  devServer: {
    historyApiFallback: true,
  },
};
