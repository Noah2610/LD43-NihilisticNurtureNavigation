pub mod player;
pub mod children;
mod person_animations;

use noframe::entity::prelude::*;
use id_generator::IdGenerator;

enum AnimState {
  Walk,
  Idle,
  Jump,
  Fall
}

#[derive(PartialEq)]
enum Axis {
  X,
  Y
}

enum WalkDirection {
  Still,
  Left,
  Right
}

pub trait Person: Entity + Velocity + IdGenerator {
  fn is_solid(&self) -> bool;
  fn solidify(&mut self);
}
