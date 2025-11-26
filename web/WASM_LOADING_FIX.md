# WASM Loading Error Fix 🔧

## The Error You Saw

```
useGame.ts:67 Failed to initialize WASM: TypeError: Cannot read properties of undefined (reading '__wbindgen_malloc')
```

## What This Means

The WASM module wasn't properly initialized before trying to use it. The `__wbindgen_malloc` function is part of the WASM module's internal API, and it's undefined because we tried to access `WasmGameEngine` before calling the initialization function.

## Root Cause

When using `wasm-pack` with `--target web`, the generated JavaScript file exports:
1. A default export: `__wbg_init()` - the initialization function
2. Named exports: `WasmGameEngine`, etc. - the actual classes

**The problem:** We were trying to use `WasmGameEngine` directly without calling the init function first.

## The Fix ✅

The `useGame.ts` file has been updated to properly initialize WASM:

**Before (Wrong):**
```typescript
const wasm = await import('./wasm/p2p_harvest_game.js');
const engine = new wasm.WasmGameEngine('Farmer'); // ❌ WASM not initialized!
```

**After (Correct):**
```typescript
const wasmModule = await import('./wasm/p2p_harvest_game.js');

// Initialize WASM first!
await wasmModule.default();

// Now we can use it
const wasm = wasmModule as WasmModule;
const engine = new wasm.WasmGameEngine('Farmer'); // ✅ Works!
```

## How to Apply This Fix

### If You're Seeing This Error:

1. **Pull the latest changes:**
   ```bash
   git pull
   ```

2. **Rebuild if needed:**
   ```bash
   cd web
   npm install
   ```

3. **Restart the dev server:**
   ```bash
   npm run dev
   ```

4. **Hard refresh your browser:**
   - Chrome/Edge: `Ctrl+Shift+R` (Windows/Linux) or `Cmd+Shift+R` (Mac)
   - Firefox: `Ctrl+F5` (Windows/Linux) or `Cmd+Shift+R` (Mac)
   - Safari: `Cmd+Option+R`

### If You Still See the Error:

```bash
# Clear everything and rebuild
cd web
rm -rf node_modules .vite dist
npm install
npm run wasm:build
npm run dev
```

## Verify the Fix

After starting the dev server, you should see:

1. ✅ No console errors
2. ✅ Game UI loads properly
3. ✅ You can see your inventory and stats
4. ✅ You can plant crops and play

## Understanding WASM Initialization

### Why is initialization needed?

WebAssembly modules need to:
1. Download the `.wasm` file
2. Compile it to native code
3. Set up memory and imports
4. Run initialization code

The `default()` export does all of this.

### The correct pattern:

```typescript
// 1. Import the module
import init, { WasmGameEngine } from './wasm/p2p_harvest_game.js';

// 2. Initialize (returns Promise)
await init();

// 3. Use the exports
const engine = new WasmGameEngine('Player');
```

Or with dynamic import:

```typescript
const wasm = await import('./wasm/p2p_harvest_game.js');
await wasm.default(); // Initialize
const engine = new wasm.WasmGameEngine('Player');
```

## Common Variations of This Error

### Error 1: "Cannot read properties of undefined"
```
TypeError: Cannot read properties of undefined (reading '__wbindgen_malloc')
```
**Fix:** Call `await wasmModule.default()` before using exports

### Error 2: "WebAssembly.instantiate(): Import #X module="wbg" error"
```
Failed to instantiate the module
```
**Fix:** Make sure you're using `--target web` when building WASM

### Error 3: "memory import must be a WebAssembly.Memory object"
```
LinkError: memory import must be a WebAssembly.Memory object
```
**Fix:** Rebuild WASM with correct target: `npm run wasm:build`

### Error 4: "fetch is not defined"
```
ReferenceError: fetch is not defined
```
**Fix:** You're trying to use it in Node.js. This is browser-only code.

## Testing the Fix

### Browser Console Test:

Open browser console (F12) and try:

```javascript
// This should work now
localStorage.getItem('harvest-game-state')

// No errors in console
// Game should be playable
```

### Manual Test:

1. ✅ Click "Plant" button on a seed
2. ✅ Click "Water Crops"
3. ✅ Click "Sleep"
4. ✅ See energy restore
5. ✅ Game state saves automatically

## Prevention

To avoid this error in future code:

### ✅ Do This:
```typescript
// Always initialize first
const wasm = await import('./wasm/p2p_harvest_game.js');
await wasm.default();
// Then use exports
```

### ❌ Don't Do This:
```typescript
// Skip initialization
const wasm = await import('./wasm/p2p_harvest_game.js');
new wasm.WasmGameEngine(); // ERROR!
```

## Related Issues

### Issue: WASM file not found (404)
**Symptoms:** `Failed to fetch WASM module`

**Fix:**
```bash
# Rebuild WASM
cd web
npm run wasm:build
```

### Issue: Vite doesn't serve WASM files
**Symptoms:** `.wasm` file returns HTML instead of binary

**Fix:** Already configured in `vite.config.ts`:
```typescript
server: {
  fs: {
    allow: ['..'] // Allows serving WASM from parent dir
  }
}
```

### Issue: WASM works in dev but not production
**Symptoms:** Works with `npm run dev`, fails with `npm run build`

**Fix:**
```bash
# Make sure to build WASM first
npm run wasm:build
npm run build
```

## Technical Details

### What `default()` does:

1. **Fetches WASM file:**
   ```javascript
   const module_or_path = new URL('p2p_harvest_game_bg.wasm', import.meta.url);
   ```

2. **Loads and instantiates:**
   ```javascript
   const { instance, module } = await __wbg_load(await module_or_path, imports);
   ```

3. **Sets up exports:**
   ```javascript
   wasm = instance.exports;
   wasm.__wbindgen_start();
   ```

### Why the error occurs:

Without initialization:
- `wasm` variable is `undefined`
- `WasmGameEngine` tries to call `wasm.__wbindgen_malloc()`
- Result: `Cannot read properties of undefined`

## Success Indicators

After applying the fix, you should see:

### Console (F12):
```
✓ No WASM-related errors
✓ Game state loads
✓ Actions work correctly
```

### UI:
```
✓ Stats panel shows data
✓ Inventory displays crops
✓ Buttons are clickable
✓ Notifications appear
```

### Functionality:
```
✓ Can plant crops
✓ Can water crops
✓ Can harvest
✓ Can sleep
✓ Game saves automatically
```

## Still Having Issues?

### Step 1: Check versions
```bash
node --version  # Should be v18+
npm --version
```

### Step 2: Clean rebuild
```bash
# From project root
rm -rf web/src/wasm/* web/node_modules web/dist
./build-web.sh
```

### Step 3: Check browser
- Use a modern browser (Chrome, Firefox, Safari, Edge)
- Make sure JavaScript is enabled
- Disable browser extensions (try incognito mode)

### Step 4: Check files exist
```bash
ls web/src/wasm/
# Should see:
# - p2p_harvest_game.js
# - p2p_harvest_game_bg.wasm
```

### Step 5: Check file content
```bash
head -20 web/src/wasm/p2p_harvest_game.js
# Should start with: let wasm;
```

## Additional Resources

- **WASM Module Docs:** [web/README.md](README.md)
- **Build Guide:** [../BUILD_INSTRUCTIONS.md](../BUILD_INSTRUCTIONS.md)
- **Quick Commands:** [../QUICK_COMMANDS.md](../QUICK_COMMANDS.md)
- **Error Fix:** [../ERROR_FIX.md](../ERROR_FIX.md)

## Summary

**The Problem:** WASM module not initialized before use

**The Solution:** Call `await wasmModule.default()` before accessing exports

**Status:** ✅ Fixed in `useGame.ts`

**Action:** Restart dev server and hard refresh browser

---

**Fixed! Happy farming! 🌾**