pub mod prelude {
  pub use super::super::Menu;
  pub use super::StatsMenu;
  pub use super::helpers::StatsText;
  pub use super::helpers::TextOrigin;
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
use score::prelude::*;
use color_rect::ColorRect;

pub struct StatsMenu {
  rect:           ColorRect,
  buttons:        Vec<Button>,
  clicked:        Option<ButtonType>,
  texts:          StatsTexts,
}

impl StatsMenu {
  pub fn new(ctx: &mut Context, window_size: Size, score: Score, highscore: Option<Score>, is_final: bool) -> GameResult<Self> {
    let rect = new_color_rect(window_size.clone());
    Ok(Self {
      buttons:        new_buttons(ctx, rect.point(), rect.size(), is_final),
      clicked:        None,
      texts:          StatsTexts::new(ctx, score, highscore, rect.point(), rect.size(), is_final)?,
      rect,
    })
  }
}

impl Mask for StatsMenu {
  fn point(&self) -> &Point {
    self.rect.point()
  }
  fn point_mut(&mut self) -> &mut Point {
    self.rect.point_mut()
  }
  fn size(&self) -> &Size {
    self.rect.size()
  }
  fn origin(&self) -> &Origin {
    self.rect.origin()
  }
}

impl Menu for StatsMenu {
  fn update(&mut self) -> GameResult<()> {
    self.update_menu()?;
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.rect.draw(ctx)?;
    self.texts.draw(ctx)?;
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
