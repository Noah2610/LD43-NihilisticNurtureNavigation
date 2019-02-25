use ggez::Context;
use noframe::geo::prelude::*;

use settings::menus::IMAGES;
use settings::buttons;
use menu::prelude::*;
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
  let padding = 64.0;
  let size = Size::new(64.0, 64.0);

  let mut btns = Vec::new();

  btns.push(ButtonBuilder::new(ctx)
            .point(window_size.center() + Point::new(size.w + padding, offset_y))
            .size(size.clone())
            .origin(Origin::Center)
            .button_type(ButtonType::TitleStart)
            .animation_from(vec![::join_str(buttons::IMAGES, "play.png")], vec![1000])
            .build().expect("Should build TitleStart Button"));
  btns.push(ButtonBuilder::new(ctx)
            .point(window_size.center() + Point::new(0.0, offset_y))
            .size(size.clone())
            .origin(Origin::Center)
            .button_type(ButtonType::TitleLevelSelect)
            .animation_from(vec![::join_str(buttons::IMAGES, "level_menu.png")], vec![1000])
            .build().expect("Should build TitleLevelSelect Button"));
  btns.push(ButtonBuilder::new(ctx)
            .point(window_size.center() + Point::new(-(size.w + padding), offset_y))
            .size(size.clone())
            .origin(Origin::Center)
            .button_type(ButtonType::TitleQuit)
            .animation_from(vec![::join_str(buttons::IMAGES, "exit.png")], vec![1000])
            .build().expect("Should build TitleQuit Button"));

  btns
}
