use ggez::Context;
use noframe::geo::prelude::*;

use settings::menus::IMAGES;
use settings::buttons;
use super::super::buttons::Button;
use super::super::ButtonType;
use animation::Animation;

pub fn new_animation(ctx: &mut Context) -> Animation {
  Animation::new(
    ctx,
    vec![::join_str(IMAGES, "title.png")],
    vec![1000]
  )
}

pub fn new_buttons(ctx: &mut Context) -> Vec<Button> {
  vec![
    Button::new(
      ctx,
      Point::new(128.0, 128.0),
      Size::new(128.0, 64.0),
      ButtonType::Start,
      vec![::join_str(buttons::IMAGES, "startgame_title.png")],
      vec![1000]
    )
  ]
}
