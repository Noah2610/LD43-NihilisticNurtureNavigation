use std::collections::HashMap;

use settings::score::*;
use persons::children::ChildType;

pub type ScoreType = u32;

pub mod prelude {
  pub use super::Score;
  pub use super::ScoreType;
}

#[derive(Debug, Clone)]
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

  pub fn semantic(&self) -> String {
    format!("Score: {}", self.score())
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

impl From<Vec<&Score>> for Score {
  fn from(scores: Vec<&Score>) -> Self {
    let mut score_acc = Score::new();
    for score in scores {
      (0 .. score.times_saved_player())
        .for_each( |_i| score_acc.saved_player() );
      for (child_type, saved) in score.times_saved_children() {
        (0 .. *saved)
          .for_each( |_i| score_acc.saved_child(child_type.clone()) )
      }
    }
    score_acc
  }
}
