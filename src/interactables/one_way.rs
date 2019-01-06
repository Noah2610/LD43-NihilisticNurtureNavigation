use ggez::{
  Context,
  GameResult
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;

use animation::Animation;
use animation::Facing;
use super::Interactable;
use super::animations::one_way;
use persons::Person;
use id_generator::prelude::*;

enum State {
  Main
}

struct OneWayAnimations {
  pub main: Animation
}

impl OneWayAnimations {
  pub fn new(ctx: &mut Context) -> Self {
    Self {
      main: one_way::new_main_animation(ctx)
    }
  }
}

pub struct OneWay {
  point:       Point,
  size:        Size,
  origin:      Origin,
  state:       State,
  animations:  OneWayAnimations,
  intersected: Vec<IdType>,
  id:          IdType
}

impl OneWay {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point,
      size,
      origin:      Origin::TopLeft,
      state:       State::Main,
      animations:  OneWayAnimations::new(ctx),
      intersected: Vec::new(),
      id:          generate_id()
    }
  }

  fn animation(&self) -> &Animation {
    match self.state {
      State::Main => &self.animations.main
    }
  }

  fn animation_mut(&mut self) -> &mut Animation {
    match self.state {
      State::Main => &mut self.animations.main,
    }
  }
}

impl Mask for OneWay {
  fn point(&self)         -> &Point { &self.point }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size { &self.size }
  fn origin(&self)        -> &Origin { &self.origin }
}

impl Entity for OneWay {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.animation_mut().update()?;
    Ok(())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    self.animation().draw(ctx, self.point(), self.size(), &Facing::default())
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    self.animation().draw_offset(ctx, self.point(), self.size(), &Facing::default(), offset)
  }
}

impl IdGenerator for OneWay {
  fn id(&self) -> IdType {
    self.id
  }
  fn set_id(&mut self, id: IdType) {
    self.id = id;
  }
}

impl Interactable for OneWay {
  fn get_intersected(&self) -> &Vec<IdType> {
    &self.intersected
  }
  fn add_intersected(&mut self, id: IdType) {
    self.intersected.push(id);
  }
  fn rm_intersected_at(&mut self, index: usize) {
    self.intersected.remove(index);
  }

  fn trigger<T: Person>(&mut self, _person: &mut T) { }
}
