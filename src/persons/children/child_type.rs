use std::cmp::Ordering;

use settings::child::{ names, shorts };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChildType {
  Larry,
  Bloat,
  Thing,
}

impl ChildType {
  pub fn from_short(name: &str) -> Option<Self> {
    use self::shorts::*;
    use self::ChildType::*;
    match name {
      LARRY => Some(Larry),
      BLOAT => Some(Bloat),
      THING => Some(Thing),
      _     => None,
    }
  }

  pub fn short(&self) -> String {
    use self::shorts::*;
    use self::ChildType::*;
    match self {
      Larry => String::from(LARRY),
      Bloat => String::from(BLOAT),
      Thing => String::from(THING),
    }
  }

  pub fn name(&self) -> String {
    use self::names::*;
    use self::ChildType::*;
    match self {
      Larry => String::from(LARRY),
      Bloat => String::from(BLOAT),
      Thing => String::from(THING),
    }
  }

  pub fn order_index(&self) -> usize {
    match self {
      ChildType::Larry => 0,
      ChildType::Bloat => 1,
      ChildType::Thing => 2,
    }
  }
}

impl PartialOrd for ChildType {
  fn partial_cmp(&self, other: &ChildType) -> Option<Ordering> {
    Some(self.order_index().cmp(&other.order_index()))
  }
}

impl Ord for ChildType {
  fn cmp(&self, other: &ChildType) -> Ordering {
    self.partial_cmp(other).unwrap_or(Ordering::Equal)
  }
}
