import type { Configuration } from 'webpack'
import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  webpack: (config: Configuration) => {
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: true,
    };

    // module と rules が未定義の場合の初期化
    config.module = config.module || {};
    config.module.rules = config.module.rules || [];

    // Use WASM as asset/resource
    config.module.rules.push({
      test: /\.wasm$/,
      type: 'asset/resource',
    });

    return config;
  },
};

export default nextConfig;