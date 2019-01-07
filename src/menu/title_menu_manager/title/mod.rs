mod helpers;

use ggez::Context;
use noframe::geo::prelude::*;

use self::helpers::*;
use menu::prelude::*;
use animation::Animation;

pub struct TitleMenu {
  point:             Point,
  size:              Size,
  origin:            Origin,
  animation:         Animation,
  buttons:           Vec<Button>,
  clicked:           Option<ButtonType>,
  show_level_select: bool,
}

impl TitleMenu {
  pub fn new(ctx: &mut Context, size: Size) -> Self {
    Self {
      point:             Point::new(0.0, 0.0),
      size:              size.clone(),
      origin:            Origin::TopLeft,
      animation:         new_animation(ctx),
      buttons:           new_buttons(ctx, &size),
      clicked:           None,
      show_level_select: false,
    }
  }

  pub fn show_level_select(&mut self) {
    self.show_level_select = true;
  }
}

impl Mask for TitleMenu {
  fn point(&self)         -> &Point     { &self.point     }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size      }
  fn origin(&self)        -> &Origin    { &self.origin    }
}

impl Menu for TitleMenu {
  fn buttons(&self) -> Vec<&Button> {
    self.buttons.iter()
      .filter( |button| if let ButtonType::TitleLevelSelect = button.button_type {
        self.show_level_select
      } else { true })
      .collect()
  }
  fn buttons_mut(&mut self) -> Vec<&mut Button> {
    let show_level_select = self.show_level_select;
    self.buttons.iter_mut()
      .filter( |button| if let ButtonType::TitleLevelSelect = button.button_type {
        show_level_select
      } else { true })
      .collect()
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