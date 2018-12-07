pub mod title;
pub mod toolbox;
pub mod ingame;
pub mod buttons;

use ggez::{
  GameResult,
  Context,
  event::MouseButton
};
use noframe::geo::prelude::*;

use self::title::TitleMenu;
use self::buttons::Button;
use animation::Animation;
use animation::Facing;

#[derive(Clone, Debug)]
pub enum ButtonType {
  Start,
  NextLevel,
  LarryLeft,
  LarryRight,
  ThingLeft,
  ThingRight,
  BloatLeft,
  BloatRight
}

pub struct MenuManager {
  pub title: TitleMenu
}

impl MenuManager {
  pub fn new(ctx: &mut Context, size: Size) -> Self {
    Self {
      title: title::TitleMenu::new(ctx, Point::new(0.0, 0.0), size)
    }
  }
}

pub trait Menu: Mask {
  fn buttons(&self) -> &Vec<Button>;
  fn buttons_mut(&mut self) -> &mut Vec<Button>;
  fn animation(&self) -> &Animation;
  fn animation_mut(&mut self) -> &mut Animation;
  fn clicked(&mut self, btn_type: ButtonType);
  fn get_clicked(&self) -> &Option<ButtonType>;
  fn clear_clicked(&mut self);
  fn has_animation(&self) -> bool;

  fn mouse_down(&mut self, x: i32, y: i32) {
    let point = Point::new(x as f32, y as f32);
    for i in 0 .. self.buttons().len() {
      let btn_type = self.buttons()[i].button_type.clone();
      if self.buttons()[i].intersects_point(&point) {
        self.clicked(btn_type);
      }
    }
  }

  fn update(&mut self) -> GameResult<()> {
    for button in self.buttons_mut() {
      button.update()?;
    }
    self.clear_clicked();
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let point = self.point().clone();
    let size = self.size().clone();
    if self.has_animation() {
      self.animation_mut().draw(ctx, &point, &size, &Facing::Right)?;
    }
    for button in self.buttons_mut() {
      button.draw(ctx)?;
    }
    Ok(())
  }
}
