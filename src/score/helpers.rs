use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;
use std::ops;

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

  pub fn with(commands: HashMap<ChildType, ScoreType>) -> Self {
    Self { commands }
  }

  pub fn commands(&self) -> &HashMap<ChildType, ScoreType> {
    &self.commands
  }

  pub fn commands_for(&self, child: ChildType) -> Option<ScoreType> {
    self.commands.iter().find( |(&k, v)| k == child ).map( |o| *o.1 )
  }

  pub fn total(&self) -> ScoreType {
    self.commands.values().sum::<ScoreType>()
  }

  pub fn commanded(&mut self, child: ChildType) {
    *self.commands.entry(child).or_insert(0) += 1;
  }

  pub fn clear(&mut self) {
    self.commands.clear();
  }
}

// IMPLEMENTATIONS

impl From<Vec<&Score>> for Score {
  fn from(scores: Vec<&Score>) -> Self {
    let mut score_acc = Score::new();
    for score in scores {
      score_acc += score;
    }
    score_acc
  }
}

impl ops::AddAssign<&Score> for Score {
  fn add_assign(&mut self, other: &Score) {
    use self::ChildType::*;
    // player
    self.times_saved_player += other.times_saved_player();
    for &child in &[Larry, Thing, Bloat] {
      // children
      if let Some(other_saved) = other.times_saved_child(child) {
        *self.times_saved_children.entry(child).or_insert(0) += other_saved;
      }
      // commands
      self.commands_counter += &other.commands_counter;
    }
  }
}

impl ops::AddAssign<&ChildCommandsCounter> for ChildCommandsCounter {
  fn add_assign(&mut self, other: &ChildCommandsCounter) {
    use self::ChildType::*;
    for &child in &[Larry, Thing, Bloat] {
      if let Some(other_commands) = other.commands_for(child) {
        *self.commands.entry(child).or_insert(0) += other_commands;
      }
    }
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
