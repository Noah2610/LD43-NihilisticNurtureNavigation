use ggez::{
  GameResult,
  Context
};
use noframe::geo::prelude::*;

use settings::buttons::IMAGES;
use super::Button;
use animation::Animation;

pub struct StartButton {
  point:     Point,
  size:      Size,
  origin:    Origin,
  animation: Animation
}

impl StartButton {
}

impl Mask for StartButton {
}

impl Button for StartButton {
  fn animation(&self) -> &Animation {
    &self.animation
  }
  fn animation_mut(&mut self) -> &mut Animation {
    &mut self.animation
  }
}
