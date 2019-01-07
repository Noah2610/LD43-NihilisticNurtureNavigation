pub mod prelude {
  pub use super::Button;
  pub use super::button_type::ButtonType;
}

pub mod button_type;
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
  text:            Option<ButtonText>,
}

impl Button {
  pub fn new(
    ctx:             &mut Context,
    point:           Point,
    size:            Size,
    origin:          Origin,
    button_type:     ButtonType,
    images:          Vec<String>,
    delays:          Vec<u64>,
    button_text_opt: Option<ButtonTextTuple>
  ) -> Self {
    Self {
      point: point.clone(),
      size:  size.clone(),
      origin,
      animation: Animation::new(
        ctx, images, delays
      ),
      button_type,
      text: button_text_opt
        .and_then( |tuple| Some( ButtonText::new(
              tuple.0,  // point
              tuple.1,  // size,
              tuple.2,  // origin
              tuple.3   // text
        ))),
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
    self.animation_mut().draw(ctx, &top_left, &size, &Facing::Right)?;
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
