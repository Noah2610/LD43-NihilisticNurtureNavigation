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
use noframe::entity::Entity;

use self::helpers::*;
use super::prelude::*;
use animation::Animation;
use color_rect::ColorRect;
use text_box::TextBox;

pub struct PauseMenu {
  buttons:      Vec<Button>,
  clicked:      Option<ButtonType>,
  title:        TextBox,
  rect:         ColorRect,
}

impl PauseMenu {
  pub fn new(ctx: &mut Context, window_size: Size) -> Self {
    Self {
      buttons:   new_buttons(ctx, &window_size),
      clicked:   None,
      title:     new_title(ctx, &window_size),
      rect:      new_color_rect(&window_size),
    }
  }
}

impl Mask for PauseMenu {
  fn point(&self)         -> &Point     { self.rect.point()     }
  fn point_mut(&mut self) -> &mut Point { self.rect.point_mut() }
  fn size(&self)          -> &Size      { self.rect.size()      }
  fn origin(&self)        -> &Origin    { self.rect.origin()    }
}

impl Menu for PauseMenu {
  fn update(&mut self) -> GameResult<()> {
    self.update_menu()?;
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.rect.draw(ctx)?;
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
