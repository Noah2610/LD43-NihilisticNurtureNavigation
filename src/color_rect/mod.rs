pub mod builder;

pub mod prelude {
  pub use super::ColorRect;
  pub use super::ColorRectBuilder;
}

pub use self::builder::ColorRectBuilder;

use ggez::{
  Context,
  GameResult,
  graphics,
};

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

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    let color = graphics::get_color(ctx);
    let point = self.top_left();
    let size  = self.size();
    let rect  = [
      point.x, point.y,
      size.w,  size.h
    ];
    self.draw_rect(ctx, rect)?;
    graphics::set_color(ctx, color)?;
    Ok(())
  }
}
