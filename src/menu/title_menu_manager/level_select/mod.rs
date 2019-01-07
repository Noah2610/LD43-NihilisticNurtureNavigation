mod helpers;

use ggez::{
  Context,
  GameResult,
};
use noframe::geo::prelude::*;

use self::helpers::*;
use menu::prelude::*;

pub struct LevelSelectMenu {
  point:     Point,
  size:      Size,
  origin:    Origin,
  animation: Animation,
  buttons:   Vec<Button>,
  clicked:   Option<ButtonType>,
}

impl LevelSelectMenu {
  pub fn new(ctx: &mut Context, size: Size) -> GameResult<Self> {
    Ok(Self {
      point:     Point::new(0.0, 0.0),
      size:      size.clone(),
      origin:    Origin::TopLeft,
      animation: new_animation(ctx, &size),
      buttons:   new_buttons(ctx, &size)?,
      clicked:   None,
    })
  }
}

impl Mask for LevelSelectMenu {
  fn point(&self)         -> &Point     { &self.point     }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size      }
  fn origin(&self)        -> &Origin    { &self.origin    }
}

impl Menu for LevelSelectMenu {
  fn buttons(&self) -> Vec<&Button> {
    self.buttons.iter().map( |button| button ).collect()
  }
  fn buttons_mut(&mut self) -> Vec<&mut Button> {
    self.buttons.iter_mut().map( |button| button ).collect()
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
