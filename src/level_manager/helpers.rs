use ggez::Context;
use noframe::geo::prelude::*;

use settings::res::{ MISSING_IMAGE, BACKGROUND_IMAGES };
use animation::prelude::*;
use menu::buttons::prelude::*;

pub fn new_background(ctx: &mut Context, n: usize) -> Option<Animation> {
  let chapter = n / 5;
  let sub = n % 5;
  match chapter {
    0 => match sub {
      0 => Some("bg1.1.png"),
      1 => Some("bg1.2.png"),
      2 => Some("bg1.3.png"),
      3 => Some("bg1.4.png"),
      4 => Some("bg1.5.png"),
      _ => None,
    },
    1 => match sub {
      0 => Some("bg2.1.png"),
      1 => Some("bg2.2.png"),
      2 => Some("bg2.3.png"),
      3 => Some("bg2.4.png"),
      4 => Some("bg2.5.png"),
      _ => None,
    },
    2 => match sub {
      0 => Some("bg3.1.png"),
      1 => Some("bg3.2.png"),
      2 => Some("bg3.3.png"),
      3 => Some("bg3.4.png"),
      4 => Some("bg3.5.png"),
      _ => None,
    },
    _ => None,
  } .and_then( |img| Some(Animation::new(
        ctx,
        vec![::join_str(BACKGROUND_IMAGES, img)],
        vec![1000]
  )))
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
