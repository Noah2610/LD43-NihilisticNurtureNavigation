use ggez::{
  Context,
  GameResult,
  graphics,
};
use noframe::geo::prelude::*;

use settings::res::*;
use settings::menus::stats::*;
use super::super::prelude::*;
use score::prelude::*;

const BUTTON_SIZE:     Size    = Size { w: 128.0, h: 64.0 };
const BUTTON_PADDING:  NumType = 16.0;
const BUTTON_OFFSET_Y: NumType = 128.0;

enum TextOrigin {
  Left,
  Right,
}

impl TextOrigin {
  pub fn val(&self) -> f32 {
    match self {
      TextOrigin::Left  => 0.0,
      TextOrigin::Right => 1.0,
    }
  }
}

struct StatsText {
  text:   graphics::Text,
  point:  Point,
  origin: TextOrigin,
}

impl StatsText {
  pub fn new(text: graphics::Text, point: Point, origin: TextOrigin) -> Self {
    Self {
      text,
      point,
      origin,
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let dest = graphics::Point2::from(&self.point);
    let param = graphics::DrawParam {
      dest,
      offset: graphics::Point2::new(self.origin.val(), 0.0),
      color:  Some(FONT_COLOR.into()),
      .. Default::default()
    };
    graphics::draw_ex(ctx, &self.text, param)?;
    Ok(())
  }
}

pub struct StatsTexts {
  score:          StatsText,
  saved_player:   StatsText,
  saved_children: Vec<StatsText>,
}

impl StatsTexts {
  pub fn new(ctx: &mut Context, score: Score, window_size: &Size) -> GameResult<Self> {
    let offset_x = BUTTON_SIZE.w * 1.5 + BUTTON_PADDING;
    let padding  = 32.0;

    let font_score = graphics::Font::new_px(ctx, fonts::DEFAULT, FONT_SIZE_SCORE)?;
    let font_saved = graphics::Font::new_px(ctx, fonts::DEFAULT, FONT_SIZE_SAVED)?;

    let score_text = StatsText::new(
      graphics::Text::new(ctx, &score.semantic_score(), &font_score)?,
      window_size.center() + Point::new(-offset_x, -BUTTON_OFFSET_Y),
      TextOrigin::Left,
    );

    let saved_player = StatsText::new(
      graphics::Text::new(ctx, &score.semantic_player(), &font_saved)?,
      window_size.center() + Point::new(offset_x, -BUTTON_OFFSET_Y),
      TextOrigin::Right,
    );

    let mut saved_children = Vec::new();
    for (i, s) in score.semantic_children().iter().enumerate() {
      saved_children.push(
        StatsText::new(
          graphics::Text::new(ctx, &s, &font_saved)?,
          window_size.center() + Point::new(offset_x, -BUTTON_OFFSET_Y + padding * (i + 1) as NumType),
          TextOrigin::Right,
        )
      );
    }

    Ok(StatsTexts {
      score: score_text,
      saved_player,
      saved_children,
    })
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.score.draw(ctx)?;
    self.saved_player.draw(ctx)?;
    for child in &mut self.saved_children {
      child.draw(ctx)?;
    }
    Ok(())
  }
}

pub fn new_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx, vec![
                 MISSING_IMAGE.to_string()
  ], vec![
  1000
  ])
}

pub fn new_buttons(ctx: &mut Context, window_size: &Size) -> Vec<Button> {
  vec![
    Button::new_with_origin(
      ctx,
      window_size.center() + Point::new(-(BUTTON_SIZE.w + BUTTON_PADDING), BUTTON_OFFSET_Y),
      BUTTON_SIZE.clone(),
      Origin::Center,
      ButtonType::StatsToTitle,
      vec![
      MISSING_IMAGE.to_string()
      ],
      vec![
      1000
      ]
    ),

    Button::new_with_origin(
      ctx,
      window_size.center() + Point::new(0.0, BUTTON_OFFSET_Y),
      BUTTON_SIZE.clone(),
      Origin::Center,
      ButtonType::StatsReset,
      vec![
      MISSING_IMAGE.to_string()
      ],
      vec![
      1000
      ]
    ),

    Button::new_with_origin(
      ctx,
      window_size.center() + Point::new(BUTTON_SIZE.w + BUTTON_PADDING, BUTTON_OFFSET_Y),
      BUTTON_SIZE.clone(),
      Origin::Center,
      ButtonType::StatsNext,
      vec![
      MISSING_IMAGE.to_string()
      ],
      vec![
      1000
      ]
    )
      ]
}
