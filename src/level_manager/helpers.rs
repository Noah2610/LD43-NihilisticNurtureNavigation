use ggez::Context;
use noframe::geo::prelude::*;

use settings::res::{ self, MISSING_IMAGE };
use animation::prelude::*;
use menu::buttons::prelude::*;

pub fn new_background(ctx: &mut Context, n: usize) -> Option<Animation> {
  match n {
    _ => Some(Animation::new(
        ctx,
        vec![::join_str(res::BACKGROUND_IMAGES, "default.png")],
        vec![1000]
    ))
  }
}

pub fn new_pause_button(ctx: &mut Context, _window_size: &Size) -> Button {
  Button::new(
    ctx,
    Point::new(4.0, 4.0),
    Size::new(32.0, 32.0),
    ButtonType::IngamePause,
    vec![MISSING_IMAGE.to_string()],
    vec![1000]
  )
}
