use ggez::{
  Context,
  GameResult
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;
use noframe::geo::mask::misc::Side;

use settings::interactables::jump_pad::*;
use animation::Animation;
use animation::Facing;
use super::Interactable;
use super::animations::jump_pad;
use persons::Person;
use id_generator::prelude::*;

enum State {
  Main,
  Trigger
}

struct JumpPadAnimations {
  pub main:    Animation,
  pub trigger: Animation
}

impl JumpPadAnimations {
  pub fn new(ctx: &mut Context) -> Self {
    Self {
      main:    jump_pad::new_main_animation(ctx),
      trigger: jump_pad::new_trigger_animation(ctx),
    }
  }
}

pub struct JumpPad {
  point:       Point,
  size:        Size,
  origin:      Origin,
  state:       State,
  animations:  JumpPadAnimations,
  intersected: Vec<IdType>,
  id:          IdType
}

impl JumpPad {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point,
      size,
      origin:      Origin::TopLeft,
      state:       State::Main,
      animations:  JumpPadAnimations::new(ctx),
      intersected: Vec::new(),
      id:          generate_id()
    }
  }

  pub fn intersects_center<T: Mask>(&self, other: &T) -> bool {
    let self_sides = self.sides().map( |(side, val)| {
      match side {
        Side::Top | Side::Bottom => val,
        Side::Left  => val + self.size.w / 2.0,
        Side::Right => val - self.size.w / 2.0,
      }
    }).collect::<_>();
    let other_sides = other.sides();
    Self::sides_intersect(self_sides, other_sides)
  }

  fn animation(&self) -> &Animation {
    match self.state {
      State::Main    => &self.animations.main,
      State::Trigger => &self.animations.trigger
    }
  }

  fn animation_mut(&mut self) -> &mut Animation {
    match self.state {
      State::Main    => &mut self.animations.main,
      State::Trigger => &mut self.animations.trigger
    }
  }
}

impl Mask for JumpPad {
  fn point(&self)         -> &Point { &self.point }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size { &self.size }
  fn origin(&self)        -> &Origin { &self.origin }
}

impl Entity for JumpPad {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let State::Trigger = self.state {
      if self.animations.trigger.played() > 1 {
        self.animations.trigger.reset();
        self.state = State::Main;
      }
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

impl IdGenerator for JumpPad {
  fn id(&self) -> IdType {
    self.id
  }
  fn set_id(&mut self, id: IdType) {
    self.id = id;
  }
}

impl Interactable for JumpPad {
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
    self.state = State::Trigger;
    self.animations.trigger.reset();
    person.set_velocity_y(-JUMP_SPEED);
    person.set_velocity_x(0.0);
    person.stop_walking();
  }
}
