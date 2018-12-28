use std::collections::HashMap;

use settings::score::*;
use persons::children::ChildType;

pub type ScoreType = u32;

pub mod prelude {
  pub use super::Score;
  pub use super::ScoreType;
}

pub struct Score {
  times_saved_player:   ScoreType,
  times_saved_children: HashMap<ChildType, ScoreType>,
}

impl Score {
  pub fn new() -> Self {
    Self {
      times_saved_player:   0,
      times_saved_children: HashMap::new(),
    }
  }

  pub fn score(&self) -> ScoreType {
    (self.times_saved_player * PLAYER_SCORE_REWARD) +
      (self.times_saved_children.values().sum::<ScoreType>() * CHILD_SCORE_REWARD)
  }

  pub fn times_saved_player(&self) -> ScoreType {
    self.times_saved_player
  }

  pub fn saved_player(&mut self) {
    self.times_saved_player += 1;
  }

  pub fn times_saved_children(&self) -> &HashMap<ChildType, ScoreType> {
    &self.times_saved_children
  }

  pub fn saved_child(&mut self, child_type: ChildType) {
    *self.times_saved_children.entry(child_type).or_insert(0) += 1;
  }
}
