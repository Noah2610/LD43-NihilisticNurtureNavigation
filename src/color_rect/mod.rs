pub mod builder;

pub use self::builder::ColorRectBuilder;

use noframe::geo::prelude::*;
use noframe::color::Color;
use noframe::entity::Entity;

pub struct ColorRect {
  point:  Point,
  size:   Size,
  origin: Origin,
  color:  Color,
}

impl ColorRect {
  pub fn new(point: Point, size: Size, origin: Origin, color: Color) -> Self {
    Self { point, size, origin, color }
  }
}

impl Mask for ColorRect {
  fn point(&self)         -> &Point { &self.point }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size { &self.size }
  fn origin(&self)        -> &Origin { &self.origin }
}

impl Entity for ColorRect {
  fn color(&self) -> Color { self.color }
}
