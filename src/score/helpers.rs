use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

use persons::children::ChildType;
use super::Score;
use super::ScoreType;

#[derive(Debug, Clone)]
pub struct ChildCommandsCounter {
  commands: HashMap<ChildType, ScoreType>,
}

impl ChildCommandsCounter {
  pub fn new() -> Self {
    Self {
      commands: HashMap::new(),
    }
  }

  pub fn commands(&self) -> &HashMap<ChildType, ScoreType> {
    &self.commands
  }

  pub fn commanded(&mut self, child: ChildType) {
    *self.commands.entry(child).or_insert(0) += 1;
  }

  pub fn clear(&mut self) {
    self.commands.clear();
  }
}

// SCORE IMPLEMENTATIONS

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

impl PartialEq for Score {
  fn eq(&self, other: &Score) -> bool {
    self.score() == other.score()
  }
}

impl PartialOrd for Score {
  fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
    Some(self.score().cmp(&other.score()))
  }
}

impl fmt::Display for Score {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.score())
  }
}
