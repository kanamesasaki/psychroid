# Use the official Node.js image (Debian-based) as the base image
FROM node:22-bullseye

# Update the system and install required packages
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install the Rust toolchain using rustup in non-interactive mode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Cargo's binary directory to the PATH environment variable
ENV PATH="/root/.cargo/bin:${PATH}"

# Install wasm-pack for building the WebAssembly module from Rust
RUN cargo install wasm-pack

# Set the working directory to /app (this will be the root of our repository)
WORKDIR /psychroid

# Copy the entire repository into the container
COPY . .

# Move to the frontend directory and install Node.js dependencies
WORKDIR /psychroid/web
RUN npm install

# Expose the port used by Next.js development server
EXPOSE 3000

# Set the working directory back to the repository root (optional)
WORKDIR /psychroid

# Default command: start the frontend development server.
# This assumes that the "start" script in web/package.json runs webpack-dev-server.
CMD ["sh", "-c", "cd web && npm run start"]

# Steps to setup the Frontend
# npx create-next-app@latest web
# ✔ Would you like to use TypeScript? … Yes
# ✔ Would you like to use ESLint? … Yes
# ✔ Would you like to use Tailwind CSS? … Yes
# ✔ Would you like your code inside a `src/` directory? … No
# ✔ Would you like to use App Router? (recommended) … Yes
# ✔ Would you like to use Turbopack for `next dev`? … No
# ✔ Would you like to customize the import alias (`@/*` by default)? … No
# cd web

# root@400b5f27a40d:/workspaces/psychroid/web# npx shadcn@latest init
# Need to install the following packages:
# shadcn@2.3.0
# Ok to proceed? (y) y

# ✔ Preflight checks.
# ✔ Verifying framework. Found Next.js.
# ✔ Validating Tailwind CSS.
# ✔ Validating import alias.
# ✔ Which style would you like to use? › Default
# ✔ Which color would you like to use as the base color? › Neutral
# ✔ Would you like to use CSS variables for theming? … yes
# ✔ Writing components.json.
# ✔ Checking registry.
# ✔ Updating tailwind.config.ts
# ✔ Updating app/globals.css
#   Installing dependencies.
# ✔ How would you like to proceed? › Use --force
# ✔ Installing dependencies.
# ✔ Created 1 file:
#   - lib/utils.ts

# Success! Project initialization completed.
# You may now add components.

# npm install d3 @types/d3

# root@400b5f27a40d:/workspaces/psychroid/web# npx shadcn@latest add button
# root@400b5f27a40d:/workspaces/psychroid/web# npx shadcn@latest add input 
# root@400b5f27a40d:/workspaces/psychroid/web# npx shadcn@latest add card 
# root@400b5f27a40d:/workspaces/psychroid/web# npx shadcn@latest add select
# root@400b5f27a40d:/workspaces/psychroid/web# npx shadcn@latest add label
# root@400b5f27a40d:/workspaces/psychroid/web# npx shadcn@latest add textarea
# root@400b5f27a40d:/workspaces/psychroid/web# npx shadcn@latest add radio-group

# [Step1] 以下のコマンドでwasm-bindgen用の出力を生成します。ここでは Web 用に出力するため、--target web を利用します。
# cd /workspaces/psychroid
# wasm-pack build --target web --out-dir web/lib
# 以下のファイル（utils.ts以外）が生成されていることを確認する。
# root@400b5f27a40d:/workspaces/psychroid/web/lib# ls 
# README.md  package.json  psychroid.d.ts  psychroid.js  psychroid_bg.wasm  psychroid_bg.wasm.d.ts  utils.ts

# [Step2] tsconfig.json の compilerOptions.paths でエイリアスが使えるように設定
# {
#   "compilerOptions": {
#     "paths": {
#       "@/*": ["./*"]
#     }
#   },
# }

# [Step3] next.config.ts を以下のように編集し、WASM ファイルを正しく扱うためのルールを追加
# Webpack 5 のネイティブ機能を使って、WASMファイルを asset/resource として出力する

# [Step4] wasm-bindgen で生成されたファイルを import して利用する

# npm install nodemailer