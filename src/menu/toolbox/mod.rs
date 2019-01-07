mod helpers;

use ggez::{
  GameResult,
  Context,
  graphics
};
use noframe::geo::prelude::*;

use self::helpers::*;
use settings::res;
use settings::fonts::*;
use super::Menu;
use super::buttons::Button;
use super::ButtonType;
use animation::Animation;
use animation::Facing;

pub struct Closeup {
  pub point:     Point,
  pub size:      Size,
  pub animation: Animation,
}

impl Closeup {
  pub fn new(point: Point, animation: Animation) -> Self {
    Self {
      point,
      size: Size::new(64.0, 64.0),
      animation,
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.animation.draw(ctx, &self.point, &self.size, &Facing::Right)
  }
}

pub struct ToolboxMenu {
  point:                   Point,
  size:                    Size,
  origin:                  Origin,
  buttons:                 Vec<Button>,
  clicked:                 Option<ButtonType>,
  pub closeups:            Vec<Closeup>,
  to_save_amount:          usize,
  previous_to_save_amount: usize,
  next_level_button:       Button,
  font:                    graphics::Font,
  to_save_text:            Option<graphics::Text>
}

impl ToolboxMenu {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point:                   point.clone(),
      size:                    size.clone(),
      origin:                  Origin::TopLeft,
      to_save_amount:          0,
      previous_to_save_amount: 0,
      font:                    graphics::Font::new(ctx, ::join_str(res::FONTS, "vcr_osd_mono.ttf"), TO_SAVE_FONT_SIZE).expect("Should load font"),
      to_save_text:            None,
      next_level_button:       new_next_level_button(ctx, &point, &size),
      buttons:                 new_buttons(ctx, &point),
      clicked:                 None,
      closeups:                new_closeups(ctx, &point),
    }
  }

  pub fn set_to_save_amount(&mut self, to_save_amount: usize) {
    self.to_save_amount = to_save_amount;
  }

  fn get_to_save_text(&mut self, ctx: &mut Context) -> GameResult<&Option<graphics::Text>> {
    if let None = self.to_save_text {
      self.to_save_text = Some(graphics::Text::new(ctx, &format!("Saving: {}", self.to_save_amount), &self.font)?);
    } else {
      if self.previous_to_save_amount != self.to_save_amount {
        self.to_save_text = Some(graphics::Text::new(ctx, &format!("Saving: {}", self.to_save_amount), &self.font)?);
      }
    }
    self.previous_to_save_amount = self.to_save_amount;
    Ok(&self.to_save_text)
  }

  fn draw_closeups(&mut self, ctx: &mut Context) -> GameResult<()> {
    for closeup in &mut self.closeups {
      closeup.draw(ctx)?;
    }
    Ok(())
  }

  fn draw_continue_button(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.to_save_amount > 0 {
      self.next_level_button.draw(ctx)?;
      if let Some(text) = self.get_to_save_text(ctx)?.clone() {
        let point = graphics::Point2::from(
          &Point::combine(vec![&self.next_level_button.top_right(), &Point::new(0.0, -32.0)])
        );
        let param = graphics::DrawParam {
          dest:   point,
          offset: graphics::Point2::new(1.0, 0.0),
          color:  Some(noframe::color::WHITE.into()),
          .. Default::default()
        };
        graphics::draw_ex(ctx, &text, param)?;
      }
    }
    Ok(())
  }
}

impl Mask for ToolboxMenu {
  fn point(&self)         -> &Point     { &self.point     }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size      }
  fn origin(&self)        -> &Origin    { &self.origin    }
}

impl Menu for ToolboxMenu {
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

  fn mouse_down(&mut self, x: i32, y: i32) {
    let point = Point::new(x as f32, y as f32);
    for i in 0 .. self.buttons().len() {
      let btn_type = self.buttons()[i].button_type.clone();
      if self.buttons()[i].intersects_point(&point) {
        self.clicked(btn_type);
      }
    }
    if self.to_save_amount > 0 && self.next_level_button.intersects_point(&point) {
      let btn_type = self.next_level_button.button_type.clone();
      self.clicked(btn_type);
    }
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.draw_menu(ctx)?;
    self.draw_closeups(ctx)?;
    self.draw_continue_button(ctx)?;
    Ok(())
  }
}
