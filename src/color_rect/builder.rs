use noframe::geo::prelude::*;
use noframe::color::Color;

use settings::color_rect::*;
use super::ColorRect;

pub struct ColorRectBuilder {
  point:  Point,
  size:   Size,
  origin: Origin,
  color:  Color,
}

impl ColorRectBuilder {
  pub fn new() -> Self {
    Self {
      point:  Point::new(0.0, 0.0),
      size:   Size::new(64.0, 64.0),
      origin: Origin::TopLeft,
      color:  DEFAULT_COLOR,
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
  pub fn color(mut self, color: Color) -> Self {
    self.color = color;
    self
  }

  pub fn build(self) -> ColorRect {
    ColorRect::new(
      self.point,
      self.size,
      self.origin,
      self.color
    )
  }
}
