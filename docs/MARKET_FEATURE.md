# Market Feature Documentation

## Overview

The Market feature allows players to purchase seeds for planting in the web version of the P2P Harvest Game. This provides a convenient way to acquire new crops without relying on P2P networking (which is CLI-only).

## Features

### 🏪 Seed Market
- **Seasonal Seeds**: Only seeds that grow in the current season are available
- **Dynamic Pricing**: Seed costs are 50% of their sell price
- **Visual Feedback**: See growth time, sell price, and valid seasons for each seed
- **Instant Purchase**: Buy seeds directly with your in-game money

## How to Use

### Opening the Market
1. Click the **🏪 Market** button in the Actions Panel
2. The market modal will display all available seeds for the current season

### Purchasing Seeds
1. Browse the available seeds in the market
2. Each seed card shows:
   - **Name**: The crop name
   - **Cost**: Purchase price (💰)
   - **Growth Days**: How long to maturity
   - **Sell Price**: Value when harvested
   - **Seasons**: Visual icons showing valid growing seasons
3. Click **Buy** on any seed you can afford
4. The seed is added to your inventory immediately
5. Your money is deducted automatically

### Season-Specific Seeds

#### 🌸 Spring Seeds
- Carrot - $25 (Growth: 3 days, Sells: $50)
- Potato - $30 (Growth: 4 days, Sells: $60)
- Parsnip - $17 (Growth: 4 days, Sells: $35)

#### ☀️ Summer Seeds
- Tomato - $40 (Growth: 5 days, Sells: $80)
- Corn - $75 (Growth: 14 days, Sells: $150)
- Melon - $125 (Growth: 12 days, Sells: $250)

#### 🍂 Autumn Seeds
- Pumpkin - $160 (Growth: 13 days, Sells: $320)
- Corn - $75 (Growth: 14 days, Sells: $150)
- Yam - $80 (Growth: 10 days, Sells: $160)

#### ❄️ Winter Seeds
- Wheat - $50 (Growth: 7 days, Sells: $100)
- Winter Seeds - $40 (Growth: 7 days, Sells: $80)

## Implementation Details

### Backend (Rust)
- **GameCommand::BuySeed**: New command for purchasing seeds
- **GameEvent::SeedPurchased**: Event fired when seed is bought
- **get_seasonal_crops()**: Returns available crops for current season
- **Pricing Logic**: Seed cost = sell_price * 0.5

### WASM Bindings
- `buySeed(seedName: string)`: Purchase a seed by name
- `getAvailableSeeds()`: Get all seeds for current season

### Frontend (React)
- **Market Modal**: Full-screen overlay with seed grid
- **Seed Cards**: Interactive cards showing seed information
- **Real-time Updates**: Money and inventory update immediately

## Tips & Strategy

1. **Buy Early**: Purchase seeds at the start of each season
2. **High Value Crops**: Melon and Pumpkin have the best profit margins
3. **Quick Crops**: Carrots are great for fast money early game
4. **Multi-Season**: Wheat grows in all seasons - always a safe bet
5. **Plan Ahead**: Seeds bought in one season can be planted in future seasons if they're compatible

## Differences from CLI P2P Trading

The web version uses a **local market system** instead of P2P trading:

| Feature | CLI P2P Trade | Web Market |
|---------|--------------|------------|
| Network Required | Yes (iroh protocol) | No |
| Trade with Players | Yes | No (NPC market) |
| Send/Receive Items | Both | Buy only |
| Platform | Desktop only | Browser-based |
| Offline Play | No | Yes |

## Future Enhancements

Potential additions to the market system:
- [ ] Sell crops back to market (beyond harvest)
- [ ] Market price fluctuations
- [ ] Bulk purchasing
- [ ] Special seasonal deals
- [ ] Market upgrades/reputation system
- [ ] Trade between browser instances (WebRTC P2P)

## Troubleshooting

### "Seed not available"
- The seed may not grow in the current season
- Check the season indicator in your stats

### "Not enough money"
- Harvest your crops to earn money
- Plant cheaper crops first (Carrot, Parsnip)
- Wait for crops to mature before buying more

### Market button disabled
- This shouldn't happen - try refreshing the page
- Check browser console for errors

## Code Examples

### TypeScript Usage
```typescript
// Get available seeds
const seeds = getAvailableSeeds();
console.log(seeds); // Array of SeedInfo objects

// Buy a seed
buySeed("Carrot");
```

### Rust/WASM
```rust
// Execute buy command
let result = engine.execute(GameCommand::BuySeed {
    seed_name: "Carrot".to_string(),
});

// Get available seeds
let current_season = engine.get_current_season();
let seeds = get_seasonal_crops(current_season);
```

## Related Files

- `/src/core/types.rs` - GameCommand and GameEvent definitions
- `/src/core/game_engine.rs` - Market logic implementation
- `/src/core/crop.rs` - Seed/crop definitions
- `/src/wasm/game_wrapper.rs` - WASM bindings
- `/web/src/App.tsx` - Market UI component
- `/web/src/useGame.ts` - Market hooks
- `/web/src/types.ts` - TypeScript types

## Contributing

To add new crops to the market:
1. Add crop definition in `src/core/crop.rs::get_seasonal_crops()`
2. Rebuild WASM: `./build-web.sh`
3. Seeds will automatically appear in the market

No frontend changes needed - the market dynamically displays all available seeds!