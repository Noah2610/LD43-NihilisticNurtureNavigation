use ggez::Context;
use noframe::geo::prelude::*;

use settings::buttons;
use settings::child;
use menu::buttons::prelude::*;
use super::Closeup;
use animation::prelude::*;
use persons::children::ChildType;

pub fn new_next_level_button(ctx: &mut Context, point: &Point, size: &Size) -> Button {
  ButtonBuilder::new(ctx)
    .point_from(size.w - 160.0, point.y)
    .size_from(128.0, 64.0)
    .origin(Origin::TopLeft)
    .button_type(ButtonType::NextLevel)
    .animation_from(vec![::join_str(buttons::IMAGES, "next_level.png")], vec![1000])
    .build().expect("Should build NextLevel Button")
}

pub fn new_buttons(ctx: &mut Context, point: &Point, children: &Vec<ChildType>) -> Vec<Button> {
  use self::ChildType::*;
  use self::ButtonType::*;

  let spacing = 128.0;
  let offset  = 32.0;

  let mut button_types = Vec::new();
  for child_type in children {
    match child_type {
      Larry => {
        button_types.push(LarryLeft);
        button_types.push(LarryRight);
      }
      Thing => {
        button_types.push(ThingLeft);
        button_types.push(ThingRight);
      }
      Bloat => {
        button_types.push(BloatLeft);
        button_types.push(BloatRight);
      }
    }
  }

  button_types.iter().enumerate().map( |(i, button_type)| {
    let x = offset + spacing * i as NumType;
    if i % 2 == 0 {
      ButtonBuilder::new(ctx)
        .point_from(x, point.y)
        .size_from(64.0, 64.0)
        .origin(Origin::TopLeft)
        .button_type(button_type.clone())
        .animation_from(vec![::join_str(buttons::IMAGES, "child_right2.png")], vec![1000])
        .facing(Facing::Left)
        .build()
    } else {
      ButtonBuilder::new(ctx)
        .point_from(x, point.y)
        .size_from(64.0, 64.0)
        .origin(Origin::TopLeft)
        .button_type(button_type.clone())
        .animation_from(vec![::join_str(buttons::IMAGES, "child_right2.png")], vec![1000])
        .build()
    }.expect(&format!("Should build {} Button", button_type))
  }).collect()
}

pub fn new_closeups(ctx: &mut Context, point: &Point, children: &Vec<ChildType>) -> Vec<Closeup> {
  use self::ChildType::*;

  let offset  = 96.0;
  let spacing = 256.0;

  children.iter().enumerate().map( |(i, child_type)| {
    let x = offset + spacing * i as NumType;
    let (images, intervals) = match child_type {
      Larry => (
        vec![::join_str(child::IMAGES, "larry_closeup_v2.png")],
        vec![1000]
      ),
      Thing => (
        vec![::join_str(child::IMAGES, "child_2_closeup.png")],
        vec![1000]
      ),
      Bloat => (
        vec![::join_str(child::IMAGES, "child_3_closeup.png")],
        vec![1000]
      ),
    };
    Closeup::new(
      Point::new(x, point.y),
      Animation::new(
        ctx,
        images,
        intervals
      )
    )
  }).collect()
}
