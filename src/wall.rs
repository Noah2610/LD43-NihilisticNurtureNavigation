use noframe::geo::prelude::*;
use noframe::entity::Entity;

pub struct Wall {
  point:  Point,
  size:   Size,
  origin: Origin
}

impl Wall {

}

impl Mask for Wall {
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

impl Entity for Wall { }
