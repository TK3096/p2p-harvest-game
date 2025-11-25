# Build Instructions 🛠️

Simple step-by-step instructions to build and run the web UI.

## Prerequisites

Before you start, make sure you have these installed:

### 1. Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. wasm-pack
```bash
cargo install wasm-pack
```

### 3. Node.js (v18 or higher)
Download from: https://nodejs.org/

Or use a version manager:
```bash
# Using nvm
nvm install 18
nvm use 18
```

Verify installations:
```bash
cargo --version
wasm-pack --version
node --version
npm --version
```

## Build Steps

### Step 1: Build the WASM Module

From the project root directory (`p2p-harvest-game/`):

```bash
wasm-pack build \
  --target web \
  --out-dir web/src/wasm \
  --features wasm \
  --no-default-features
```

This will:
- Compile Rust code to WebAssembly
- Generate JavaScript bindings
- Output files to `web/src/wasm/`

**Expected output:** `✨ Done in X seconds` with files in `web/src/wasm/`

### Step 2: Install Node Dependencies

```bash
cd web
npm install
```

This will:
- Install React, TypeScript, Vite, and other dependencies
- Create `node_modules/` directory

**Expected output:** `added X packages in Y seconds`

### Step 3: Start Development Server

```bash
npm run dev
```

This will:
- Start Vite development server
- Open on port 3000
- Enable hot module replacement

**Expected output:**
```
  VITE v5.0.8  ready in XXX ms

  ➜  Local:   http://localhost:3000/
  ➜  Network: use --host to expose
```

### Step 4: Open in Browser

Navigate to: **http://localhost:3000**

You should see the Harvest Game interface! 🎮

## Quick Commands

Once everything is set up:

```bash
# Start dev server (from web/ directory)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Rebuild WASM only (from project root)
wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features
```

## Troubleshooting

### Error: "uuid feature error" during WASM build

**Solution:** The Cargo.toml has been fixed to include the `js` feature for uuid.

If you still see this error:
```bash
# Make sure Cargo.toml has:
uuid = { version = "1.18", features = ["v4", "serde", "js"] }
```

### Error: "wasm-pack: command not found"

**Solution:**
```bash
cargo install wasm-pack
# Wait for installation to complete
```

### Error: "node: command not found"

**Solution:** Install Node.js from https://nodejs.org/

### Error: Port 3000 already in use

**Solution:** Edit `web/vite.config.ts`:
```typescript
server: {
  port: 3001,  // Change to any available port
}
```

### Error: Cannot find module when running dev server

**Solution:**
```bash
cd web
rm -rf node_modules package-lock.json
npm install
```

### WASM files not found in browser

**Solution:**
1. Make sure WASM build completed successfully
2. Check that files exist in `web/src/wasm/`
3. Refresh browser (Cmd+Shift+R or Ctrl+Shift+R)

### Game shows blank screen

**Solution:**
1. Open browser console (F12)
2. Look for error messages
3. Check that all build steps completed
4. Try: `npm run wasm:build` from web/ directory

## File Verification

After building, you should have these files:

```
p2p-harvest-game/
├── web/
│   ├── src/
│   │   ├── wasm/
│   │   │   ├── p2p_harvest_game.js      ✓ Generated
│   │   │   ├── p2p_harvest_game.d.ts    ✓ Generated
│   │   │   ├── p2p_harvest_game_bg.wasm ✓ Generated
│   │   │   └── package.json             ✓ Generated
│   │   ├── App.tsx
│   │   ├── useGame.ts
│   │   └── ...
│   └── node_modules/                    ✓ Created by npm install
```

## Development Workflow

### Making Changes to Rust Code

1. Edit files in `src/core/` or `src/wasm/`
2. Rebuild WASM:
   ```bash
   wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features
   ```
3. Refresh browser

### Making Changes to React/UI

1. Edit files in `web/src/`
2. Changes apply automatically (hot reload)
3. No rebuild needed!

## Production Build

To create optimized files for deployment:

```bash
# From project root
wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features --release

# Then build React app
cd web
npm run build
```

Output will be in `web/dist/` directory.

Deploy this folder to:
- Vercel
- Netlify
- GitHub Pages
- Any static hosting service

## Alternative: Using the Build Script

If you prefer automation:

```bash
# From project root
./build-web.sh
```

This script does all the steps above automatically.

## Getting Help

If you encounter issues:

1. Check this file for troubleshooting steps
2. Read the error message carefully
3. Check [WEB_QUICKSTART.md](WEB_QUICKSTART.md)
4. Check browser console (F12) for errors
5. Open an issue on GitHub with:
   - Error message
   - Steps you took
   - Your OS and versions

## Version Information

Check your versions:
```bash
rustc --version    # Should be 1.70+
cargo --version
wasm-pack --version
node --version     # Should be v18+
npm --version
```

## Success Indicators

You'll know everything works when:
- ✓ WASM build completes without errors
- ✓ npm install completes successfully
- ✓ Dev server starts on port 3000
- ✓ Browser shows game interface
- ✓ You can plant crops and play the game

---

**Ready to build?** Start with Step 1 above!

For more details, see:
- [WEB_QUICKSTART.md](WEB_QUICKSTART.md) - Quick start guide
- [web/README.md](web/README.md) - Detailed documentation
- [GETTING_STARTED_WEB.md](GETTING_STARTED_WEB.md) - Complete guide