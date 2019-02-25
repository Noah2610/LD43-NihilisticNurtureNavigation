pub mod prelude {
  pub use super::Button;
  pub use super::button_type::ButtonType;
  pub use super::builder::ButtonBuilder;
}

pub mod button_type;
pub mod builder;
mod button_text;

pub use self::button_type::ButtonType;

use ggez::{
  GameResult,
  Context,
};
use noframe::geo::prelude::*;

use self::button_text::prelude::*;
use animation::Animation;
use animation::Facing;

pub struct Button {
  point:           Point,
  size:            Size,
  origin:          Origin,
  animation:       Animation,
  pub button_type: ButtonType,
  facing:          Facing,
  text:            Option<ButtonText>,
}

impl Button {
  pub fn new(
    point:           Point,
    size:            Size,
    origin:          Origin,
    button_type:     ButtonType,
    animation:       Animation,
    facing:          Facing,
    text:            Option<ButtonText>
  ) -> Self {
    Self {
      point: point.clone(),
      size:  size.clone(),
      origin,
      animation,
      button_type,
      facing,
      text,
    }
  }

  pub fn animation(&self) -> &Animation {
    &self.animation
  }

  pub fn animation_mut(&mut self) -> &mut Animation {
    &mut self.animation
  }

  pub fn mouse_in(&mut self) {
    unimplemented!();  // TODO
  }

  pub fn mouse_out(&mut self) {
    unimplemented!();  // TODO
  }

  pub fn update(&mut self) -> GameResult<()> {
    self.animation_mut().update()
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let size = self.size().clone();
    let top_left = self.top_left();
    let facing = self.facing.clone();
    self.animation_mut().draw(ctx, &top_left, &size, &facing)?;
    if let Some(text) = &mut self.text {
      text.draw(ctx)?;
    }
    Ok(())
  }
}

impl Mask for Button {
  fn point(&self)         -> &Point     { &self.point     }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size      }
  fn origin(&self)        -> &Origin    { &self.origin    }
}
