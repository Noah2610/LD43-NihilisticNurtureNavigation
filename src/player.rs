use ggez::{
  GameResult,
  Context,
  graphics::Image
};

use noframe::geo::prelude::*;
use noframe::entity::Entity;

pub struct Player {
  point:  Point,
  size:   Size,
  origin: Origin,
  image:  Image
}

impl Player {
}

impl Mask for Player {
  fn point(&self) -> &Point {
    &self.point
  }
  fn point_mut(&mut self) -> &mut Point {
    &mut self.point
  }
  fn size(&self) -> &Size {
    &self.size
  }
  fn origin(&self) -> &Origin {
    &self.origin
  }
}

impl Entity for Player {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }
}
