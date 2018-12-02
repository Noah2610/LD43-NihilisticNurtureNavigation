//! Stupid simple "ID Generator" (picks a random number between 0 and a lot)

use std::u64;

use rand::Rng;

pub trait IdGenerator {
  fn id(&self) -> u64;
  fn set_id(&mut self, id: u64);

  fn generate_id(&mut self) {
    self.set_id(rand::thread_rng().gen_range(u64::MIN, u64::MAX));
  }

  fn is(&self, id: u64) -> bool {
    self.id() == id
  }
}
