# Web UI Quick Start Guide 🚀

Get the P2P Harvest Game running in your browser in just a few minutes!

## Prerequisites

You'll need these tools installed:

1. **Rust** - The programming language
2. **wasm-pack** - Compiles Rust to WebAssembly
3. **Node.js** (v18+) - JavaScript runtime for the frontend

## One-Time Setup

### Step 1: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Step 2: Install wasm-pack

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Or with cargo:
```bash
cargo install wasm-pack
```

### Step 3: Install Node.js

Download from https://nodejs.org/ (get the LTS version)

Or with a package manager:
- **macOS**: `brew install node`
- **Ubuntu/Debian**: `sudo apt install nodejs npm`
- **Windows**: Download installer from nodejs.org

Verify installation:
```bash
node --version  # should be v18 or higher
npm --version
```

## Quick Start (Automated)

Run the build script from the project root:

```bash
./build-web.sh
```

This will:
- ✅ Check all prerequisites
- ✅ Build the WASM module
- ✅ Install npm dependencies
- ✅ Set everything up

Then start the dev server:

```bash
cd web
npm run dev
```

Open your browser to: **http://localhost:3000**

## Quick Start (Manual)

If you prefer to run commands manually:

### 1. Build WASM Module

From the project root:

```bash
wasm-pack build \
  --target web \
  --out-dir web/src/wasm \
  --features wasm \
  --no-default-features
```

### 2. Install Frontend Dependencies

```bash
cd web
npm install
```

### 3. Start Development Server

```bash
npm run dev
```

### 4. Open in Browser

Navigate to **http://localhost:3000**

## After Code Changes

### Rust Code Changed?

Rebuild the WASM module:

```bash
# From project root
./build-web.sh

# Or manually
wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features
```

Then refresh your browser.

### React Code Changed?

No action needed! Vite's hot reload will automatically update the page.

## Game Controls

### Planting Crops
1. Look at your **Inventory** section
2. Click the **Plant** button on any seed
3. The crop moves to your **Fields**

### Watering
1. Click **💧 Water Crops** button
2. This waters all crops in your fields
3. Repeat daily until crops are ready

### Harvesting
1. When crops show "✓ Ready!" badge
2. Click **🌾 Harvest** button
3. Earn money from your crops!

### Managing Energy
- Each action costs energy (⚡)
- Click **💤 Sleep** to restore energy
- Watch the energy bar at the top

### Advancing Time
- Click **⏭️ Advance Day** to skip ahead
- Or use **Sleep** to advance while restoring energy

## Troubleshooting

### "Failed to load game" Error

**Solution**: Make sure WASM is built:
```bash
cd web
npm run wasm:build
```

### Port 3000 Already in Use

**Solution**: Change the port in `web/vite.config.ts`:
```typescript
server: {
  port: 3001,  // or any other port
}
```

### WASM Build Fails

**Problem**: Missing dependencies or wrong Rust version

**Solution**:
```bash
# Update Rust
rustup update

# Install wasm-pack
cargo install wasm-pack

# Try building again
cd web
npm run wasm:build
```

### Game State Not Saving

**Solution**: 
- Make sure localStorage is enabled in your browser
- Try a different browser
- Click "Reset Game" if save is corrupted

### Blank Screen

**Solution**:
1. Check browser console (F12) for errors
2. Make sure you ran `npm install`
3. Rebuild WASM: `npm run wasm:build`
4. Clear browser cache and refresh

## Development Workflow

### Making Changes to Game Logic (Rust)

1. Edit files in `src/core/`
2. Rebuild WASM: `./build-web.sh`
3. Refresh browser

### Making Changes to UI (React)

1. Edit files in `web/src/`
2. Changes apply automatically (hot reload)
3. No rebuild needed!

### Adding New Features

1. Add Rust logic in `src/core/`
2. Update WASM wrapper in `src/wasm/game_wrapper.rs`
3. Update TypeScript types in `web/src/types.ts`
4. Update React UI in `web/src/App.tsx`
5. Rebuild and test

## Production Build

To create an optimized build for deployment:

```bash
cd web
npm run wasm:build
npm run build
```

Files will be in `web/dist/`. Deploy these to any static hosting service:
- **Vercel**: `vercel deploy`
- **Netlify**: `netlify deploy`
- **GitHub Pages**: Push to `gh-pages` branch
- **Any HTTP server**: Copy `dist/` folder

## Tips for Development

1. **Keep WASM build terminal open**: See errors immediately
2. **Use React DevTools**: Install browser extension for debugging
3. **Check console logs**: F12 → Console tab
4. **Test in different browsers**: Chrome, Firefox, Safari
5. **Use TypeScript**: It catches errors before runtime!

## Next Steps

- Read the [full web README](web/README.md) for detailed info
- Check [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) for architecture details
- Explore the code in `web/src/` to customize the UI
- Add new crops in `src/core/crop.rs`

## Getting Help

If you run into issues:

1. Check the error message in browser console (F12)
2. Read the [web/README.md](web/README.md) troubleshooting section
3. Make sure all prerequisites are installed
4. Try running `./build-web.sh` again
5. Open an issue on GitHub with error details

---

**Ready to farm?** Run `./build-web.sh && cd web && npm run dev` 🌱

Happy harvesting! 🌾