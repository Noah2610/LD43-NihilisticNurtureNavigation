// pub mod title;

use ggez::{
  GameResult,
  Context
};
use noframe::geo::prelude::*;

use animation::Animation;
use animation::Facing;
use super::ButtonType;

pub struct Button {
  point:           Point,
  size:            Size,
  origin:          Origin,
  animation:       Animation,
  pub button_type: ButtonType
}

impl Button {
  pub fn new(ctx: &mut Context, point: Point, size: Size, button_type: ButtonType, images: Vec<String>, delays: Vec<u64>) -> Self {
    Self {
      point,
      size,
      origin:    Origin::TopLeft,
      animation: Animation::new(
        ctx, images, delays
      ),
      button_type
    }
  }

  pub fn animation(&self) -> &Animation {
    &self.animation
  }

  pub fn animation_mut(&mut self) -> &mut Animation {
    &mut self.animation
  }

  pub fn update(&mut self) -> GameResult<()> {
    self.animation_mut().update()
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let point = self.point().clone();
    let size = self.size().clone();
    self.animation_mut().draw(ctx, &point, &size, &Facing::Right)
  }
}

impl Mask for Button {
  fn point(&self)         -> &Point     { &self.point     }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size      }
  fn origin(&self)        -> &Origin    { &self.origin    }
}
