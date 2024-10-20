const webpack = require('webpack');
const path = require('path');

const devOnlyConfig = {
  devtool: 'source-map',
}

const isDevelopment = (mode) => mode === 'development'

const getModeConfig = (mode) => {
  return isDevelopment(mode) ? devOnlyConfig : {}
}

module.exports = (env, argv) => ({
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
      BACKEND_PORT: 'auto'
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
  ...getModeConfig(argv.mode)
});
