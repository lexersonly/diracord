import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  cacheComponents: true,
  partialPrefetching: true,
  async redirects() {
    return [
      // Components v2 moved to its own area; keep old links (README on npm) working.
      { source: "/docs/components-v2", destination: "/components/overview", permanent: true },
    ];
  },
  turbopack: {
    // The site lives inside the Diracord monorepo; keep Turbopack inside site/.
    root: process.cwd(),
    rules: {
      "*.css": {
        loaders: ["@tailwindcss/turbopack"],
        as: "*.css",
      },
    },
  },
};

export default nextConfig;
