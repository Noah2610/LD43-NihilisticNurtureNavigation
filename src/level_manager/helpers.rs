use ggez::Context;
use noframe::geo::prelude::*;

use settings::res::{ self, MISSING_IMAGE };
use animation::prelude::*;
use menu::buttons::prelude::*;

pub fn new_background(ctx: &mut Context, n: usize) -> Option<Animation> {
  match n {
    _ => Some(Animation::new(
        ctx,
        vec![::join_str(res::BACKGROUND_IMAGES, "bg4.png")],
        vec![1000]
    ))
  }
}

pub fn new_pause_button(ctx: &mut Context, _window_size: &Size) -> Button {
  ButtonBuilder::new(ctx)
    .point(Point::new(4.0, 4.0))
    .size(Size::new(32.0, 32.0))
    .origin(Origin::TopLeft)
    .button_type(ButtonType::IngamePause)
    .animation_from(vec![MISSING_IMAGE.to_string()], vec![1000])
    .build().expect("Should build IngamePause Button")
}
