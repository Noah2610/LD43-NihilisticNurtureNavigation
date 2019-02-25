use ggez::{
  Context,
  graphics::{ Text, Font },
};
use noframe::geo::prelude::*;

use settings::res::*;
use settings::menus;
use settings::buttons;
use settings::menus::pause::*;
use menu::buttons::prelude::*;
use animation::prelude::*;
use color_rect::prelude::*;
use image_text::prelude::*;

pub fn new_color_rect(window_size: &Size) -> ColorRect {
  ColorRectBuilder::new()
    .size(window_size.clone())
    .color([0.0, 0.0, 0.0, 0.9])
    .build()
}

pub fn new_buttons(ctx: &mut Context, window_size: &Size) -> Vec<Button> {
  let size = Size::new(64.0, 64.0);
  let padding = 32.0;

  let mut btns = Vec::new();

  btns.push(ButtonBuilder::new(ctx)
            .point(window_size.center() + Point::new(size.w + padding, 0.0))
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
            .point(window_size.center() - Point::new(size.w + padding, 0.0))
            .size(size.clone())
            .origin(Origin::Center)
            .button_type(ButtonType::PauseToTitle)
            .animation_from(vec![::join_str(buttons::IMAGES, "return.png")], vec![1000])
            .build().expect("Should build PauseToTitle Button"));

  btns
}

pub fn new_title(ctx: &mut Context, window_size: &Size) -> ImageText {
  let font = Font::new(ctx, fonts::DEFAULT, TITLE_FONT_SIZE).expect("Create font");
  ImageTextBuilder::new()
    .point(window_size.center() - Point::new(0.0, 192.0))
    .size_from(256.0, 96.0)
    .origin(Origin::Center)
    .bg_color([0.5, 0.5, 0.5, 1.0])
    .text(Text::new(ctx, "Paused", &font).expect("Create text"))
    .text_color(TITLE_FONT_COLOR)
    .text_origin(TextOrigin::Center)
    .build()
}
