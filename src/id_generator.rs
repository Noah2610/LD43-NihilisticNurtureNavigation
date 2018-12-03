//! Stupid simple "ID Generator" (picks a random number between 0 and a lot)

pub mod prelude {
  pub use std::u32;
  pub use super::{ IdGenerator, IdType, generate_id };
}

use std::u32;

use rand::Rng;

pub type IdType = u32;

pub fn generate_id() -> IdType {
  rand::thread_rng().gen_range(u32::MIN, u32::MAX)
}

pub trait IdGenerator {
  fn id(&self) -> IdType;
  fn set_id(&mut self, id: IdType);

  fn generate_id(&mut self) {
    self.set_id(rand::thread_rng().gen_range(100_u32, u32::MAX));
  }

  fn is<T: IdGenerator>(&self, other: &T) -> bool {
    self.id() == other.id()
  }

  fn has_id(&self, id: IdType) -> bool {
    self.id() == id
  }
}
