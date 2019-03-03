mod helpers;

use ggez::{
  Context,
  GameResult,
};
use noframe::geo::prelude::*;

use menu::prelude::*;
use self::helpers::*;

pub struct ThankYouMenu {
  point:     Point,
  size:      Size,
  origin:    Origin,
  animation: Animation,
  buttons:   Vec<Button>,
  clicked:   Option<ButtonType>,
}

impl ThankYouMenu {
  pub fn new(ctx: &mut Context, window_size: &Size) -> GameResult<Self> {
    Ok(Self {
      point:     Point::new(0.0, 0.0),
      size:      window_size.clone(),
      origin:    Origin::TopLeft,
      animation: new_animation(ctx)?,
      buttons:   new_buttons(ctx, window_size)?,
      clicked:   None,
    })
  }
}

impl Mask for ThankYouMenu {
  fn point(&self)         -> &Point     { &self.point }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size }
  fn origin(&self)        -> &Origin    { &self.origin }
}

impl Menu for ThankYouMenu {
  fn buttons(&self) -> Vec<&Button> {
    if self.animation.played() > 0 {
      self.buttons.iter().map( |button| button ).collect()
    } else { Vec::new() }
  }
  fn buttons_mut(&mut self) -> Vec<&mut Button> {
    if self.animation.played() > 0 {
      self.buttons.iter_mut().map( |button| button ).collect()
    } else { Vec::new() }
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
