# Web UI Guide 🎮

A visual guide to the P2P Harvest Game web interface.

## UI Layout

```
┌─────────────────────────────────────────────────────────────────┐
│  🌾 Harvest Game                              [Reset Game]      │
└─────────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────────┐
│  ✓ Crop planted successfully!                              ×    │
└─────────────────────────────────────────────────────────────────┘

┌──────────────┐  ┌────────────────────────────────┐  ┌───────────┐
│ STATS PANEL  │  │      MAIN GAME AREA            │  │  ACTIONS  │
│              │  │                                │  │           │
│ Day 5        │  │  🌱 Fields (3)                 │  │ 💧 Water  │
│ Spring       │  │  ┌─────┬─────┬─────┐          │  │           │
│ Year 1       │  │  │Carrt│Tomat│Potat│          │  │ 🌾 Harvest│
│              │  │  │ ✓   │ 60% │ 75% │          │  │           │
│ Farmer       │  │  └─────┴─────┴─────┘          │  │ 💤 Sleep  │
│ 💰 $1,250    │  │                                │  │           │
│              │  │  🎒 Inventory (4)              │  │ ⏭️ Advance│
│ ⚡ Energy    │  │  ┌─────┬─────┬─────┬─────┐    │  │           │
│ ▓▓▓▓▓▓░░░░   │  │  │Carrt│Wheat│Tomat│Potat│    │  └───────────┘
│ 85/100       │  │  │Plant│Plant│Plant│Plant│    │
│              │  │  └─────┴─────┴─────┴─────┘    │
└──────────────┘  └────────────────────────────────┘
```

## Panel Breakdown

### 1. Header Bar
```
┌─────────────────────────────────────────────────────┐
│  🌾 Harvest Game                    [Reset Game]    │
└─────────────────────────────────────────────────────┘
```
- **Title**: Game name with emoji
- **Reset Button**: Click to restart game (confirms first)

### 2. Notification Area
```
┌─────────────────────────────────────────────────────┐
│  ✓ Crop planted successfully!                  ×    │
└─────────────────────────────────────────────────────┘
```
- **Success** (green border): Positive actions
- **Error** (red border): When actions fail
- **Click to dismiss** or it auto-fades

### 3. Stats Panel (Left Side)

#### Day & Season Card
```
┌──────────────┐
│ 📅 Day 5     │
│ Spring       │ ← Current season (color-coded)
│ Year 1, Day 5│ ← Detailed time info
└──────────────┘
```
- Shows current day number
- Displays season with color
- Year and day within season

#### Player Info Card
```
┌──────────────┐
│ 👤 Farmer    │
│ 💰 $1,250    │ ← Your money
└──────────────┘
```
- Player name
- Current money (earned from harvesting)

#### Energy Card
```
┌──────────────┐
│ ⚡ Energy    │
│ ▓▓▓▓▓▓░░░░   │ ← Visual bar
│ 85 / 100     │ ← Current/Max
└──────────────┘
```
- **Green bar**: >50% energy
- **Orange bar**: 25-50% energy
- **Red bar**: <25% energy
- Restores to 100 when sleeping

### 4. Main Game Area (Center)

#### Fields Section
```
🌱 Fields (3)  ← Number of planted crops

┌─────────┐ ┌─────────┐ ┌─────────┐
│ Carrot  │ │ Tomato  │ │ Potato  │
│ ✓ Ready!│ │ 60%     │ │ 75%     │ ← Growth progress
│ 3/3 days│ │ 3/5 days│ │ 3/4 days│
│ $50     │ │ $80     │ │ $60     │ ← Sell price
└─────────┘ └─────────┘ └─────────┘
   Ready      Growing     Growing
```

**Crop Card States:**
- **Normal** (grey border): Growing
- **Ready** (green border): Harvest now!
- **Progress Bar**: Visual growth indicator
- **Days Counter**: Watered days / Total needed

#### Inventory Section
```
🎒 Inventory (4)  ← Number of seeds available

┌─────────┐ ┌─────────┐ ┌─────────┐
│ Carrot  │ │ Wheat   │ │ Tomato  │
│ 3 days  │ │ 7 days  │ │ 5 days  │ ← Growth time
│ $50     │ │ $100    │ │ $80     │ ← Value
│ 🌸☀️🍂  │ │ 🌸☀️🍂❄️│ │ ☀️      │ ← Seasons
│ [Plant] │ │ [Plant] │ │ [Plant] │
│ (⚡15)  │ │ (⚡15)  │ │ (⚡15)  │ ← Energy cost
└─────────┘ └─────────┘ └─────────┘
```

**Season Emojis:**
- 🌸 = Spring
- ☀️ = Summer
- 🍂 = Autumn
- ❄️ = Winter

**Plant Button States:**
- **Enabled**: Enough energy to plant
- **Disabled** (greyed): Not enough energy

### 5. Actions Panel (Right Side)
```
┌─────────────┐
│ ⚔️ Actions  │
├─────────────┤
│ 💧 Water    │ ← Water all crops
│   Crops     │
├─────────────┤
│ 🌾 Harvest  │ ← Collect ready crops
├─────────────┤
│ 💤 Sleep    │ ← Restore energy + advance day
├─────────────┤
│ ⏭️ Advance  │ ← Skip to next day
│   Day       │
└─────────────┘
```

**Button States:**
- **Enabled** (purple gradient): Can use
- **Disabled** (grey): Can't use (no crops, etc.)

## How to Play

### Starting Out

1. **Check Your Inventory** 🎒
   - You start with 4 free seeds
   - Each has different growth time and value

2. **Plant Your First Crop** 🌱
   ```
   Inventory → Click [Plant] → Crop moves to Fields
   ```
   - Costs 15 energy per crop
   - Watch energy bar!

3. **Water Daily** 💧
   ```
   Fields → Click [Water Crops]
   ```
   - Waters ALL crops at once
   - Costs 15 energy per crop
   - Do this every day!

4. **Watch Growth** 📈
   ```
   0/3 days → 1/3 days → 2/3 days → ✓ Ready!
   ```
   - Progress bar fills up
   - "Ready!" badge appears

5. **Harvest** 🌾
   ```
   When ready → Click [Harvest]
   ```
   - Get money!
   - Crop removed from field
   - Free up space for new crops

6. **Manage Energy** ⚡
   ```
   Low energy? → Click [Sleep]
   ```
   - Restores to 100
   - Advances to next day
   - Crops stay watered

### Daily Routine

```
Morning (Full Energy)
├─ Plant new seeds (if space)
├─ Water all crops
└─ Check progress

Afternoon (Low Energy)
├─ Harvest ready crops
└─ Sleep (restore energy)

Repeat! 🔄
```

### Season Changes

Every 28 days, the season changes:
```
Day 1-28:   Spring 🌸
Day 29-56:  Summer ☀️
Day 57-84:  Autumn 🍂
Day 85-112: Winter ❄️
Day 113+:   Spring again...
```

**Warning**: Crops die if they can't grow in the new season!
- Plant seasonal crops before change
- Or harvest before season ends

### Making Money 💰

**Low Value, Fast Growth:**
- Carrot: $50 (3 days)
- Potato: $60 (4 days)

**Medium Value, Medium Growth:**
- Tomato: $80 (5 days)
- Wheat: $100 (7 days)

**High Value, Slow Growth:**
- Melon: $250 (12 days)
- Pumpkin: $320 (13 days)

**Strategy Tips:**
1. Start with fast crops (Carrot)
2. Build money quickly
3. Later: plant high-value crops
4. Mix fast & slow for steady income

## Visual Indicators

### Progress Bars

**Crop Growth:**
```
▓▓▓▓▓▓▓░░░  70% complete
▓▓▓▓▓▓▓▓▓▓  100% - Ready to harvest!
```

**Energy Bar:**
```
▓▓▓▓▓▓▓▓▓▓  100% - Full energy (Green)
▓▓▓▓▓▓░░░░   60% - Medium (Green)
▓▓▓░░░░░░░   30% - Low (Orange)
▓░░░░░░░░░   10% - Critical (Red)
```

### Color Coding

**Seasons:**
- Spring: Pink/Cherry blossom
- Summer: Yellow/Bright
- Autumn: Orange/Red
- Winter: Blue/White

**UI Elements:**
- Purple gradient: Primary actions
- Green: Success, ready crops
- Red: Errors, warnings
- Grey: Disabled buttons
- White: Cards, backgrounds

### Animations

**On Hover:**
- Cards lift up slightly
- Buttons grow shadow
- Smooth transitions

**On Click:**
- Button press effect
- Notification slide-in
- Progress bar smooth fill

## Keyboard Shortcuts

Currently none, but potential future additions:
- `Space` - Sleep
- `W` - Water crops
- `H` - Harvest
- `P` - Plant first seed
- `R` - Reset game

## Mobile Experience

### Responsive Design
```
Desktop (>1200px)   Tablet (768-1200px)   Mobile (<768px)
┌──┬────┬──┐         ┌─────────┐           ┌─────────┐
│░░│████│░░│         │░░░░░░░░░│           │░░░░░░░░░│
│░░│████│░░│         │█████████│           │█████████│
│░░│████│░░│         │█████████│           │█████████│
└──┴────┴──┘         └─────────┘           └─────────┘
3-column layout      Single column         Stacked view
```

**Mobile Optimizations:**
- Larger touch targets
- Stacked layout
- Full-width cards
- Simplified spacing

## Accessibility

### Current Features:
- ✅ Color contrast (WCAG AA)
- ✅ Readable font sizes
- ✅ Clear button states
- ✅ Visual feedback

### Future Improvements:
- [ ] Keyboard navigation
- [ ] Screen reader support
- [ ] Focus indicators
- [ ] Skip links

## Tips & Tricks

### Efficiency Tips

1. **Batch Planting**
   ```
   Plant all seeds at once → Water all → Sleep
   Better than: Plant one, sleep, repeat
   ```

2. **Energy Management**
   ```
   Full energy = 100
   Plant 6 crops (90 energy)
   Save 10 for emergencies
   ```

3. **Season Planning**
   ```
   Day 25 of Spring → 3 days left
   Don't plant 5-day crops!
   Plant 3-day crops instead
   ```

4. **Money Optimization**
   ```
   Early game: Fast crops (Carrot)
   Mid game: Mix fast + medium
   Late game: High-value crops
   ```

### Visual Cues

**Watch for these:**
- ✓ Green border = Ready to harvest
- Red energy bar = Need to sleep soon
- Empty fields = Space to plant
- Full inventory = Plant some seeds!

### Common Mistakes

❌ **Don't:**
- Plant crops at season end (will die)
- Use all energy at once (save some)
- Forget to water (crops won't grow)
- Ignore ready crops (free money!)

✅ **Do:**
- Check seasons before planting
- Keep 10-20 energy reserve
- Water crops daily
- Harvest immediately when ready

## Troubleshooting UI Issues

### Problem: Can't Click Buttons

**Check:**
1. Is button greyed out? (Disabled state)
2. Do you have enough energy?
3. Are there crops to interact with?

### Problem: Progress Not Updating

**Solution:**
- Refresh the page
- Game auto-saves, progress is safe
- Check browser console for errors

### Problem: Game Looks Weird

**Try:**
1. Zoom to 100% (Ctrl+0 or Cmd+0)
2. Use modern browser (Chrome, Firefox, Safari)
3. Clear browser cache
4. Check screen width (responsive design)

## Advanced Features

### LocalStorage Inspection

**View your save data:**
1. Open DevTools (F12)
2. Application → Local Storage
3. Find key: `harvest-game-state`
4. See JSON game state

**Manual Edit (Advanced):**
```javascript
// In browser console
localStorage.setItem('harvest-game-state', '{"day":100,"player":{...}}')
location.reload()
```

### Performance Monitoring

**Check FPS:**
```javascript
// In browser console
let lastTime = performance.now()
requestAnimationFrame(function loop() {
  const now = performance.now()
  console.log('FPS:', 1000 / (now - lastTime))
  lastTime = now
  requestAnimationFrame(loop)
})
```

## Customization Ideas

Want to modify the UI? Here's what you can change:

### Colors (`web/src/App.css`)
```css
/* Change theme colors */
--primary: #667eea;     /* Purple */
--success: #48bb78;     /* Green */
--error: #f56565;       /* Red */
--warning: #ff9800;     /* Orange */
```

### Layout (`web/src/App.tsx`)
```tsx
// Reorder sections
<div className="game-container">
  {/* Move panels around */}
  <ActionsPanel />
  <MainContent />
  <StatsPanel />
</div>
```

### Features (`web/src/useGame.ts`)
```typescript
// Add new game actions
const buySeeds = useCallback(() => {
  // Implementation
}, [gameEngine])
```

---

## Quick Reference

| Action | Button | Energy Cost | Effect |
|--------|--------|-------------|--------|
| Plant | [Plant] | 15 per crop | Seed → Field |
| Water | 💧 Water | 15 per crop | Grows crops |
| Harvest | 🌾 Harvest | 0 | Earn money |
| Sleep | 💤 Sleep | 0 | Restore energy |
| Advance | ⏭️ Advance | 0 | Skip day |

| Season | Days | Emoji | Best Crops |
|--------|------|-------|------------|
| Spring | 1-28 | 🌸 | Carrot, Potato |
| Summer | 29-56 | ☀️ | Tomato, Melon |
| Autumn | 57-84 | 🍂 | Pumpkin, Corn |
| Winter | 85-112 | ❄️ | Wheat, Winter Seeds |

---

**Enjoy your farming! 🌾** For technical details, see [web/README.md](README.md)