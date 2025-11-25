// TypeScript types matching the Rust game engine types

export enum Season {
  Spring = "Spring",
  Summer = "Summer",
  Autumn = "Autumn",
  Winter = "Winter",
}

export interface Crop {
  id: string;
  name: string;
  growth_days: number;
  sell_price: number;
  watered_days: number[];
  ready_harvest: boolean;
  energy_cost: number;
  seasons: Season[];
}

export interface GameInfo {
  day: number;
  player_name: string;
  player_money: number;
  player_energy: number;
  max_energy: number;
  current_season: Season;
  year: number;
  day_in_season: number;
  inventory: Crop[];
  fields: Crop[];
}

export interface SeasonChangeEvent {
  old_season: Season;
  new_season: Season;
  day: number;
  crops_died: string[];
}

export type GameEvent =
  | { DayAdvanced: { new_day: number; season_change: SeasonChangeEvent | null } }
  | { Slept: { old_day: number; new_day: number; season_change: SeasonChangeEvent | null } }
  | { CropPlanted: { crop_name: string; remaining_energy: number } }
  | { CropsWatered: { remaining_energy: number } }
  | { CropsHarvested: { earnings: number; total_money: number } }
  | { EnergyRestored: null };

export type GameResult =
  | { Success: GameEvent }
  | { Error: string };

export type GameCommand =
  | { Sleep: null }
  | { PlantCrop: { crop_index: number } }
  | { WaterCrops: null }
  | { HarvestCrops: null }
  | { AdvanceDay: null };
