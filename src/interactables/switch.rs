use ggez::{
  Context,
  GameResult,
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;

use animation::Animation;
use animation::Facing;
use super::Interactable;
use super::animations::switch;
use persons::Person;
use id_generator::prelude::*;

enum State {
  On,
  Off,
  TurningOn,
  TurningOff
}

struct SwitchAnimations {
  pub on:          Animation,
  pub off:         Animation,
  pub turning_on:  Animation,
  pub turning_off: Animation
}

impl SwitchAnimations {
  pub fn new(ctx: &mut Context, color: &str) -> Self {
    Self {
      on:          switch::new_on_animation(ctx, color),
      off:         switch::new_off_animation(ctx, color),
      turning_on:  switch::new_turning_on_animation(ctx, color),
      turning_off: switch::new_turning_off_animation(ctx, color)
    }
  }
}

pub struct Switch {
  point:                 Point,
  size:                  Size,
  origin:                Origin,
  state:                 State,
  animations:            SwitchAnimations,
  intersected:           Vec<IdType>,
  id:                    IdType,
  triggers:              Vec<IdType>,
  trigger_interactables: bool
}

impl Switch {
  pub fn new(ctx: &mut Context, point: Point, size: Size, id: IdType, color: &str, triggers: Vec<IdType>) -> Self {
    Self {
      point,
      size,
      origin:                Origin::TopLeft,
      state:                 State::Off,
      animations:            SwitchAnimations::new(ctx, color),
      intersected:           Vec::new(),
      id,
      triggers,
      trigger_interactables: false
    }
  }

  pub fn get_interactables_to_trigger(&self) -> Vec<IdType> {
    if !self.trigger_interactables { return vec![]; }
    self.triggers.clone()
  }

  pub fn interactables_triggered(&mut self) {
    self.trigger_interactables = false;
  }

  fn animation(&self) -> &Animation {
    match self.state {
      State::On         => &self.animations.on,
      State::Off        => &self.animations.off,
      State::TurningOn  => &self.animations.turning_on,
      State::TurningOff => &self.animations.turning_off
    }
  }

  fn animation_mut(&mut self) -> &mut Animation {
    match self.state {
      State::On         => &mut self.animations.on,
      State::Off        => &mut self.animations.off,
      State::TurningOn  => &mut self.animations.turning_on,
      State::TurningOff => &mut self.animations.turning_off
    }
  }

  fn facing(&self) -> Facing {
    match self.state {
      State::On         => Facing::Right,
      State::Off        => Facing::Left,
      State::TurningOn  => Facing::Right,
      State::TurningOff => Facing::Left,
    }
  }
}

impl Mask for Switch {
  fn point(&self)         -> &Point { &self.point }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size { &self.size }
  fn origin(&self)        -> &Origin { &self.origin }
}

impl Entity for Switch {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    if self.animation().played() >= 1 {
      match self.state {
        State::TurningOn => {
          self.animation_mut().reset();
          self.state = State::On;
        }
        State::TurningOff => {
          self.animation_mut().reset();
          self.state = State::Off;
        }
        _ => ()
      };
    }
    self.animation_mut().update()?;
    Ok(())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    self.animation().draw(ctx, self.point(), self.size(), &self.facing())
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    self.animation().draw_offset(ctx, self.point(), self.size(), &self.facing(), offset)
  }
}

impl IdGenerator for Switch {
  fn id(&self) -> IdType {
    self.id
  }
  fn set_id(&mut self, id: IdType) {
    self.id = id;
  }
}

impl Interactable for Switch {
  fn get_intersected(&self) -> &Vec<IdType> {
    &self.intersected
  }
  fn add_intersected(&mut self, id: IdType) {
    self.intersected.push(id);
  }
  fn rm_intersected_at(&mut self, index: usize) {
    self.intersected.remove(index);
  }

  fn trigger<T: Person>(&mut self, _ctx: &mut Context, _person: &mut T) {
    match &self.state {
      State::On  => self.state = State::TurningOff,
      State::Off => self.state = State::TurningOn,
      _          => return,
    };
    self.trigger_interactables = true;
  }
}
