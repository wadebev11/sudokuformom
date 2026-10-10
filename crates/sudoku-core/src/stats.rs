use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::techniques::Difficulty;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DifficultyStats {
    pub solved: u32,
    pub best_seconds: Option<u64>,
    total_seconds: u64,
}

impl DifficultyStats {
    pub fn average_seconds(&self) -> Option<u64> {
        (self.solved > 0).then(|| self.total_seconds / u64::from(self.solved))
    }
}

/// Every finished puzzle counts, whether or not Hints or Mistake marking were used.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Stats(BTreeMap<Difficulty, DifficultyStats>);

impl Stats {
    pub fn for_difficulty(&self, difficulty: Difficulty) -> DifficultyStats {
        self.0.get(&difficulty).copied().unwrap_or_default()
    }

    pub fn record_solve(&mut self, difficulty: Difficulty, seconds: u64) {
        let stats = self.0.entry(difficulty).or_default();
        stats.solved += 1;
        stats.total_seconds += seconds;
        stats.best_seconds = Some(stats.best_seconds.map_or(seconds, |best| best.min(seconds)));
    }
}
