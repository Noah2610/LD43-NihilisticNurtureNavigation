mod animations;

mod jump_pad;
mod switch;

pub mod prelude {
  pub use super::Interactable;
  pub use super::jump_pad::JumpPad;
  pub use super::switch::Switch;
}

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;

use persons::Person;

pub trait Interactable: Entity {
  fn trigger<T: Person>(&mut self, person: &mut T);
}
