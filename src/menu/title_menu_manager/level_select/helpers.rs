use ggez::{
  Context,
  GameResult,
  graphics::{ Text, Font },
};
use noframe::geo::prelude::*;
use noframe::geo::mask::misc::Side;

use settings::res::fonts;
use settings::buttons;
use settings::level_manager::LEVEL_NAMES;
use settings::menus::title::level_select::*;
use menu::prelude::*;
use color_rect::prelude::*;

pub fn new_color_rect(window_size: &Size) -> ColorRect {
  ColorRectBuilder::new()
    .size(window_size.clone())
    .color([0.0, 0.0, 0.0, 1.0])
    .build()
}

pub fn new_buttons(ctx: &mut Context, window_size: &Size) -> GameResult<Vec<Button>> {
  let columns = 3;
  let rows    = 5;
  let padding          = Point::new(48.0, 32.0);
  let border_padding   = Point::new(64.0, 32.0);
  let buttons_area     = Rect::new(
    Point::new(border_padding.x, border_padding.y),
    Size::new(
      window_size.w - border_padding.x * 2.0,
      window_size.h - border_padding.y * 2.0
    ),
    Origin::TopLeft
  );
  let size             = Size::new(
    (buttons_area.size().w - padding.x * (columns - 1) as NumType) / columns as NumType,
    80.0
  );
  let column_point = Point::new(
    buttons_area.side(Side::Left) + size.w / 2.0,
    buttons_area.side(Side::Top)
  );
  let text_offset  = Point::new(-80.0, 0.0);
  let font         = Font::new_px(ctx, fonts::DEFAULT, FONT_SIZE)?;
  let mut buttons  = Vec::new();

  for col in 0 .. columns {
    for row in 0 .. rows {
      let i    = rows * col + row;
      let name = LEVEL_NAMES[i];
      let image_filename = &format!("level_select{}.png", col + 1);
      let center = Point::new(
        column_point.x + (size.w + padding.x) * col as NumType,
        column_point.y + size.h / 2.0 + (size.h + padding.y) * row as NumType
      );
      let origin = Origin::Center;
      let text   = Text::new(ctx, &format!("{}) {}", i + 1, &::semantic(&name.replace(".json", ""))), &font)?;
      buttons.push(ButtonBuilder::new(ctx)
                   .point(center.clone())
                   .size(size.clone())
                   .origin(origin)
                   .button_type(ButtonType::LevelSelectLevel(i))
                   .animation_from(vec![::join_str(buttons::IMAGES, image_filename)], vec![1000])
                   .text_from(
                     center + text_offset.clone(),
                     Size::new(text.width() as NumType, text.height() as NumType),
                     Origin::CenterLeft,
                     text
                   ).build()?);
    }
  }

  buttons.push(ButtonBuilder::new(ctx)
               .point(buttons_area.bottom_left())
               .size(Size::new(64.0, 64.0))
               .origin(Origin::BottomLeft)
               .button_type(ButtonType::LevelSelectBack)
               .animation_from(vec![::join_str(buttons::IMAGES, "arrow.png")], vec![1000])
               .build()?);

  Ok(buttons)
}
