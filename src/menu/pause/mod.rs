pub mod prelude {
  pub use super::PauseMenu;
  pub use super::super::Menu;
}

mod helpers;

use ggez::{
  Context
};
use noframe::geo::prelude::*;

use self::helpers::*;
use super::prelude::*;

pub struct PauseMenu {
  point:        Point,
  size:         Size,
  origin:       Origin,
  animation:    Animation,
  buttons:      Vec<Button>,
  clicked:      Option<ButtonType>,
}

impl PauseMenu {
  pub fn new(ctx: &mut Context, window_size: Size) -> Self {
    Self {
      point:     Point::new(0.0, window_size.w),
      size:      window_size.clone(),
      origin:    Origin::TopLeft,
      animation: new_animation(ctx),
      buttons:   new_buttons(ctx, &window_size),
      clicked:   None,
    }
  }
}

impl Mask for PauseMenu {
  fn point(&self) -> &Point {
    &self.point
  }
  fn point_mut(&mut self) -> &mut Point {
    &mut self.point
  }
  fn size(&self) -> &Size {
    &self.size
  }
  fn origin(&self) -> &Origin {
    &self.origin
  }
}

impl Menu for PauseMenu {
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

  fn clicked(&mut self, btn_type: ButtonType) {
    self.clicked = Some(btn_type);
  }

  fn get_clicked(&self) -> &Option<ButtonType> {
    &self.clicked
  }

  fn clear_clicked(&mut self) {
    self.clicked = None;
  }

  fn has_animation(&self) -> bool {
    false
  }
}
