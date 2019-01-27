use ggez::Context;
use noframe::geo::prelude::*;

use settings::res::*;
use settings::menus;
use settings::buttons;
use menu::buttons::prelude::*;
use animation::prelude::*;

pub fn new_animation(ctx: &mut Context, window_size: &Size) -> Animation {
  Animation::new(ctx,
                 vec![::join_str(menus::IMAGES, "pause.png")],
                 vec![1000])
}

pub fn new_buttons(ctx: &mut Context, window_size: &Size) -> Vec<Button> {
  let size = Size::new(64.0, 64.0);
  let padding = 16.0;

  let mut btns = Vec::new();

  btns.push(ButtonBuilder::new(ctx)
            .point(window_size.center() - Point::new(0.0, size.h + padding))
            .size(size.clone())
            .origin(Origin::Center)
            .button_type(ButtonType::PauseResume)
            .animation_from(vec![::join_str(buttons::IMAGES, "play.png")], vec![1000])
            .build().expect("Should build PauseResume Button"));
  btns.push(ButtonBuilder::new(ctx)
            .point(window_size.center())
            .size(size.clone())
            .origin(Origin::Center)
            .button_type(ButtonType::PauseReset)
            .animation_from(vec![::join_str(buttons::IMAGES, "retry.png")], vec![1000])
            .build().expect("Should build PauseReset Button"));
  btns.push(ButtonBuilder::new(ctx)
            .point(window_size.center() + Point::new(0.0, size.h + padding))
            .size(size.clone())
            .origin(Origin::Center)
            .button_type(ButtonType::PauseToTitle)
            .animation_from(vec![::join_str(buttons::IMAGES, "return.png")], vec![1000])
            .build().expect("Should build PauseToTitle Button"));

  btns
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
