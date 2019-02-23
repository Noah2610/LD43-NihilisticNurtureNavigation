use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChildType {
  Larry,
  Thing,
  Bloat
}

impl ChildType {
  pub fn name(&self) -> String {
    match self {
      ChildType::Larry => String::from("Larry"),
      ChildType::Thing => String::from("The Thing"),
      ChildType::Bloat => String::from("Bloat"),
    }
  }

  pub fn order_index(&self) -> usize {
    match self {
      ChildType::Larry => 0,
      ChildType::Thing => 2,
      ChildType::Bloat => 1,
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
