pub mod player;
pub mod children;
mod person_animations;

pub enum AnimState {
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
