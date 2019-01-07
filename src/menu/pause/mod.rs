pub mod prelude {
  pub use super::PauseMenu;
  pub use super::super::Menu;
}

mod helpers;

use ggez::{
  Context,
  GameResult,
};
use noframe::geo::prelude::*;

use self::helpers::*;
use super::prelude::*;
use animation::AnimationRect;

pub struct PauseMenu {
  point:        Point,
  size:         Size,
  origin:       Origin,
  buttons:      Vec<Button>,
  clicked:      Option<ButtonType>,
  title:        AnimationRect,
}

impl PauseMenu {
  pub fn new(ctx: &mut Context, window_size: Size) -> Self {
    Self {
      point:     Point::new(0.0, window_size.w),
      size:      window_size.clone(),
      origin:    Origin::TopLeft,
      buttons:   new_buttons(ctx, &window_size),
      clicked:   None,
      title:     new_title(ctx, &window_size),
    }
  }
}

impl Mask for PauseMenu {
  fn point(&self)         -> &Point     { &self.point     }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size      }
  fn origin(&self)        -> &Origin    { &self.origin    }
}

impl Menu for PauseMenu {
  fn update(&mut self) -> GameResult<()> {
    self.title.update()?;
    self.update_menu()?;
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.title.draw(ctx)?;
    self.draw_menu(ctx)?;
    Ok(())
  }

  fn buttons(&self) -> Vec<&Button> {
    self.buttons.iter().map( |button| button ).collect()
  }
  fn buttons_mut(&mut self) -> Vec<&mut Button> {
    self.buttons.iter_mut().map( |button| button ).collect()
  }
  fn animation(&self) -> Option<&Animation> {
    None
  }
  fn animation_mut(&mut self) -> Option<&mut Animation> {
    None
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
}
