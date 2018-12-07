use ggez::{
  GameResult,
  Context,
  graphics
};
use noframe::geo::prelude::*;

use settings::menus::IMAGES;
use settings::buttons;
use settings::fonts::*;
use super::Menu;
use super::buttons::Button;
use super::ButtonType;
use animation::Animation;
use animation::Facing;

pub struct Closeup {
  pub point:     Point,
  pub size:      Size,
  pub animation: Animation
}

impl Closeup {
  pub fn draw(&mut self, ctx: &mut Context) {
    self.animation.draw(ctx, &self.point, &self.size, &Facing::Right);
  }
}

pub struct ToolboxMenu {
  point:                   Point,
  size:                    Size,
  origin:                  Origin,
  buttons:                 Vec<Button>,
  animation:               Animation,
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
      font:                    graphics::Font::new(ctx, ::join_str(FONTS, "vcr_osd_mono.ttf"), TO_SAVE_FONT_SIZE).expect("Should load font"),
      to_save_text:            None,
      next_level_button:       Button::new(
        ctx,
        Point::new(size.w - 160.0, point.y),
        Size::new(128.0, 64.0),
        ButtonType::NextLevel,
        vec![::join_str(buttons::IMAGES, "next_level.png")],
        vec![1000]
      ),
      buttons: vec![
        Button::new(
          ctx,
          Point::new(32.0, point.y),
          Size::new(64.0, 64.0),
          ButtonType::LarryLeft,
          vec![::join_str(buttons::IMAGES, "child_left.png")],
          vec![1000]
        ),
        Button::new(
          ctx,
          Point::new(160.0, point.y),
          Size::new(64.0, 64.0),
          ButtonType::LarryRight,
          vec![::join_str(buttons::IMAGES, "child_right.png")],
          vec![1000]
        ),
        Button::new(
          ctx,
          Point::new(288.0, point.y),
          Size::new(64.0, 64.0),
          ButtonType::ThingLeft,
          vec![::join_str(buttons::IMAGES, "child_left.png")],
          vec![1000]
        ),
        Button::new(
          ctx,
          Point::new(416.0, point.y),
          Size::new(64.0, 64.0),
          ButtonType::ThingRight,
          vec![::join_str(buttons::IMAGES, "child_right.png")],
          vec![1000]
        ),
        Button::new(
          ctx,
          Point::new(544.0, point.y),
          Size::new(64.0, 64.0),
          ButtonType::BloatLeft,
          vec![::join_str(buttons::IMAGES, "child_left.png")],
          vec![1000]
        ),
        Button::new(
          ctx,
          Point::new(672.0, point.y),
          Size::new(64.0, 64.0),
          ButtonType::BloatRight,
          vec![::join_str(buttons::IMAGES, "child_right.png")],
          vec![1000]
        )
          ],
          animation: Animation::new(
            ctx,
            vec![],
            vec![1000]
          ),
          clicked: None,
          closeups: vec![
            Closeup {
              point: Point::new(96.0, point.y),
              size:  Size::new(64.0, 64.0),
              animation: Animation::new(
                ctx,
                vec![::join_str(::settings::child::IMAGES, "larry_closeup_v2.png")],
                vec![1000]
              )
            },
            Closeup {
              point: Point::new(352.0, point.y),
              size:  Size::new(64.0, 64.0),
              animation: Animation::new(
                ctx,
                vec![::join_str(::settings::child::IMAGES, "child_2_closeup.png")],
                vec![1000]
              )
            },
            Closeup {
              point: Point::new(608.0, point.y),
              size:  Size::new(64.0, 64.0),
              animation: Animation::new(
                ctx,
                vec![::join_str(::settings::child::IMAGES, "child_3_closeup.png")],
                vec![1000]
              )
            }
          ]
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
}

impl Mask for ToolboxMenu {
  fn point(&self)         -> &Point     { &self.point     }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size      }
  fn origin(&self)        -> &Origin    { &self.origin    }
}

impl Menu for ToolboxMenu {
  fn has_animation(&self) -> bool { false }
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
    let point = self.point().clone();
    let size = self.size().clone();
    if self.has_animation() {
      self.animation_mut().draw(ctx, &point, &size, &Facing::Right)?;
    }
    for button in self.buttons_mut() {
      button.draw(ctx)?;
    }
    if self.to_save_amount > 0 {
      self.next_level_button.draw(ctx)?;
      if let Some(text) = self.get_to_save_text(ctx)?.clone() {
        let point = graphics::Point2::from(
          &Point::combine(vec![self.next_level_button.point(), &Point::new(0.0, -32.0)])
        );
        graphics::set_color(ctx, noframe::color::WHITE.into())?;
        graphics::draw(ctx, &text, point, 0.0)?;
      }
    }
    Ok(())
  }
}
