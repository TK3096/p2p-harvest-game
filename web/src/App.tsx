import { useGame } from "./useGame";
import "./App.css";
import { useState } from "react";

function App() {
  const {
    gameInfo,
    loading,
    error,
    message,
    sleep,
    plantCrop,
    waterCrops,
    harvestCrops,
    advanceDay,
    resetGame,
    clearMessage,
    showNameInput,
    playerName,
    setPlayerName,
    handleNameSubmit,
  } = useGame();

  if (loading) {
    return (
      <div className="app">
        <div className="loading">
          <h2>🌱 Loading Harvest Game...</h2>
        </div>
      </div>
    );
  }

  // Show name input modal
  if (showNameInput) {
    return (
      <div className="app">
        <div className="modal-overlay">
          <div className="modal">
            <h2>🌾 Welcome to Harvest Game!</h2>
            <p>Enter your farmer name to begin:</p>
            <form
              onSubmit={(e) => {
                e.preventDefault();
                handleNameSubmit(playerName);
              }}
            >
              <input
                type="text"
                className="name-input"
                placeholder="Enter your name..."
                value={playerName}
                onChange={(e) => setPlayerName(e.target.value)}
                autoFocus
                maxLength={20}
              />
              <button type="submit" className="action-button">
                Start Farming! 🚜
              </button>
            </form>
            {error && <p className="error-text">{error}</p>}
          </div>
        </div>
      </div>
    );
  }

  if (!gameInfo) {
    return (
      <div className="app">
        <div className="error">
          <h2>❌ Failed to load game</h2>
          <p>Please refresh the page</p>
        </div>
      </div>
    );
  }

  const energyPercentage = (gameInfo.player_energy / gameInfo.max_energy) * 100;

  return (
    <div className="app">
      <header className="header">
        <h1>🌾 Harvest Game</h1>
        <button className="reset-button" onClick={resetGame}>
          Reset Game
        </button>
      </header>

      {/* Message/Error Display */}
      {(message || error) && (
        <div
          className={`notification ${error ? "error" : "success"}`}
          onClick={clearMessage}
        >
          {error || message}
          <span className="close">×</span>
        </div>
      )}

      <div className="game-container">
        {/* Player Stats */}
        <div className="stats-panel">
          <div className="stat-card">
            <h3>📅 Day {gameInfo.day}</h3>
            <p className="season">{gameInfo.current_season}</p>
            <p className="year-info">
              Year {gameInfo.year}, Day {gameInfo.day_in_season}
            </p>
          </div>

          <div className="stat-card">
            <h3>👤 {gameInfo.player_name}</h3>
            <p className="money">💰 ${gameInfo.player_money}</p>
          </div>

          <div className="stat-card">
            <h3>⚡ Energy</h3>
            <div className="energy-bar-container">
              <div
                className="energy-bar"
                style={{
                  width: `${energyPercentage}%`,
                  backgroundColor:
                    energyPercentage > 50
                      ? "#4caf50"
                      : energyPercentage > 25
                        ? "#ff9800"
                        : "#f44336",
                }}
              />
            </div>
            <p className="energy-text">
              {gameInfo.player_energy} / {gameInfo.max_energy}
            </p>
          </div>
        </div>

        {/* Main Game Area */}
        <div className="main-content">
          {/* Fields */}
          <div className="section">
            <h2>🌱 Fields ({gameInfo.fields.length})</h2>
            <div className="crops-grid">
              {gameInfo.fields.length === 0 ? (
                <p className="empty-message">No crops planted yet!</p>
              ) : (
                gameInfo.fields.map((crop) => (
                  <div
                    key={crop.id}
                    className={`crop-card ${crop.ready_harvest ? "ready" : ""}`}
                  >
                    <div className="crop-header">
                      <span className="crop-name">{crop.name}</span>
                      {crop.ready_harvest && (
                        <span className="ready-badge">✓ Ready!</span>
                      )}
                    </div>
                    <div className="crop-progress">
                      <div
                        className="progress-bar"
                        style={{
                          width: `${(crop.watered_days.length / crop.growth_days) * 100}%`,
                        }}
                      />
                    </div>
                    <p className="crop-info">
                      {crop.watered_days.length} / {crop.growth_days} days
                    </p>
                    <p className="crop-value">Sell: ${crop.sell_price}</p>
                  </div>
                ))
              )}
            </div>
          </div>

          {/* Inventory */}
          <div className="section">
            <h2>🎒 Inventory ({gameInfo.inventory.length})</h2>
            <div className="crops-grid">
              {gameInfo.inventory.length === 0 ? (
                <p className="empty-message">
                  No seeds available. Buy some from the market!
                </p>
              ) : (
                gameInfo.inventory.map((crop, index) => (
                  <div key={crop.id} className="crop-card inventory-item">
                    <div className="crop-header">
                      <span className="crop-name">{crop.name}</span>
                    </div>
                    <p className="crop-info">Growth: {crop.growth_days} days</p>
                    <p className="crop-value">Value: ${crop.sell_price}</p>
                    <p className="crop-seasons">
                      {crop.seasons
                        .map((s) => {
                          const emoji =
                            s === "Spring"
                              ? "🌸"
                              : s === "Summer"
                                ? "☀️"
                                : s === "Autumn"
                                  ? "🍂"
                                  : "❄️";
                          return emoji;
                        })
                        .join(" ")}
                    </p>
                    <button
                      className="action-button small"
                      onClick={() => plantCrop(index)}
                      disabled={gameInfo.player_energy < crop.energy_cost}
                    >
                      Plant (⚡{crop.energy_cost})
                    </button>
                  </div>
                ))
              )}
            </div>
          </div>
        </div>

        {/* Actions Panel */}
        <div className="actions-panel">
          <h2>⚔️ Actions</h2>

          <button
            className="action-button"
            onClick={waterCrops}
            disabled={gameInfo.fields.length === 0}
          >
            💧 Water Crops
          </button>

          <button
            className="action-button"
            onClick={harvestCrops}
            disabled={
              gameInfo.fields.length === 0 ||
              !gameInfo.fields.some((c) => c.ready_harvest)
            }
          >
            🌾 Harvest
          </button>

          <button className="action-button" onClick={sleep}>
            💤 Sleep
          </button>

          <button className="action-button secondary" onClick={advanceDay}>
            ⏭️ Advance Day
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
