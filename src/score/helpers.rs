use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;
use std::ops;

use persons::children::ChildType;
use super::Score;
use super::ScoreType;

#[derive(Debug, Clone)]
pub struct ChildMovesCounter {
  moves: HashMap<ChildType, ScoreType>,
}

impl ChildMovesCounter {
  pub fn new() -> Self {
    Self {
      moves: HashMap::new(),
    }
  }

  pub fn with(moves: HashMap<ChildType, ScoreType>) -> Self {
    Self { moves }
  }

  pub fn moves(&self) -> &HashMap<ChildType, ScoreType> {
    &self.moves
  }

  pub fn moves_for(&self, child: ChildType) -> Option<ScoreType> {
    self.moves.iter().find( |(&k, v)| k == child ).map( |o| *o.1 )
  }

  pub fn total(&self) -> ScoreType {
    self.moves.values().sum::<ScoreType>()
  }

  pub fn moved(&mut self, child: ChildType) {
    *self.moves.entry(child).or_insert(0) += 1;
  }

  pub fn clear(&mut self) {
    self.moves.clear();
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
      // moves
      self.moves_counter += &other.moves_counter;
    }
  }
}

impl ops::AddAssign<&ChildMovesCounter> for ChildMovesCounter {
  fn add_assign(&mut self, other: &ChildMovesCounter) {
    use self::ChildType::*;
    for &child in &[Larry, Thing, Bloat] {
      if let Some(other_moves) = other.moves_for(child) {
        *self.moves.entry(child).or_insert(0) += other_moves;
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
