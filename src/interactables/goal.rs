use ggez::{
  Context,
  GameResult,
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;

use animation::Animation;
use animation::Facing;
use super::Interactable;
use super::animations::goal;
use persons::Person;
use id_generator::prelude::*;

#[derive(PartialEq)]
enum State {
  Zero,
  One,
  Two,
  Three,
  Four,
}

struct GoalAnimations {
  pub zero:  Animation,
  pub one:   Animation,
  pub two:   Animation,
  pub three: Animation,
  pub four:  Animation,
}

impl GoalAnimations {
  pub fn new(ctx: &mut Context) -> Self {
    Self {
      zero:  goal::new_zero_animation(ctx),
      one:   goal::new_one_animation(ctx),
      two:   goal::new_two_animation(ctx),
      three: goal::new_three_animation(ctx),
      four:  goal::new_four_animation(ctx),
    }
  }
}

pub struct Goal {
  point:       Point,
  size:        Size,
  origin:      Origin,
  state:       State,
  animations:  GoalAnimations,
  intersected: Vec<IdType>,
  id:          IdType
}

impl Goal {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point,
      size,
      origin:      Origin::TopLeft,
      state:       State::Zero,
      animations:  GoalAnimations::new(ctx),
      intersected: Vec::new(),
      id:          generate_id()
    }
  }

  fn animation(&self) -> &Animation {
    use self::State::*;
    match self.state {
      Zero  => &self.animations.zero,
      One   => &self.animations.one,
      Two   => &self.animations.two,
      Three => &self.animations.three,
      Four  => &self.animations.four,
    }
  }

  fn animation_mut(&mut self) -> &mut Animation {
    use self::State::*;
    match self.state {
      Zero  => &mut self.animations.zero,
      One   => &mut self.animations.one,
      Two   => &mut self.animations.two,
      Three => &mut self.animations.three,
      Four  => &mut self.animations.four,
    }
  }

  fn handle_animation(&mut self) {
    use self::State::*;
    let new_state = match self.intersected.len() {
      0 => Zero,
      1 => One,
      2 => Two,
      3 => Three,
      4 => Four,
      _ => Zero,
    };
    if new_state != self.state {
      self.state = new_state;
    }
  }
}

impl Mask for Goal {
  fn point(&self)         -> &Point { &self.point }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size { &self.size }
  fn origin(&self)        -> &Origin { &self.origin }
}

impl Entity for Goal {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.handle_animation();
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

impl IdGenerator for Goal {
  fn id(&self) -> IdType {
    self.id
  }
  fn set_id(&mut self, id: IdType) {
    self.id = id;
  }
}

impl Interactable for Goal {
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
