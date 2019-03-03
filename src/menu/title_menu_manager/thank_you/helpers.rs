use ggez::{
  Context,
  GameResult,
};
use noframe::geo::prelude::*;

use settings::menus::*;
use settings::buttons;
use animation::prelude::*;
use menu::buttons::prelude::*;

pub fn new_animation(ctx: &mut Context) -> GameResult<Animation> {
  let mut images = Vec::new();
  let mut intervals = Vec::new();
  for i in 0 ..= 18 {
    images.push(format!("{}thank_you_{:02}.png", IMAGES, i));
    intervals.push(100);
  }
  intervals.pop();
  intervals.push(2000);
  Ok(Animation::new(
      ctx,
      images,
      intervals
  ))
}

pub fn new_buttons(ctx: &mut Context, window_size: &Size) -> GameResult<Vec<Button>> {
  let padding = Point::new(32.0, 32.0);
  let mut btns = Vec::new();

  btns.push(ButtonBuilder::new(ctx)
            .point_from(padding.x, window_size.h - padding.y)
            .size_from(64.0, 64.0)
            .origin(Origin::BottomLeft)
            .button_type(ButtonType::ThankYouBack)
            .animation_from(vec![::join_str(buttons::IMAGES, "arrow.png")], vec![1000])
            .build()?);

  Ok(btns)
}
