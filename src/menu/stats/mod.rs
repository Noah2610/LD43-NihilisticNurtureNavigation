pub mod prelude {
  pub use super::StatsMenu;
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
use score::prelude::*;

pub struct StatsMenu {
  point:          Point,
  size:           Size,
  origin:         Origin,
  animation:      Animation,
  buttons:        Vec<Button>,
  clicked:        Option<ButtonType>,
  texts:          StatsTexts,
  final_thankyou: Option<AnimationRect>,
}

impl StatsMenu {
  pub fn new(ctx: &mut Context, window_size: Size, score: Score, is_final: bool) -> GameResult<Self> {
    Ok(Self {
      point:          Point::new(0.0, window_size.w),
      size:           window_size.clone(),
      origin:         Origin::TopLeft,
      animation:      new_animation(ctx),
      buttons:        new_buttons(ctx, &window_size, is_final),
      clicked:        None,
      texts:          StatsTexts::new(ctx, score, &window_size)?,
      final_thankyou: if is_final { Some(new_final_thankyou(ctx, &window_size)) } else { None },
    })
  }
}

impl Mask for StatsMenu {
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

impl Menu for StatsMenu {
  fn update(&mut self) -> GameResult<()> {
    if let Some(thankyou) = &mut self.final_thankyou {
      thankyou.update()?;
    }
    self.update_menu()?;
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(thankyou) = &mut self.final_thankyou {
      thankyou.draw(ctx)?;
    }
    self.texts.draw(ctx)?;
    self.draw_menu(ctx)?;
    Ok(())
  }

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
