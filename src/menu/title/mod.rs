mod helpers;

use ggez::Context;
use noframe::geo::prelude::*;

use self::helpers::*;
use super::Menu;
use super::buttons::Button;
use super::ButtonType;
use animation::Animation;

pub struct TitleMenuManager {
  pub title: TitleMenu
}

impl TitleMenuManager {
  pub fn new(ctx: &mut Context, size: Size) -> Self {
    Self {
      title: TitleMenu::new(ctx, Point::new(0.0, 0.0), size)
    }
  }
}

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
      size:      size.clone(),
      origin:    Origin::TopLeft,
      buttons:   new_buttons(ctx, &size),
      animation: new_animation(ctx),
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
  fn buttons(&self) -> &Vec<Button> {
    &self.buttons
  }
  fn buttons_mut(&mut self) -> &mut Vec<Button> {
    &mut self.buttons
  }
  fn animation(&self) -> Option<&Animation> {
    Some(&self.animation)
  }
  fn animation_mut(&mut self) -> Option<&mut Animation> {
    Some(&mut self.animation)
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
