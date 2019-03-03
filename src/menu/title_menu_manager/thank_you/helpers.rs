use ggez::{
  Context,
  GameResult,
};

use settings::menus::*;
use animation::prelude::*;

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
