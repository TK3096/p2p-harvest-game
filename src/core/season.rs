use serde::{Deserialize, Serialize};

const DAYS_PER_SEASON: u32 = 10;
const DAYS_PER_YEAR: u32 = DAYS_PER_SEASON * 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn from_day(day: u32) -> Self {
        let year_day = ((day - 1) % (DAYS_PER_SEASON * 4)) + 1;

        match year_day {
            d if d <= DAYS_PER_SEASON => Season::Spring,
            d if d <= DAYS_PER_SEASON * 2 => Season::Summer,
            d if d <= DAYS_PER_SEASON * 3 => Season::Autumn,
            _ => Season::Winter,
        }
    }

    pub fn day_in_season(day: u32) -> u32 {
        let year_day = ((day - 1) % (DAYS_PER_SEASON * 4)) + 1;

        if year_day <= DAYS_PER_SEASON {
            year_day
        } else if year_day <= DAYS_PER_SEASON * 2 {
            year_day - DAYS_PER_SEASON
        } else if year_day <= DAYS_PER_SEASON * 3 {
            year_day - DAYS_PER_SEASON * 2
        } else {
            year_day - DAYS_PER_SEASON * 3
        }
    }

    pub fn year(day: u32) -> u32 {
        ((day - 1) / DAYS_PER_YEAR) + 1
    }

    pub fn icon(&self) -> &str {
        match self {
            Season::Spring => "🌸",
            Season::Summer => "☀️",
            Season::Autumn => "🍂",
            Season::Winter => "❄️",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Autumn => "Autumn",
            Season::Winter => "Winter",
        }
    }
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.icon(), self.name())
    }
}
