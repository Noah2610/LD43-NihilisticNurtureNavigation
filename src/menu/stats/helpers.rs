use ggez::{
  Context,
  GameResult,
  graphics,
};
use noframe::geo::prelude::*;

use settings::res::*;
use settings::menus::stats::*;
use settings::buttons;
use animation::prelude::*;
use menu::buttons::prelude::*;
use score::prelude::*;

const BUTTON_SIZE:     Size    = Size { w: 64.0, h: 64.0 };
const BUTTON_PADDING:  NumType = 64.0;
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
  saved_player:   Option<StatsText>,
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
      window_size.center() + Point::new(-offset_x, -BUTTON_OFFSET_Y - padding),
      TextOrigin::Left
    );

    let saved_player = if let Some(score) = &score.semantic_player() {
      Some(StatsText::new(
        graphics::Text::new(ctx, score, &font_saved)?,
        window_size.center() + Point::new(offset_x, -BUTTON_OFFSET_Y),
        TextOrigin::Right
      ))
    } else {
      None
    };

    let mut saved_children = Vec::new();
    for (i, s) in score.semantic_children().iter().enumerate() {
      let i_plus = if let Some(_) = saved_player {
        1
      } else { 0 };
      saved_children.push(
        StatsText::new(
          graphics::Text::new(ctx, &s, &font_saved)?,
          window_size.center() + Point::new(offset_x, -BUTTON_OFFSET_Y + padding * (i + i_plus) as NumType),
          TextOrigin::Right
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
    if let Some(saved_player) = &mut self.saved_player {
      saved_player.draw(ctx)?;
    }
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

pub fn new_buttons(ctx: &mut Context, window_size: &Size, is_final: bool) -> Vec<Button> {
  let mut vec = Vec::new();

  if !is_final {
    vec.push(
      ButtonBuilder::new(ctx)
      .point(window_size.center() + Point::new(BUTTON_SIZE.w + BUTTON_PADDING, BUTTON_OFFSET_Y))
      .size(BUTTON_SIZE.clone())
      .origin(Origin::Center)
      .button_type(ButtonType::StatsNext)
      .animation_from(vec![::join_str(buttons::IMAGES, "return.png")], vec![1000])
      .facing(Facing::Left)
      .build().expect("Should build StatsNext Button")
    );

    vec.push(
      ButtonBuilder::new(ctx)
      .point(window_size.center() + Point::new(0.0, BUTTON_OFFSET_Y))
      .size(BUTTON_SIZE.clone())
      .origin(Origin::Center)
      .button_type(ButtonType::StatsReset)
      .animation_from(vec![::join_str(buttons::IMAGES, "retry.png")], vec![1000])
      .build().expect("Should build StatsReset Button")
    );
  }

  vec.push(
    ButtonBuilder::new(ctx)
    .point(window_size.center() + if !is_final {
      Point::new(-(BUTTON_SIZE.w + BUTTON_PADDING), BUTTON_OFFSET_Y)
    } else {
      Point::new(0.0, BUTTON_OFFSET_Y)
    })
    .size(BUTTON_SIZE.clone())
    .origin(Origin::Center)
    .button_type(ButtonType::StatsToTitle)
    .animation_from(vec![::join_str(buttons::IMAGES, "return.png")], vec![1000])
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
