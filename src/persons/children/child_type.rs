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
}
