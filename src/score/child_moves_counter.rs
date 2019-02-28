use std::collections::HashMap;
use std::ops;

use persons::children::ChildType;
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
    self.moves.iter().find( |(&k, _)| k == child ).map( |o| *o.1 )
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
