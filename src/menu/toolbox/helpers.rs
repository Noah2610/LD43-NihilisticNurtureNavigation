use ggez::Context;
use noframe::geo::prelude::*;

use settings::buttons;
use menu::buttons::prelude::*;
use super::Closeup;
use animation::Animation;

pub fn new_next_level_button(ctx: &mut Context, point: &Point, size: &Size) -> Button {
  ButtonBuilder::new(ctx)
    .point_from(size.w - 160.0, point.y)
    .size_from(128.0, 64.0)
    .origin(Origin::TopLeft)
    .button_type(ButtonType::NextLevel)
    .animation_from(vec![::join_str(buttons::IMAGES, "next_level.png")], vec![1000])
    .build().expect("Should build NextLevel Button")
}

pub fn new_buttons(ctx: &mut Context, point: &Point) -> Vec<Button> {
  let offset_x = 128.0;

  [
    ButtonType::LarryLeft, ButtonType::LarryRight,
    ButtonType::ThingLeft, ButtonType::ThingRight,
    ButtonType::BloatLeft, ButtonType::BloatRight,
  ].iter().enumerate().map( |(i, button_type)| {
    let x = 32.0 + offset_x * i as NumType;
    if i % 2 == 0 {
      ButtonBuilder::new(ctx)
        .point_from(x, point.y)
        .size_from(64.0, 64.0)
        .origin(Origin::TopLeft)
        .button_type(button_type.clone())
        .animation_from(vec![::join_str(buttons::IMAGES, "child_left.png")], vec![1000])
        .build()
    } else {
      ButtonBuilder::new(ctx)
        .point_from(x, point.y)
        .size_from(64.0, 64.0)
        .origin(Origin::TopLeft)
        .button_type(button_type.clone())
        .animation_from(vec![::join_str(buttons::IMAGES, "child_right.png")], vec![1000])
        .build()
    }.expect(&format!("Should build {} Button", button_type))
  }).collect()
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
