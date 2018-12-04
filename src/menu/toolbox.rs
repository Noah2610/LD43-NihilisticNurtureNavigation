use ggez::{
  GameResult,
  Context
};
use noframe::geo::prelude::*;

use settings::menus::IMAGES;
use settings::buttons;
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
  point:        Point,
  size:         Size,
  origin:       Origin,
  buttons:      Vec<Button>,
  animation:    Animation,
  clicked:      Option<ButtonType>,
  pub closeups: Vec<Closeup>
}

impl ToolboxMenu {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point: point.clone(),
      size: size.clone(),
      origin:    Origin::TopLeft,
      buttons:   vec![
        Button::new(
          ctx,
          Point::new(64.0, point.y - 64.0),
          Size::new(64.0, 64.0),
          ButtonType::LarryLeft,
          vec![::join_str(buttons::IMAGES, "child_left.png")],
          vec![1000]
        ),
        Button::new(
          ctx,
          Point::new(192.0, point.y - 64.0),
          Size::new(64.0, 64.0),
          ButtonType::LarryRight,
          vec![::join_str(buttons::IMAGES, "child_right.png")],
          vec![1000]
        ),
        Button::new(
          ctx,
          Point::new(320.0, point.y - 64.0),
          Size::new(64.0, 64.0),
          ButtonType::ThingLeft,
          vec![::join_str(buttons::IMAGES, "child_left.png")],
          vec![1000]
        ),
        Button::new(
          ctx,
          Point::new(448.0, point.y - 64.0),
          Size::new(64.0, 64.0),
          ButtonType::ThingRight,
          vec![::join_str(buttons::IMAGES, "child_right.png")],
          vec![1000]
        ),
        Button::new(
          ctx,
          Point::new(576.0, point.y - 64.0),
          Size::new(64.0, 64.0),
          ButtonType::BloatLeft,
          vec![::join_str(buttons::IMAGES, "child_left.png")],
          vec![1000]
        ),
        Button::new(
          ctx,
          Point::new(704.0, point.y - 64.0),
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
          point: Point::new(128.0, point.y - 64.0),
          size:  Size::new(64.0, 64.0),
          animation: Animation::new(
            ctx,
            vec![::join_str(::settings::child::IMAGES, "larry_closeup_v2.png")],
            vec![1000]
          )
        },
        Closeup {
          point: Point::new(384.0, point.y - 64.0),
          size:  Size::new(64.0, 64.0),
          animation: Animation::new(
            ctx,
            vec![::join_str(::settings::child::IMAGES, "child_2_closeup.png")],
            vec![1000]
          )
        },
        Closeup {
          point: Point::new(640.0, point.y - 64.0),
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
}
