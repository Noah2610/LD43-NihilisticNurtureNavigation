pub mod player;
pub mod children;
mod person_animations;

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
