use ggez::{
  GameResult,
  graphics::{ self, Image, Text, FilterMode },
};
use noframe::geo::prelude::*;
use noframe::color::{ self, Color };

use super::TextBox;
use super::TextOrigin;

pub struct TextBoxBuilder {
  point:       Point,
  size:        Size,
  origin:      Origin,
  bg_color:    Option<Color>,
  image:       Option<Image>,
  text:        Option<Text>,
  text_color:  Option<Color>,
  text_origin: TextOrigin,
}

impl TextBoxBuilder {
  pub fn new() -> Self {
    Self {
      point:       Point::new(0.0, 0.0),
      size:        Size::new(64.0, 64.0),
      origin:      Origin::TopLeft,
      bg_color:    None,
      image:       None,
      text:        None,
      text_color:  None,
      text_origin: TextOrigin::TopLeft,
    }
  }

  pub fn point(mut self, point: Point) -> Self {
    self.point = point;
    self
  }
  pub fn point_from(mut self, x: NumType, y: NumType) -> Self {
    self.point = Point::new(x, y);
    self
  }
  pub fn size(mut self, size: Size) -> Self {
    self.size = size;
    self
  }
  pub fn size_from(mut self, w: NumType, h: NumType) -> Self {
    self.size = Size::new(w, h);
    self
  }
  pub fn origin(mut self, origin: Origin) -> Self {
    self.origin = origin;
    self
  }
  pub fn bg_color(mut self, bg_color: Color) -> Self {
    self.bg_color = Some(bg_color);
    self
  }
  pub fn image(mut self, mut image: Image) -> Self {
    image.set_filter(FilterMode::Nearest);
    self.image = Some(image);
    self
  }
  pub fn text(mut self, text: Text) -> Self {
    self.text = Some(text);
    self
  }
  pub fn text_color(mut self, text_color: Color) -> Self {
    self.text_color = Some(text_color);
    self
  }
  pub fn text_origin(mut self, text_origin: TextOrigin) -> Self {
    self.text_origin = text_origin;
    self
  }

  pub fn build(self) -> TextBox {
    TextBox::new(
      self.point,
      self.size,
      self.origin,
      self.bg_color,
      self.image,
      self.text,
      self.text_color,
      self.text_origin
    )
  }
}
