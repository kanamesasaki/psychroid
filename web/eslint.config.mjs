import { dirname } from "path";
import { fileURLToPath } from "url";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
});

const eslintConfig = [
  ...compat.extends("next/core-web-vitals", "next/typescript"),

  {
    // ignore WASM files
    ignores: ["lib/psychroid.js", "lib/psychroid_bg.wasm.d.ts", "lib/psychroid.d.ts"],
  },
];

export default eslintConfig;
