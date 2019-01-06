use ggez::{
  Context,
  GameResult
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;

use animation::Animation;
use animation::Facing;
use super::Interactable;
use super::animations::door;
use persons::Person;
use id_generator::prelude::*;

#[derive(PartialEq)]
pub enum State {
  Open,
  Closed,
  Opening,
  Closing
}

struct DoorAnimations {
  pub open:    Animation,
  pub closed:  Animation,
  pub opening: Animation,
  pub closing: Animation
}

impl DoorAnimations {
  pub fn new(ctx: &mut Context, color: &str) -> Self {
    Self {
      open:    door::new_open_animation(ctx, color),
      closed:  door::new_closed_animation(ctx, color),
      opening: door::new_opening_animation(ctx, color),
      closing: door::new_closing_animation(ctx, color),
    }
  }
}

pub struct Door {
  point:       Point,
  size:        Size,
  origin:      Origin,
  state:       State,
  animations:  DoorAnimations,
  intersected: Vec<IdType>,
  id:          IdType
}

impl Door {
  pub fn new(ctx: &mut Context, point: Point, size: Size, id: IdType, color: &str, state: State) -> Self {
    Self {
      point,
      size,
      origin:      Origin::TopLeft,
      state,
      animations:  DoorAnimations::new(ctx, color),
      intersected: Vec::new(),
      id
    }
  }

  fn animation(&self) -> &Animation {
    match self.state {
      State::Open    => &self.animations.open,
      State::Closed  => &self.animations.closed,
      State::Opening => &self.animations.opening,
      State::Closing => &self.animations.closing
    }
  }

  fn animation_mut(&mut self) -> &mut Animation {
    match self.state {
      State::Open    => &mut self.animations.open,
      State::Closed  => &mut self.animations.closed,
      State::Opening => &mut self.animations.opening,
      State::Closing => &mut self.animations.closing
    }
  }

  pub fn is_solid(&self) -> bool {
    self.state == State::Closed || self.state == State::Closing
  }
}

impl Mask for Door {
  fn point(&self)         -> &Point { &self.point }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size { &self.size }
  fn origin(&self)        -> &Origin { &self.origin }
}

impl Entity for Door {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    if self.animation().played() > 0 {
      match self.state {
        State::Opening => {
          self.animation_mut().reset();
          self.state = State::Open;
        },
        State::Closing => {
          self.animation_mut().reset();
          self.state = State::Closed;
        },
        _ => ()
      };
    }
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

impl IdGenerator for Door {
  fn id(&self) -> IdType {
    self.id
  }
  fn set_id(&mut self, id: IdType) {
    self.id = id;
  }
}

impl Interactable for Door {
  fn get_intersected(&self) -> &Vec<IdType> {
    &self.intersected
  }
  fn add_intersected(&mut self, id: IdType) {
    self.intersected.push(id);
  }
  fn rm_intersected_at(&mut self, index: usize) {
    self.intersected.remove(index);
  }

  fn trigger<T: Person>(&mut self, _person: &mut T) {
    match self.state {
      State::Open   => self.state = State::Closing,
      State::Closed => self.state = State::Opening,
      _             => ()
    };
  }
}
