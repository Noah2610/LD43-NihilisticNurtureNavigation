use ggez::Context;
use noframe::geo::prelude::*;

use settings::buttons;
use super::super::buttons::Button;
use super::super::ButtonType;
use super::Closeup;
use animation::Animation;

pub fn new_next_level_button(ctx: &mut Context, point: &Point, size: &Size) -> Button {
  Button::new(
    ctx,
    Point::new(size.w - 160.0, point.y),
    Size::new(128.0, 64.0),
    Origin::TopLeft,
    ButtonType::NextLevel,
    vec![::join_str(buttons::IMAGES, "next_level.png")],
    vec![1000],
    None
  )
}

pub fn new_buttons(ctx: &mut Context, point: &Point) -> Vec<Button> {
  vec![
    Button::new(
      ctx,
      Point::new(32.0, point.y),
      Size::new(64.0, 64.0),
      Origin::TopLeft,
      ButtonType::LarryLeft,
      vec![::join_str(buttons::IMAGES, "child_left.png")],
      vec![1000],
      None
    ),
    Button::new(
      ctx,
      Point::new(160.0, point.y),
      Size::new(64.0, 64.0),
      Origin::TopLeft,
      ButtonType::LarryRight,
      vec![::join_str(buttons::IMAGES, "child_right.png")],
      vec![1000],
      None
    ),
    Button::new(
      ctx,
      Point::new(288.0, point.y),
      Size::new(64.0, 64.0),
      Origin::TopLeft,
      ButtonType::ThingLeft,
      vec![::join_str(buttons::IMAGES, "child_left.png")],
      vec![1000],
      None
    ),
    Button::new(
      ctx,
      Point::new(416.0, point.y),
      Size::new(64.0, 64.0),
      Origin::TopLeft,
      ButtonType::ThingRight,
      vec![::join_str(buttons::IMAGES, "child_right.png")],
      vec![1000],
      None
    ),
    Button::new(
      ctx,
      Point::new(544.0, point.y),
      Size::new(64.0, 64.0),
      Origin::TopLeft,
      ButtonType::BloatLeft,
      vec![::join_str(buttons::IMAGES, "child_left.png")],
      vec![1000],
      None
    ),
    Button::new(
      ctx,
      Point::new(672.0, point.y),
      Size::new(64.0, 64.0),
      Origin::TopLeft,
      ButtonType::BloatRight,
      vec![::join_str(buttons::IMAGES, "child_right.png")],
      vec![1000],
      None
    )
      ]
}

pub fn new_closeups(ctx: &mut Context, point: &Point) -> Vec<Closeup> {
  vec![
    Closeup::new(
      Point::new(96.0, point.y),
      Animation::new(
        ctx,
        vec![::join_str(::settings::child::IMAGES, "larry_closeup_v2.png")],
        vec![1000]
      )
    ),
    Closeup::new(
      Point::new(352.0, point.y),
      Animation::new(
        ctx,
        vec![::join_str(::settings::child::IMAGES, "child_2_closeup.png")],
        vec![1000]
      )
    ),
    Closeup::new(
      Point::new(608.0, point.y),
      Animation::new(
        ctx,
        vec![::join_str(::settings::child::IMAGES, "child_3_closeup.png")],
        vec![1000]
      )
    )
      ]
}
