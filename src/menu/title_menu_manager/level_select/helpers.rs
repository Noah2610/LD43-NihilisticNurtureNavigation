use ggez::{
  Context,
  GameResult,
  graphics::{ Text, Font },
};
use noframe::geo::prelude::*;

use settings::res::{ MISSING_IMAGE, fonts };
use settings::buttons;
use settings::level_manager::LEVEL_NAMES;
use settings::menus;
use settings::menus::title::level_select::*;
use menu::prelude::*;

pub fn new_animation(ctx: &mut Context, _window_size: &Size) -> Animation {
  Animation::new(
    ctx,
    vec![::join_str(menus::IMAGES, "title.png")],
    vec![1000]
  )
}

pub fn new_buttons(ctx: &mut Context, window_size: &Size) -> GameResult<Vec<Button>> {
  let columns = 3.0;
  let window_rect = Rect::new(Point::new(0.0, 0.0), window_size.clone(), Origin::TopLeft);
  let padding          = 32.0;  //8.0;
  let border_padding   = 32.0;
  let size             = Size::new((window_size.w - border_padding * 2.0 - padding * (columns - 1.0)) / (columns), 72.0 /*32.0*/);
  let back_btn_size    = Size::new(128.0, 64.0);
  let initial_top_left = Point::new(border_padding, border_padding);
  let font             = Font::new_px(ctx, fonts::DEFAULT, FONT_SIZE)?;
  let text_offset      = Point::new(/*padding*/ 128.0, size.h * 0.5);
  let entries_per_col  = ((window_size.h - back_btn_size.h - border_padding * 3.0) / (size.h + padding)) as usize;

  let back_button = ButtonBuilder::new(ctx)
    .point(window_rect.bottom_left() + Point::new(border_padding, -border_padding))
    .size(back_btn_size)
    .origin(Origin::BottomLeft)
    .button_type(ButtonType::LevelSelectBack)
    .animation_from(vec![::join_str(buttons::IMAGES, "return.png")], vec![1000])
    .build().expect("Should build LevelSelectBack Button");

  Ok(LEVEL_NAMES.iter().enumerate()
     .map( |(i, name)| {
       let col = i / entries_per_col;
       let top_left = initial_top_left.clone() +
         Point::new((size.w + padding) * col as NumType,
         (size.h + padding) * (i - col * entries_per_col) as NumType);
       let text = Text::new(ctx, &format!("{}) {}", i + 1, &::semantic(&name.replace(".json", ""))), &font)?;
       Ok(ButtonBuilder::new(ctx)
          .point(top_left.clone())
          .size(size.clone())
          .origin(Origin::TopLeft)
          .button_type(ButtonType::LevelSelectLevel(i))
          .animation_from(vec![::join_str(buttons::IMAGES, "level_select1.png")], vec![1000])
          .text_from(
            top_left + text_offset.clone(),
            Size::new(text.width() as NumType, text.height() as NumType),
            Origin::CenterLeft,
            text
          ).build().expect("Should build LevelSelectLevel Button")
       )
     })
  .chain(vec![Ok(back_button)]).collect::<GameResult<Vec<Button>>>()?)
}
