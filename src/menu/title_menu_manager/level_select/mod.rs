mod helpers;

use ggez::{
  Context,
  GameResult,
};
use noframe::geo::prelude::*;
use noframe::entity::Entity;

use self::helpers::*;
use menu::prelude::*;
use color_rect::ColorRect;

pub struct LevelSelectMenu {
  buttons:   Vec<Button>,
  clicked:   Option<ButtonType>,
  rect:      ColorRect,
}

impl LevelSelectMenu {
  pub fn new(ctx: &mut Context, size: Size) -> GameResult<Self> {
    Ok(Self {
      buttons:   new_buttons(ctx, &size)?,
      clicked:   None,
      rect:      new_color_rect(&size),
    })
  }
}

impl Mask for LevelSelectMenu {
  fn point(&self)         -> &Point     { self.rect.point()     }
  fn point_mut(&mut self) -> &mut Point { self.rect.point_mut() }
  fn size(&self)          -> &Size      { self.rect.size()      }
  fn origin(&self)        -> &Origin    { self.rect.origin()    }
}

impl Menu for LevelSelectMenu {
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.rect.draw(ctx)?;
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
