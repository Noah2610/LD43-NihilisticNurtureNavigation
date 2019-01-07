use ggez::Context;
use noframe::geo::prelude::*;

use settings::res::*;
use super::super::prelude::*;
use animation::prelude::*;

pub fn new_buttons(ctx: &mut Context, window_size: &Size) -> Vec<Button> {
  let size = Size::new(128.0, 64.0);
  let padding = 16.0;

  vec![
    Button::new(
      ctx,
      window_size.center() - Point::new(0.0, size.h + padding),
      size.clone(),
      Origin::Center,
      ButtonType::PauseResume,
      vec![MISSING_IMAGE.to_string()],
      vec![1000],
      None
    ),

    Button::new(
      ctx,
      window_size.center(),
      size.clone(),
      Origin::Center,
      ButtonType::PauseReset,
      vec![MISSING_IMAGE.to_string()],
      vec![1000],
      None
    ),

    Button::new(
      ctx,
      window_size.center() + Point::new(0.0, size.h + padding),
      size.clone(),
      Origin::Center,
      ButtonType::PauseToTitle,
      vec![MISSING_IMAGE.to_string()],
      vec![1000],
      None
    )
      ]
}

pub fn new_title(ctx: &mut Context, window_size: &Size) -> AnimationRect {
  AnimationRect::new(
    window_size.center() - Point::new(0.0, 192.0),
    Size::new(256.0, 96.0),
    Origin::Center,
    Animation::new(
      ctx,
      vec![MISSING_IMAGE.to_string()],
      vec![1000]
    )
  )
}
