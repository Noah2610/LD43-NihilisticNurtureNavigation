//! Stupid simple "ID Generator" (picks a random number between 0 and a lot)

pub mod prelude {
  pub use std::u64;
  pub use super::{ IdGenerator, IdType, generate_id };
}

use std::u64;

use rand::Rng;

pub type IdType = u64;

pub fn generate_id() -> IdType {
  rand::thread_rng().gen_range(u64::MIN, u64::MAX)
}

pub trait IdGenerator {
  fn id(&self) -> IdType;
  fn set_id(&mut self, id: IdType);

  fn generate_id(&mut self) {
    self.set_id(rand::thread_rng().gen_range(u64::MIN, u64::MAX));
  }

  fn is<T: IdGenerator>(&self, other: &T) -> bool {
    self.id() == other.id()
  }

  fn has_id(&self, id: IdType) -> bool {
    self.id() == id
  }
}
