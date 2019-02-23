pub mod prelude {
  pub use super::Menu;
  pub use super::buttons::prelude::*;
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
  fn buttons(&self) -> Vec<&Button>;
  fn buttons_mut(&mut self) -> Vec<&mut Button>;
  fn animation(&self) -> Option<&Animation>;
  fn animation_mut(&mut self) -> Option<&mut Animation>;
  fn clicked(&mut self, btn_type: ButtonType);
  fn get_clicked(&self) -> &Option<ButtonType>;
  fn clear_clicked(&mut self);

  fn mouse_down(&mut self, x: i32, y: i32) {
    for btn_type in {
      self.buttons_intersecting_point(Point::new(x as NumType, y as NumType)).iter()
        .map( |btn| btn.button_type.clone() ).collect::<Vec<ButtonType>>()
    } {
      self.clicked(btn_type);
    }
  }

  // TODO UNIMPLEMENTED!!!
  fn mouse_move(&mut self, x: i32, y: i32, xrel: i32, yrel: i32) {
    let point = Point::new(x as NumType, y as NumType);
    for btn in self.buttons_mut() {
      if btn.intersects_point(&point) {
        btn.mouse_in();
      } else {
        btn.mouse_out();
      }
    }
  }

  fn buttons_intersecting_point(&self, point: Point) -> Vec<&Button> {
    let mut btns = Vec::new();
    for btn in self.buttons() {
      if btn.intersects_point(&point) {
        btns.push(btn);
      }
    }
    btns
  }

  fn buttons_intersecting_point_mut(&mut self, point: Point) -> Vec<&mut Button> {
    let mut btns = Vec::new();
    for btn in self.buttons_mut() {
      if btn.intersects_point(&point) {
        btns.push(btn);
      }
    }
    btns
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
