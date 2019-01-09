mod defaults {
  use ggez::Context;
  use noframe::geo::prelude::*;
  use animation::Animation;
  use settings::res::MISSING_IMAGE;

  pub const POINT:  Point  = Point { x: 0.0,  y: 0.0  };
  pub const SIZE:   Size   = Size  { w: 64.0, h: 64.0 };
  pub const ORIGIN: Origin = Origin::TopLeft;

  pub fn animation(ctx: &mut Context) -> Animation {
    Animation::new(
      ctx,
      vec![MISSING_IMAGE.to_string()],
      vec![1000]
    )
  }
}

use ggez::{
  Context,
  GameResult,
  GameError,
  graphics,
};
use noframe::geo::prelude::*;

use super::Button;
use super::button_type::ButtonType;
use super::button_text::ButtonText;
use animation::Animation;

pub struct ButtonBuilder<'a> {
  ctx:         &'a mut Context,
  point:       Point,
  size:        Size,
  origin:      Origin,
  animation:   Animation,
  button_type: Option<ButtonType>,
  text:        Option<ButtonText>,
}

impl<'a> ButtonBuilder<'a> {
  pub fn new(ctx: &'a mut Context) -> Self {
    let animation = defaults::animation(ctx);
    Self {
      ctx,
      point:       defaults::POINT,
      size:        defaults::SIZE,
      origin:      defaults::ORIGIN,
      animation,
      button_type: None,
      text:        None,
    }
  }

  pub fn point(mut self, point: Point) -> Self {
    self.point = point;
    self
  }
  pub fn point_from(self, x: NumType, y: NumType) -> Self {
    self.point(Point::new(x, y))
  }

  pub fn size(mut self, size: Size) -> Self {
    self.size = size;
    self
  }
  pub fn size_from(self, w: NumType, h: NumType) -> Self {
    self.size(Size::new(w, h))
  }

  pub fn origin(mut self, origin: Origin) -> Self {
    self.origin = origin;
    self
  }

  pub fn animation(mut self, animation: Animation) -> Self {
    self.animation = animation;
    self
  }
  pub fn animation_from(self, images: Vec<String>, delays: Vec<u64>) -> Self {
    let animation = Animation::new(self.ctx, images, delays);
    self.animation(animation)
  }

  pub fn button_type(mut self, button_type: ButtonType) -> Self {
    self.button_type = Some(button_type);
    self
  }

  pub fn text(mut self, text: ButtonText) -> Self {
    self.text = Some(text);
    self
  }
  pub fn text_from(self, point: Point, size: Size, origin: Origin, text: graphics::Text) -> Self {
    self.text(ButtonText::new(point, size, origin, text))
  }

  pub fn build(self) -> GameResult<Button> {
    self.validate()?;
    Ok(Button::new(
        self.point,
        self.size,
        self.origin,
        self.button_type.unwrap(),
        self.animation,
        self.text
    ))
  }

  fn validate(&self) -> GameResult<()> {
    if let None = self.button_type {
      Err(GameError::from("ButtonBuilder needs at least a ButtonType, use method `.button_type(ButtonType)`".to_string()))
    } else {
      Ok(())
    }
  }
}
