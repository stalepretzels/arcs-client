const path = require('path')
const webpack = require('webpack')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  entry: {
    index: './index.js',
    },
    output: {
    filename: 'bundle.js',
    },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
      extraArgs: "--target web"
    }),
    // Have this example work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    new webpack.ProvidePlugin({
      TextDecoder: ['text-encoding', 'TextDecoder'],
      TextEncoder: ['text-encoding', 'TextEncoder']
    })
  ],
  mode: 'development',
  experiments: {
    asyncWebAssembly: true
  }
}