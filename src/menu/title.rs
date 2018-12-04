use ggez::{
  GameResult,
  Context
};
use noframe::geo::prelude::*;

use settings::menus::IMAGES;
use settings::buttons;
use super::Menu;
use super::buttons::Button;
use super::ButtonType;
use animation::Animation;
use animation::Facing;

pub struct TitleMenu {
  point:       Point,
  size:        Size,
  origin:      Origin,
  buttons:     Vec<Button>,
  animation:   Animation,
  clicked:     Option<ButtonType>
}

impl TitleMenu {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point,
      size,
      origin:    Origin::TopLeft,
      buttons:   vec![
        Button::new(
          ctx,
          Point::new(128.0, 128.0),
          Size::new(128.0, 64.0),
          ButtonType::Start,
          vec![::join_str(buttons::IMAGES, "startgame_title.png")],
          vec![1000]
        )
      ],
      animation: Animation::new(
        ctx,
        vec![::join_str(IMAGES, "title.png")],
        vec![1000]
      ),
      clicked: None
    }
  }
}

impl Mask for TitleMenu {
  fn point(&self)         -> &Point     { &self.point     }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size      }
  fn origin(&self)        -> &Origin    { &self.origin    }
}

impl Menu for TitleMenu {
  fn has_animation(&self) -> bool { true }
  fn buttons(&self) -> &Vec<Button> {
    &self.buttons
  }
  fn buttons_mut(&mut self) -> &mut Vec<Button> {
    &mut self.buttons
  }
  fn animation(&self) -> &Animation {
    &self.animation
  }
  fn animation_mut(&mut self) -> &mut Animation {
    &mut self.animation
  }
  fn clicked(&mut self, button_type: ButtonType) {
    self.clicked = Some(button_type);
  }
  fn get_clicked(&self) -> &Option<ButtonType> {
    &self.clicked
  }
  fn clear_clicked(&mut self) {
    self.clicked = None;
  }
}
