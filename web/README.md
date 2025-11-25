# Harvest Game - Web UI

A React-based web interface for the P2P Harvest Game, powered by WebAssembly (WASM).

## Prerequisites

Before you begin, make sure you have the following installed:

1. **Rust** (for building WASM)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm-pack** (for building Rust to WASM)
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

3. **Node.js** (v18 or higher recommended)
   - Download from: https://nodejs.org/
   - Or use a version manager like `nvm`:
     ```bash
     nvm install 18
     nvm use 18
     ```

## Installation

1. Navigate to the web directory:
   ```bash
   cd web
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

## Building and Running

### Development Mode

To build the WASM module and start the development server in one command:

```bash
npm run wasm:dev
```

This will:
1. Build the Rust code to WASM
2. Start the Vite development server
3. Open your browser to `http://localhost:3000`

### Manual Steps

If you prefer to run the steps separately:

1. **Build WASM** (run this from the `web` directory):
   ```bash
   npm run wasm:build
   ```

2. **Start dev server**:
   ```bash
   npm run dev
   ```

### Production Build

To create a production build:

```bash
npm run wasm:build
npm run build
```

The built files will be in the `dist` directory. You can preview the production build with:

```bash
npm run preview
```

## Project Structure

```
web/
├── src/
│   ├── wasm/                 # Generated WASM files (auto-generated)
│   ├── App.tsx               # Main game component
│   ├── App.css               # Game styles
│   ├── useGame.ts            # Game state hook (WASM integration)
│   ├── types.ts              # TypeScript type definitions
│   ├── main.tsx              # Application entry point
│   ├── index.css             # Global styles
│   └── vite-env.d.ts         # Vite types
├── index.html                # HTML entry point
├── package.json              # Node dependencies
├── tsconfig.json             # TypeScript configuration
├── vite.config.ts            # Vite configuration
└── README.md                 # This file
```

## Game Features

### Current Features ✅

- **Day/Night Cycle**: Advance days and watch seasons change
- **Energy Management**: Monitor and restore energy through sleep
- **Crop Planting**: Plant seeds from your inventory
- **Crop Growth**: Water crops daily to help them grow
- **Harvesting**: Harvest mature crops for money
- **Season System**: Different seasons affect which crops can grow
- **Auto-Save**: Game state automatically saves to browser localStorage
- **Responsive Design**: Works on desktop and mobile devices

### Game Actions

1. **Plant Crops**: Select seeds from your inventory and plant them in your fields
2. **Water Crops**: Water your planted crops daily to help them grow
3. **Harvest**: Collect mature crops and earn money
4. **Sleep**: Restore your energy for the next day
5. **Advance Day**: Skip to the next day manually

### Tips for Playing

- Each action (planting, watering) costs energy
- Sleep to restore energy to 100
- Crops need to be watered for a specific number of days before harvest
- Different crops grow in different seasons
- Crops will die when the season changes if they can't grow in the new season

## Development

### Hot Reload

The Vite dev server supports hot module replacement (HMR). Changes to React components will update instantly without losing game state.

**Note**: If you modify the Rust code, you'll need to rebuild the WASM:
```bash
npm run wasm:build
```
Then refresh your browser.

### Debugging

1. **Browser DevTools**: Open your browser's developer console to see logs
2. **React DevTools**: Install the React DevTools browser extension for component inspection
3. **WASM Inspector**: Use browser WASM debugging tools to inspect the WebAssembly module

### TypeScript

The project uses TypeScript for type safety. Type definitions for the game engine are in `src/types.ts`.

## Browser Compatibility

The game requires a modern browser with WebAssembly support:

- ✅ Chrome/Edge 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Opera 44+

## Troubleshooting

### WASM Build Fails

**Problem**: `wasm-pack` command not found or build fails

**Solution**:
1. Make sure `wasm-pack` is installed:
   ```bash
   cargo install wasm-pack
   ```
2. Make sure you're in the `web` directory when running `npm run wasm:build`
3. Try running the build command directly:
   ```bash
   cd .. && wasm-pack build --target web --out-dir web/src/wasm --features wasm --no-default-features
   ```

### Development Server Won't Start

**Problem**: Port 3000 is already in use

**Solution**: Edit `vite.config.ts` and change the port:
```typescript
server: {
  port: 3001, // Change to any available port
}
```

### Game State Lost

**Problem**: Game resets when refreshing the page

**Solution**: The game uses localStorage for persistence. If it's not working:
1. Check that localStorage is enabled in your browser
2. Check browser console for errors
3. Try using a different browser
4. Click "Reset Game" to start fresh if the save is corrupted

### WASM Loading Error

**Problem**: "Failed to initialize WASM" error

**Solution**:
1. Make sure the WASM module is built: `npm run wasm:build`
2. Check that the `src/wasm/` directory exists and contains the generated files
3. Clear browser cache and refresh
4. Check browser console for detailed error messages

## Architecture

### WASM Integration

The game core is written in Rust and compiled to WebAssembly. The React frontend communicates with the WASM module through:

1. **WasmGameEngine**: A wrapper class that exposes game methods to JavaScript
2. **useGame Hook**: A React hook that manages WASM initialization and game state
3. **JSON Serialization**: Game state is serialized to/from JSON for communication

### State Management

- Game state is managed in the WASM module (Rust)
- React components receive read-only state through the `useGame` hook
- User actions trigger WASM method calls
- Results update the React UI

## Future Enhancements

- [ ] Add shop to buy more seeds
- [ ] Multiplayer/P2P trading features
- [ ] More crop varieties
- [ ] Weather effects
- [ ] Achievements system
- [ ] Sound effects and music
- [ ] Animations for actions
- [ ] Mobile-optimized touch controls
- [ ] Persistent cloud saves (optional)

## Contributing

1. Make changes to the Rust core in `../src/core/`
2. Rebuild WASM: `npm run wasm:build`
3. Test in the browser
4. Make React UI changes in `src/`
5. Submit a pull request

## License

[Same as parent project]

---

Built with ❤️ using Rust 🦀, WebAssembly 🕸️, React ⚛️, and Vite ⚡