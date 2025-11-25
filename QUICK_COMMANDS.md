# Quick Commands Reference 🚀

Essential commands for P2P Harvest Game Web UI development.

## First Time Setup

```bash
# 1. Install wasm-pack (one-time only)
cargo install wasm-pack

# 2. Build everything
./build-web.sh

# 3. Start the game
cd web && npm run dev
```

**Then open:** http://localhost:3000

---

## Daily Development

### Start Development Server
```bash
cd web
npm run dev
```

### Rebuild WASM (after editing Rust code)
```bash
# From project root
wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features

# Or use the shortcut from web/ directory
cd web && npm run wasm:build
```

### Quick Build & Run
```bash
# Rebuilds WASM and starts dev server
cd web && npm run wasm:dev
```

---

## Build Commands

### Development Build
```bash
# WASM module
wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features

# React app (dev mode)
cd web && npm run dev
```

### Production Build
```bash
# WASM module (optimized)
wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features --release

# React app (production)
cd web && npm run build

# Preview production build
cd web && npm run preview
```

---

## Testing & Verification

### Test Rust Core
```bash
cargo test
```

### Check WASM Compilation
```bash
cargo check --lib --no-default-features --features wasm
```

### Check Diagnostics
```bash
cargo clippy --features wasm --no-default-features
```

---

## Cleanup Commands

### Clean Rust Build
```bash
cargo clean
```

### Clean WASM Output
```bash
rm -rf web/src/wasm/*
```

### Clean Node Modules
```bash
cd web && rm -rf node_modules package-lock.json
```

### Full Clean & Rebuild
```bash
# Clean everything
cargo clean
rm -rf web/src/wasm/* web/node_modules web/package-lock.json

# Rebuild from scratch
./build-web.sh
```

---

## Troubleshooting Commands

### Reset Game State (in browser)
```javascript
// Open browser console (F12) and run:
localStorage.removeItem('harvest-game-state')
location.reload()
```

### Check Versions
```bash
rustc --version
cargo --version
wasm-pack --version
node --version
npm --version
```

### Reinstall Dependencies
```bash
cd web
rm -rf node_modules package-lock.json
npm install
```

### Fix Port Conflict
```bash
# Edit web/vite.config.ts and change port
# Or run on different port:
cd web && npm run dev -- --port 3001
```

---

## File Operations

### View WASM Files
```bash
ls -lh web/src/wasm/
```

### Check File Sizes
```bash
# WASM module size
du -h web/src/wasm/p2p_harvest_game_bg.wasm

# Production build size
du -h web/dist/
```

### View Game State
```bash
# On macOS/Linux
cat ~/.config/harvest-game/.game-state.json

# Or check localStorage in browser (F12 → Application → Local Storage)
```

---

## Git Commands

### Ignore Generated Files
```bash
# Already configured in .gitignore:
# - web/src/wasm/* (except .gitkeep)
# - web/node_modules/
# - web/dist/
# - target/
```

### Commit Changes
```bash
git add .
git commit -m "Your message"
git push
```

---

## Deployment Commands

### Deploy to Vercel
```bash
cd web
npm run build
vercel deploy
```

### Deploy to Netlify
```bash
cd web
npm run build
netlify deploy
```

### Deploy to GitHub Pages
```bash
cd web
npm run build
cd dist
git init
git add -A
git commit -m 'Deploy'
git push -f git@github.com:username/repo.git main:gh-pages
```

---

## Shortcuts & Aliases

Add these to your shell profile for faster workflow:

```bash
# ~/.bashrc or ~/.zshrc

# Build WASM
alias wasm-build='wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features'

# Start dev server
alias game-dev='cd web && npm run dev'

# Build everything
alias game-build='./build-web.sh'

# Quick restart
alias game-restart='cd web && npm run wasm:build && npm run dev'
```

---

## NPM Scripts (from web/ directory)

```bash
npm run dev           # Start dev server
npm run build         # Build for production
npm run preview       # Preview production build
npm run wasm:build    # Rebuild WASM module
npm run wasm:dev      # Rebuild WASM + start dev
```

---

## Most Used Commands

**90% of the time, you'll use these:**

```bash
# First time
./build-web.sh

# Start development
cd web && npm run dev

# After editing Rust code
cd web && npm run wasm:build
# Then refresh browser

# That's it! 🎮
```

---

## Quick Reference Table

| Task | Command |
|------|---------|
| Setup | `./build-web.sh` |
| Dev Server | `cd web && npm run dev` |
| Rebuild WASM | `npm run wasm:build` (from web/) |
| Full Rebuild | `npm run wasm:dev` (from web/) |
| Production | `npm run build` (from web/) |
| Test Rust | `cargo test` |
| Clean All | `cargo clean && rm -rf web/node_modules` |

---

## Emergency Reset

If everything breaks:

```bash
# 1. Clean everything
cargo clean
rm -rf web/src/wasm/* web/node_modules web/dist

# 2. Update tools
rustup update
cargo install wasm-pack --force

# 3. Rebuild
./build-web.sh

# 4. Start fresh
cd web && npm run dev
```

---

**Tip:** Bookmark this file! Press `Cmd/Ctrl + D` in your editor.

**Need help?** Check:
- [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) - Detailed steps
- [ERROR_FIX.md](ERROR_FIX.md) - Common errors
- [WEB_QUICKSTART.md](WEB_QUICKSTART.md) - Getting started

Happy farming! 🌾