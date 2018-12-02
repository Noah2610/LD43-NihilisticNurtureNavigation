use std::f32;

use rand::Rng;

pub trait IdGenerator {
  fn id(&self) -> f32;
  fn set_id(&mut self, id: f32);

  fn generate_id(&mut self) {
    let id = rand::thread_rng().gen_range(f32::MIN, f32::MAX);
  }
}
