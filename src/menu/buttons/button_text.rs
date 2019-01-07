pub mod prelude {
  pub use super::ButtonText;
  pub use super::ButtonTextTuple;
}

use ggez::{
  GameResult,
  Context,
  graphics::{ self, Point2, Text },
};
use noframe::geo::prelude::*;

//                          point  size  origin  text
pub type ButtonTextTuple = (Point, Size, Origin, Text);

pub struct ButtonText {
  point:  Point,
  size:   Size,
  origin: Origin,
  text:   Text,
}

impl ButtonText {
  pub fn new(point: Point, size: Size, origin: Origin, text: Text) -> Self {
    Self {
      point,
      size,
      origin,
      text,
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let param = graphics::DrawParam {
      dest:  Point2::from(&self.top_left()),
      color: Some(noframe::color::WHITE.into()),  // TODO: custom color config
      .. Default::default()
    };
    graphics::draw_ex(ctx, &self.text, param)?;
    Ok(())
  }
}

impl Mask for ButtonText {
  fn point(&self)         -> &Point     { &self.point     }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size      { &self.size      }
  fn origin(&self)        -> &Origin    { &self.origin    }
}
