use ggez::{
  Context,
  GameResult,
  graphics,
};
use noframe::geo::prelude::*;
use noframe::color::Color;

use settings::res::*;
use settings::menus::stats::*;
use settings::buttons;
use settings::score::{ SCORE_COLOR, HIGHSCORE_COLOR, NEW_HIGHSCORE_COLOR };
use animation::prelude::*;
use menu::buttons::prelude::*;
use score::prelude::*;
use color_rect::prelude::*;

pub enum TextOrigin {
  Left,
  Right,
  Center,
}

impl TextOrigin {
  pub fn val(&self) -> f32 {
    match self {
      TextOrigin::Left   => 0.0,
      TextOrigin::Right  => 1.0,
      TextOrigin::Center => 0.5,
    }
  }
}

pub struct StatsText {
  text:   graphics::Text,
  point:  Point,
  origin: TextOrigin,
  color:  Option<Color>,
}

impl StatsText {
  pub fn new(text: graphics::Text, point: Point, origin: TextOrigin, color: Option<Color>) -> Self {
    Self {
      text,
      point,
      origin,
      color,
    }
  }

  pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    let dest = graphics::Point2::from(&self.point);
    let param = graphics::DrawParam {
      dest,
      offset: graphics::Point2::new(self.origin.val(), 0.0),
      color:  Some(self.color.unwrap_or(FONT_COLOR).into()),
      .. Default::default()
    };
    graphics::draw_ex(ctx, &self.text, param)?;
    Ok(())
  }
}

pub struct StatsTexts {
  score:          StatsText,
  highscore:      Option<StatsText>,
  saved_player:   Option<StatsText>,
  saved_children: Vec<StatsText>,
}

impl StatsTexts {
  pub fn new(
    ctx: &mut Context,
    score: Score,
    highscore_opt: Option<Score>,
    point: &Point,
    size: &Size,
    is_final: bool
  ) -> GameResult<Self> {
    let font_score = graphics::Font::new_px(ctx, fonts::DEFAULT, FONT_SIZE_SCORE)?;
    let font_saved = graphics::Font::new_px(ctx, fonts::DEFAULT, FONT_SIZE_SAVED)?;
    let offset = Point::new(32.0, 32.0);
    let score_offset = Point::new(0.0, 8.0);
    let saved_offset = Point::new(0.0, 8.0);
    let point_score = point.clone() + offset.clone();
    let point_saved = Point::new(
      point.x + size.w - offset.x,
      point_score.y + offset.y + font_score.get_height() as NumType
    );

    let semantic_score = if is_final {
      format!("Total Best Score: {}", score)
    } else {
      score.semantic_score()
    };
    let score_text = StatsText::new(
      graphics::Text::new(ctx, &semantic_score, &font_score)?,
      point_score.clone(),
      TextOrigin::Left,
      Some(SCORE_COLOR)
    );

    let highscore_text = if let Some(highscore) = highscore_opt {
      let text;
      let color;
      if highscore >= score {
        text = highscore.semantic_highscore();
        color = HIGHSCORE_COLOR;
      } else {
        text = "New Highscore!".to_string();
        color = NEW_HIGHSCORE_COLOR;
      }
      Some(StatsText::new(
          graphics::Text::new(ctx, &text, &font_score)?,
          Point::new(
            point_score.x + score_offset.x,
            point_score.y + score_offset.y + font_score.get_height() as NumType
          ),
          TextOrigin::Left,
          Some(color)
      ))
    } else { None };

    let saved_player = if let Some(score) = &score.semantic_player() {
      Some(StatsText::new(
        graphics::Text::new(ctx, score, &font_saved)?,
        point_saved.clone() + Point::new(
          0.0,
          if highscore_text.is_some() { font_score.get_height() as NumType } else { 0.0 }
        ),
        TextOrigin::Right,
        None
      ))
    } else {
      None
    };

    let mut saved_children = Vec::new();
    for (i, s) in score.semantic_children().iter().enumerate() {
      let i_plus = if saved_player.is_some() {
        1
      } else { 0 };
      saved_children.push(
        StatsText::new(
          graphics::Text::new(ctx, &s, &font_saved)?,
          point_saved.clone() + Point::new(
            saved_offset.x,
            if highscore_text.is_some() {
              font_score.get_height() as NumType
            } else { 0.0 } + (font_saved.get_height() as NumType + saved_offset.y) * (i + i_plus) as NumType
          ),
          TextOrigin::Right,
          None
        )
      );
    }

    Ok(StatsTexts {
      score:     score_text,
      highscore: highscore_text,
      saved_player,
      saved_children,
    })
  }

  pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    self.score.draw(ctx)?;
    if let Some(highscore) = &self.highscore {
      highscore.draw(ctx)?;
    }
    if let Some(saved_player) = &self.saved_player {
      saved_player.draw(ctx)?;
    }
    for child in &self.saved_children {
      child.draw(ctx)?;
    }
    Ok(())
  }
}

pub fn new_color_rect(window_size: Size) -> ColorRect {
  let part = Point::new(window_size.w / 3.5, window_size.h / 3.5);
  let color = [0.66, 0.66, 0.66, 0.7];
  ColorRectBuilder::new()
    .point(part.clone())
    .size_from(window_size.w - part.x * 2.0, window_size.h - part.y * 2.0)
    .color(color)
    .origin(Origin::TopLeft)
    .build()
}

pub fn new_buttons(ctx: &mut Context, point: &Point, size: &Size, is_final: bool) -> Vec<Button> {
  let mut vec = Vec::new();
  let offset = Point::new(32.0, 32.0);
  let bottom_center = Point::new(
    point.x + size.w / 2.0,
    point.y + size.h - offset.y
  );
  let button_offset = Point::new(128.0, 0.0);
  let button_size = Size::new(64.0, 64.0);

  if !is_final {
    vec.push(
      ButtonBuilder::new(ctx)
      .point(bottom_center.clone() + button_offset.clone())
      .size(button_size.clone())
      .origin(Origin::BottomCenter)
      .button_type(ButtonType::StatsNext)
      .animation_from(vec![::join_str(buttons::IMAGES, "arrow_alt_3.png")], vec![1000])
      .facing(Facing::Left)
      .build().expect("Should build StatsNext Button")
    );

    vec.push(
      ButtonBuilder::new(ctx)
      .point(bottom_center.clone())
      .size(button_size.clone())
      .origin(Origin::BottomCenter)
      .button_type(ButtonType::StatsReset)
      .animation_from(vec![::join_str(buttons::IMAGES, "retry_alt_3.png")], vec![1000])
      .build().expect("Should build StatsReset Button")
    );
  }

  vec.push(
    ButtonBuilder::new(ctx)
    .point(if is_final {
      bottom_center
    } else {
      bottom_center - button_offset
    })
    .size(button_size)
    .origin(Origin::BottomCenter)
    .button_type(ButtonType::StatsToTitle)
    .animation_from(vec![::join_str(buttons::IMAGES, "arrow_alt_3.png")], vec![1000])
    .build().expect("Should build StatsToTitle Button")
  );

  vec
}

pub fn new_final_thankyou(ctx: &mut Context, window_size: &Size) -> AnimationRect {
  AnimationRect::new(
    window_size.center() - Point::new(0.0, 256.0),
    Size::new(512.0, 128.0),
    Origin::Center,
    Animation::new(
      ctx,
      vec![MISSING_IMAGE.to_string()],
      vec![1000]
    )
  )
}
