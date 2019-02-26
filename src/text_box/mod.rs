pub mod builder;

pub mod prelude {
  pub use super::TextBox;
  pub use super::TextBoxBuilder;
  pub use super::TextOrigin;
}

pub use self::builder::TextBoxBuilder;

use ggez::{
  Context,
  GameResult,
  graphics::{ self, Image, Text },
};
use noframe::geo::prelude::*;
use noframe::color::Color;
use noframe::entity::Entity;

use settings::color_rect;

pub enum TextOrigin {
  TopLeft,
  TopRight,
  TopCenter,
  BottomLeft,
  BottomRight,
  BottomCenter,
  CenterLeft,
  CenterRight,
  Center,
}

impl TextOrigin {
  pub fn val(&self) -> (f32, f32) {
    use self::TextOrigin::*;
    match self {
      TopLeft      => (0.0, 0.0),
      TopRight     => (1.0, 0.0),
      TopCenter    => (0.5, 0.0),
      BottomLeft   => (0.0, 1.0),
      BottomRight  => (1.0, 1.0),
      BottomCenter => (0.5, 1.0),
      CenterLeft   => (0.0, 0.5),
      CenterRight  => (1.0, 0.5),
      Center       => (0.5, 0.5),
    }
  }
}

pub struct TextBox {
  point:       Point,
  size:        Size,
  origin:      Origin,
  bg_color:    Option<Color>,
  image:       Option<Image>,
  text:        Option<Text>,
  text_color:  Option<Color>,
  text_origin: TextOrigin,
}

impl TextBox {
  pub fn new(
    point:       Point,
    size:        Size,
    origin:      Origin,
    bg_color:    Option<Color>,
    image:       Option<Image>,
    text:        Option<Text>,
    text_color:  Option<Color>,
    text_origin: TextOrigin
  ) -> Self {
    Self {
      point,
      size,
      origin,
      bg_color,
      image,
      text,
      text_color,
      text_origin,
    }
  }

  fn image_scale(&self) -> Option<(NumType, NumType)> {
    if let Some(image) = &self.image {
      let size = Size::new(image.width() as NumType, image.height() as NumType);
      Some((
          self.size.w / size.w,
          self.size.h / size.h
      ))
    } else { None }
  }
}

impl Mask for TextBox {
  fn point(&self)         -> &Point     { &self.point }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size }
  fn origin(&self)        -> &Origin    { &self.origin }
}

impl Entity for TextBox {
  fn color(&self) -> Color {
    self.bg_color.unwrap_or(color_rect::DEFAULT_COLOR)
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    // IMAGE
    if let Some(image) = &self.image {
      let mut param = graphics::DrawParam {
        dest: graphics::Point2::from(&self.top_left()),
        .. Default::default()
      };
      let scale = self.image_scale().unwrap();
      param.scale = graphics::Point2::new(scale.0, scale.1);
      graphics::draw_ex(ctx, image, param)?;

    // COLOR
    } else if let Some(color) = &self.bg_color {
      let prev_color = graphics::get_color(ctx);
      let point = self.top_left();
      let size  = self.size();
      let rect  = [
        point.x, point.y,
        size.w,  size.h
      ];
      self.draw_rect(ctx, rect)?;
      graphics::set_color(ctx, prev_color)?;
    }

    // TEXT
    if let Some(text) = &self.text {
      let offset = self.text_origin.val();
      let mut param = graphics::DrawParam {
        dest:   graphics::Point2::from(&self.point),
        color:  self.text_color.map( |c| c.into() ),
        offset: graphics::Point2::new(offset.0, offset.1),
        .. Default::default()
      };
      graphics::draw_ex(ctx, text, param)?;
    }
    Ok(())
  }
}
