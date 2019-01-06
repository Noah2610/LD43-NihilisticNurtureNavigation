pub mod prelude {
  pub use super::Button;
  pub use super::button_type::ButtonType;
}

pub mod button_type;

pub use self::button_type::ButtonType;

use ggez::{
  GameResult,
  Context
};
use noframe::geo::prelude::*;

use animation::Animation;
use animation::Facing;

pub struct Button {
  point:           Point,
  size:            Size,
  origin:          Origin,
  animation:       Animation,
  pub button_type: ButtonType
}

impl Button {
  pub fn new(ctx: &mut Context, point: Point, size: Size, button_type: ButtonType, images: Vec<String>, delays: Vec<u64>) -> Self {
    Self::new_with_origin(
      ctx,
      point,
      size,
      Origin::TopLeft,
      button_type,
      images,
      delays
      )
  }

  pub fn new_with_origin(ctx: &mut Context, point: Point, size: Size, origin: Origin, button_type: ButtonType, images: Vec<String>, delays: Vec<u64>) -> Self {
    Self {
      point,
      size,
      origin,
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
    let size = self.size().clone();
    let top_left = self.top_left();
    self.animation_mut().draw(ctx, &top_left, &size, &Facing::Right)
  }
}

impl Mask for Button {
  fn point(&self)         -> &Point     { &self.point     }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size      }
  fn origin(&self)        -> &Origin    { &self.origin    }
}
