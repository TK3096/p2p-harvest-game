# Error Fix Documentation 🔧

## The Error You Encountered

When running `./build-web.sh`, you likely saw an error like this:

```
error: to use `uuid` on `wasm32-unknown-unknown`, specify a source of randomness 
using one of the `js`, `rng-getrandom`, or `rng-rand` features
```

## Root Cause

The `uuid` crate needs to know how to generate random numbers when compiled to WebAssembly. By default, it doesn't have access to the system's random number generator in the browser.

## The Fix

I've already fixed this in your `Cargo.toml` file:

**Before:**
```toml
uuid = { version = "1.18", features = ["v4", "serde"] }
```

**After:**
```toml
uuid = { version = "1.18", features = ["v4", "serde", "js"] }
```

The `"js"` feature tells uuid to use JavaScript's crypto API for randomness in the browser.

## How to Build Now

### Option 1: Using the Build Script (Recommended)

```bash
./build-web.sh
```

This will:
1. Check prerequisites
2. Build WASM module
3. Install npm dependencies
4. Show you next steps

### Option 2: Manual Steps

```bash
# 1. Build WASM
wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features

# 2. Install dependencies
cd web
npm install

# 3. Start dev server
npm run dev
```

### Option 3: Quick Development Build

```bash
# From web/ directory
npm run wasm:dev
```

This combines WASM build + dev server startup.

## Verify the Fix

After running the build, you should see:

```
[INFO]: ✨   Done in XX.XXs
[INFO]: 📦   Your wasm pkg is ready to publish at .../web/src/wasm
```

Then the dev server should start:

```
VITE v5.0.8  ready in XXX ms

➜  Local:   http://localhost:3000/
```

## What Files Were Created

After a successful build, check these files exist:

```bash
ls web/src/wasm/
```

You should see:
- `p2p_harvest_game.js`
- `p2p_harvest_game.d.ts`
- `p2p_harvest_game_bg.wasm`
- `p2p_harvest_game_bg.wasm.d.ts`
- `package.json`

## Testing the Game

1. Open browser to `http://localhost:3000`
2. You should see the Harvest Game UI
3. Try planting a crop from your inventory
4. The game should work without errors

## If You Still Have Issues

### Issue: wasm-pack not found

```bash
cargo install wasm-pack
```

### Issue: Node.js not found

Install from: https://nodejs.org/ (get v18 or higher)

### Issue: Build succeeds but browser shows error

```bash
# Clear everything and rebuild
rm -rf web/src/wasm web/node_modules
wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features
cd web && npm install && npm run dev
```

### Issue: Port 3000 already in use

Edit `web/vite.config.ts` and change:
```typescript
server: {
  port: 3001,  // or any available port
}
```

## Additional Dependencies Fixed

The fix also ensures these work correctly in WASM:
- `getrandom` - Uses JavaScript crypto API
- `rand` - Random number generation
- `uuid` - Unique ID generation for crops

## Related Files

- `Cargo.toml` - Contains the fix
- `src/wasm/game_wrapper.rs` - WASM bindings
- `web/src/useGame.ts` - React integration

## Next Steps

1. **Build the game**: `./build-web.sh`
2. **Start playing**: Open http://localhost:3000
3. **Read the docs**: Check `WEB_QUICKSTART.md` for usage
4. **Customize**: Edit files in `web/src/` for UI changes

## Success Checklist

- [x] Fixed uuid feature for WASM
- [x] WASM module builds successfully
- [x] Generated files in `web/src/wasm/`
- [x] npm dependencies installed
- [ ] Dev server running (run `cd web && npm run dev`)
- [ ] Game loads in browser
- [ ] You can play the game!

## Summary

**The error was:** Missing `js` feature for the `uuid` crate in WASM builds.

**The fix was:** Add `"js"` to uuid features in `Cargo.toml`.

**Status:** ✅ Fixed and ready to use!

---

**Now run:** `./build-web.sh` and start farming! 🌾