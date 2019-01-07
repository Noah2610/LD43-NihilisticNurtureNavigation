pub mod prelude {
  pub use super::Menu;
  pub use super::buttons::Button;
  pub use super::buttons::ButtonType;
  pub use animation::prelude::*;
}

pub mod title_menu_manager;
pub mod toolbox;
pub mod pause;
pub mod stats;

pub mod buttons;

use ggez::{
  GameResult,
  Context,
};
use noframe::geo::prelude::*;

use self::buttons::Button;
use self::buttons::ButtonType;
use animation::Animation;
use animation::Facing;

pub trait Menu: Mask {
  fn buttons(&self) -> &Vec<Button>;
  fn buttons_mut(&mut self) -> &mut Vec<Button>;
  fn animation(&self) -> Option<&Animation>;
  fn animation_mut(&mut self) -> Option<&mut Animation>;
  fn clicked(&mut self, btn_type: ButtonType);
  fn get_clicked(&self) -> &Option<ButtonType>;
  fn clear_clicked(&mut self);

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
    self.update_menu()
  }

  fn update_menu(&mut self) -> GameResult<()> {
    for button in self.buttons_mut() {
      button.update()?;
    }
    self.clear_clicked();
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.draw_menu(ctx)
  }

  fn draw_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    let point = self.point().clone();
    let size = self.size().clone();
    if let Some(anim) = self.animation_mut() {
      anim.draw(ctx, &point, &size, &Facing::Right)?;
    }
    for button in self.buttons_mut() {
      button.draw(ctx)?;
    }
    Ok(())
  }
}
