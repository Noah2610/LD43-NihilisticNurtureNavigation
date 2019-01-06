use ggez::Context;
use noframe::geo::prelude::*;

use settings::menus::IMAGES;
use settings::buttons;
use settings::res::{ self, MISSING_IMAGE };
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

pub fn new_buttons(ctx: &mut Context, window_size: &Size) -> Vec<Button> {
  let offset_y = 64.0;
  let padding = 128.0;
  let size = Size::new(128.0, 64.0);

  vec![
    Button::new_with_origin(
      ctx,
      window_size.center() + Point::new(0.0, offset_y),
      size.clone(),
      Origin::Center,
      ButtonType::TitleStart,
      vec![::join_str(buttons::IMAGES, "startgame_title.png")],
      vec![1000]
    ),
    Button::new_with_origin(
      ctx,
      window_size.center() + Point::new(size.w + padding, offset_y),
      size.clone(),
      Origin::Center,
      ButtonType::TitleLevelSelect,
      vec![MISSING_IMAGE.to_string()],
      vec![1000]
    ),
    Button::new_with_origin(
      ctx,
      window_size.center() + Point::new(-(size.w + padding), offset_y),
      size.clone(),
      Origin::Center,
      ButtonType::TitleQuit,
      vec![MISSING_IMAGE.to_string()],
      vec![1000]
    )
  ]
}
