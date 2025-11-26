import { useEffect, useState, useCallback } from "react";
import type { GameInfo, GameResult } from "./types";

interface WasmGameEngine {
  toJson(): string;
  getInfo(): string;
  sleep(): string;
  plantCrop(cropIndex: number): string;
  waterCrops(): string;
  harvestCrops(): string;
  advanceDay(): string;
  getDay(): number;
  getCurrentSeason(): string;
  buySeed(seedName: string): string;
  getAvailableSeeds(): string;
}

interface WasmGameEngineConstructor {
  new (playerName: string): WasmGameEngine;
  fromJson(json: string): WasmGameEngine;
}

interface WasmModule {
  WasmGameEngine: WasmGameEngineConstructor;
  default(): Promise<void>;
}

const STORAGE_KEY = "harvest-game-state";

export function useGame() {
  const [gameEngine, setGameEngine] = useState<WasmGameEngine | null>(null);
  const [gameInfo, setGameInfo] = useState<GameInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [showNameInput, setShowNameInput] = useState(false);
  const [playerName, setPlayerName] = useState("");

  const updateGameInfo = useCallback((engine: WasmGameEngine) => {
    try {
      const infoJson = engine.getInfo();
      const info: GameInfo = JSON.parse(infoJson);
      setGameInfo(info);
    } catch (e) {
      console.error("Failed to get game info:", e);
      setError("Failed to update game state");
    }
  }, []);

  // Initialize WASM and load game
  useEffect(() => {
    let mounted = true;

    async function initWasm() {
      try {
        const wasmModule = (await import(
          "./wasm/p2p_harvest_game.js"
        )) as unknown as WasmModule;

        // Initialize WASM - this is required before using any exports
        await wasmModule.default();

        if (!mounted) return;

        // Try to load saved game state
        const savedState = localStorage.getItem(STORAGE_KEY);
        let engine: WasmGameEngine;

        if (savedState) {
          try {
            engine = wasmModule.WasmGameEngine.fromJson(savedState);
            setMessage("Game loaded from save!");
            setGameEngine(engine);
            updateGameInfo(engine);
            setLoading(false);
          } catch (e) {
            console.error("Failed to load saved game:", e);
            // Save corrupted - ask for new name
            setShowNameInput(true);
            setLoading(false);
          }
        } else {
          // No saved game - ask for player name
          setShowNameInput(true);
          setLoading(false);
        }
      } catch (err) {
        console.error("Failed to initialize WASM:", err);
        setError("Failed to load game. Please refresh the page.");
        setLoading(false);
      }
    }

    initWasm();

    return () => {
      mounted = false;
    };
  }, [updateGameInfo]);

  // Save game state whenever it changes
  useEffect(() => {
    if (gameEngine) {
      try {
        const state = gameEngine.toJson();
        localStorage.setItem(STORAGE_KEY, state);
      } catch (e) {
        console.error("Failed to save game:", e);
      }
    }
  }, [gameInfo, gameEngine]);

  const executeAction = useCallback(
    (action: () => string, successMsg: string) => {
      if (!gameEngine) return;

      try {
        const resultJson = action();
        const result: GameResult = JSON.parse(resultJson);

        if ("Success" in result) {
          setMessage(successMsg);
          setError(null);
        } else if ("Error" in result) {
          setError(result.Error);
          setMessage(null);
        }

        updateGameInfo(gameEngine);
      } catch (e) {
        console.error("Action failed:", e);
        setError("Action failed. Please try again.");
      }
    },
    [gameEngine, updateGameInfo],
  );

  const sleep = useCallback(() => {
    executeAction(
      () => gameEngine!.sleep(),
      "💤 You slept well! Energy restored.",
    );
  }, [gameEngine, executeAction]);

  const plantCrop = useCallback(
    (cropIndex: number) => {
      executeAction(
        () => gameEngine!.plantCrop(cropIndex),
        "🌱 Crop planted successfully!",
      );
    },
    [gameEngine, executeAction],
  );

  const waterCrops = useCallback(() => {
    executeAction(() => gameEngine!.waterCrops(), "💧 Crops watered!");
  }, [gameEngine, executeAction]);

  const harvestCrops = useCallback(() => {
    executeAction(() => gameEngine!.harvestCrops(), "🌾 Harvest complete!");
  }, [gameEngine, executeAction]);

  const advanceDay = useCallback(() => {
    executeAction(() => gameEngine!.advanceDay(), "🌅 Day advanced!");
  }, [gameEngine, executeAction]);

  const buySeed = useCallback(
    (seedName: string) => {
      executeAction(
        () => gameEngine!.buySeed(seedName),
        `🌱 Purchased ${seedName} seed!`,
      );
    },
    [gameEngine, executeAction],
  );

  const getAvailableSeeds = useCallback(() => {
    if (!gameEngine) return [];
    try {
      const seedsJson = gameEngine.getAvailableSeeds();
      return JSON.parse(seedsJson);
    } catch (e) {
      console.error("Failed to get available seeds:", e);
      return [];
    }
  }, [gameEngine]);

  const startNewGame = useCallback(
    async (name: string) => {
      try {
        const wasmModule = (await import(
          "./wasm/p2p_harvest_game.js"
        )) as unknown as WasmModule;
        await wasmModule.default();
        const newEngine = new wasmModule.WasmGameEngine(name);
        setGameEngine(newEngine);
        updateGameInfo(newEngine);
        setShowNameInput(false);
        setPlayerName("");
        setMessage(`Welcome, ${name}! Happy farming! 🌾`);
        setError(null);
      } catch (e) {
        console.error("Failed to start new game:", e);
        setError("Failed to start new game");
      }
    },
    [updateGameInfo],
  );

  const handleNameSubmit = useCallback(
    (name: string) => {
      const trimmedName = name.trim();
      if (!trimmedName) {
        setError("Please enter a name!");
        return;
      }
      startNewGame(trimmedName);
    },
    [startNewGame],
  );

  const resetGame = useCallback(() => {
    if (!gameEngine) return;

    const confirmed = window.confirm(
      "Are you sure you want to reset your game? All progress will be lost!",
    );
    if (!confirmed) return;

    localStorage.removeItem(STORAGE_KEY);
    setGameEngine(null);
    setGameInfo(null);
    setShowNameInput(true);
    setPlayerName("");
    setMessage("Game reset! Enter your name to start fresh.");
  }, [gameEngine]);

  const clearMessage = useCallback(() => {
    setMessage(null);
    setError(null);
  }, []);

  return {
    gameInfo,
    loading,
    error,
    message,
    sleep,
    plantCrop,
    waterCrops,
    harvestCrops,
    advanceDay,
    buySeed,
    getAvailableSeeds,
    resetGame,
    clearMessage,
    showNameInput,
    playerName,
    setPlayerName,
    handleNameSubmit,
  };
}
