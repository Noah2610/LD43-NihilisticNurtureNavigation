use ggez::{
  Context,
  GameResult
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;

use super::Interactable;
use persons::Person;
use id_generator::prelude::*;

pub struct Solidifier {
  point:       Point,
  size:        Size,
  origin:      Origin,
  intersected: Vec<IdType>,
  id:          IdType
}

impl Solidifier {
  pub fn new(_ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point,
      size,
      origin:      Origin::TopLeft,
      intersected: Vec::new(),
      id:          generate_id()
    }
  }

}

impl Mask for Solidifier {
  fn point(&self)         -> &Point { &self.point }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size { &self.size }
  fn origin(&self)        -> &Origin { &self.origin }
}

impl Entity for Solidifier {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }

  fn draw(&self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }

  fn draw_offset(&self, _ctx: &mut Context, _offset: &Point) -> GameResult<()> {
    Ok(())
  }
}

impl IdGenerator for Solidifier {
  fn id(&self) -> IdType {
    self.id
  }
  fn set_id(&mut self, id: IdType) {
    self.id = id;
  }
}

impl Interactable for Solidifier {
  fn get_intersected(&self) -> &Vec<IdType> {
    &self.intersected
  }
  fn add_intersected(&mut self, id: IdType) {
    self.intersected.push(id);
  }
  fn rm_intersected_at(&mut self, index: usize) {
    self.intersected.remove(index);
  }

  fn trigger<T: Person>(&mut self, person: &mut T) {
    person.solidify();
  }
}
