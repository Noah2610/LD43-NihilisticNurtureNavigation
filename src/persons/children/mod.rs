mod child_type;

mod larry;
mod thing;
mod bloat;

pub use self::child_type::ChildType;

use ggez::{
  GameResult,
  Context
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;
use noframe::deltatime::Deltatime;

use settings::child::*;
use super::Person;
use super::Axis;
use super::AnimState;
use super::WalkDirection;
use super::person_animations::PersonAnimations;
use animation::Facing;
use gravity::Gravity;
use id_generator::prelude::*;

pub struct Child {
  point:            Point,
  size:             Size,
  origin:           Origin,
  velocity:         Point,
  max_velocity:     Point,
  has_moved:        Vec<Axis>,
  animations:       PersonAnimations,
  anim_state:       AnimState,
  walk_direction:   WalkDirection,
  facing:           Facing,
  gravity_increase: Point,
  pub child_type:   ChildType,
  id:               IdType,
  solid:            bool,
  dt:               Deltatime
}

impl Child {
  pub fn new(ctx: &mut Context, point: Point, size: Size, child_type: ChildType) -> Self {
    Self {
      point,
      size,
      origin:           Origin::TopLeft,
      velocity:         Point::new(0.0, 0.0),
      max_velocity:     Point::new(MAX_VELOCITY_X, MAX_VELOCITY_Y),
      has_moved:        Vec::new(),
      animations:       PersonAnimations::new_child_animations(ctx, &child_type),
      anim_state:       AnimState::Idle,
      walk_direction:   WalkDirection::Still,
      facing:           Facing::default(),
      gravity_increase: Point::new(0.0, GRAVITY_INCREASE),
      child_type,
      id:               generate_id(),
      solid:            false,
      dt:               Deltatime::new()
    }
  }

  pub fn is_not_walking(&self) -> bool {
    WalkDirection::Still == self.walk_direction && self.velocity.x == 0.0 && self.on_floor()
  }

  pub fn state_is_still(&self) -> bool {
    WalkDirection::Still == self.walk_direction
  }

  pub fn is_walking(&self) -> bool {
    !self.is_not_walking()
  }

  // Returns true if child successfully started walking left
  pub fn try_walk_left(&mut self) -> bool {
    if let WalkDirection::Still = self.walk_direction {
      self.walk_direction = WalkDirection::Left;
      true
    } else { false }
  }

  // Returns true if child successfully started walking right
  pub fn try_walk_right(&mut self) -> bool {
    if let WalkDirection::Still = self.walk_direction {
      self.walk_direction = WalkDirection::Right;
      true
    } else { false }
  }

  pub fn walk_direction_mult(&self) -> f32 {
    match self.walk_direction {
      WalkDirection::Right =>  1.0,
      WalkDirection::Left  => -1.0,
      WalkDirection::Still => -1.0,
    }
  }

  pub fn stop_walking(&mut self) {
    self.walk_direction = WalkDirection::Still;
  }

  fn handle_walk(&mut self) {
    if self.is_solid() { return; }
    match self.walk_direction {
      WalkDirection::Left  => {
        self.add_velocity(&Point::new(-SPEED_INCREASE, 0.0));
        self.moved_on_axis(Axis::X);
      },
      WalkDirection::Right => {
        self.add_velocity(&Point::new( SPEED_INCREASE, 0.0));
        self.moved_on_axis(Axis::X);
      },
      WalkDirection::Still => ()
    };
  }

  fn handle_anim_state(&mut self) {
    self.anim_state = match self.velocity.as_tup() {
      (_x, y) if y <  0.0 => AnimState::Jump,
      (_x, y) if y >  0.0 => AnimState::Fall,
      (x, _y) if x != 0.0 => AnimState::Walk,
      _                   => AnimState::Idle
    };
  }

  fn handle_facing(&mut self) {
    match self.walk_direction {
      WalkDirection::Right => self.facing = Facing::Right,
      WalkDirection::Left  => self.facing = Facing::Left,
      WalkDirection::Still => ()
    };
  }
}

impl Mask for Child {
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

impl Entity for Child {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.handle_anim_state();
    self.animations.get_by_state_mut(&self.anim_state).update()?;
    self.handle_walk();
    self.handle_decrease_velocity();
    self.handle_facing();
    self.update_gravity();
    self.dt.update();
    Ok(())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    self.animations.get_by_state(&self.anim_state).draw(ctx, &self.point, &self.size, &self.facing)
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    self.animations.get_by_state(&self.anim_state).draw_offset(ctx, &self.point, &self.size, &self.facing, offset)
  }
}

impl Velocity for Child {
  fn velocity(&self) -> &Point {
    &self.velocity
  }
  fn usable_velocity(&self) -> Point {
    self.velocity.mult_axes_by(self.dt.secs())
  }
  fn velocity_mut(&mut self) -> &mut Point {
    &mut self.velocity
  }
  fn max_velocity(&self) -> Point {
    self.max_velocity.clone()
  }
}

impl Movement for Child {}

impl Gravity for Child {
  fn gravity_increase(&self) -> Point {
    self.gravity_increase.mult_axes_by(self.dt.secs())
  }
}

impl Person for Child {
  fn reset_dt(&mut self, dt: &Deltatime) {
    self.dt = dt.clone();
  }

  fn moved_axes(&self) -> &Vec<Axis> {
    &self.has_moved
  }
  fn add_moved_axis(&mut self, axis: Axis) {
    self.has_moved.push(axis);
  }
  fn clear_moved_axes(&mut self) {
    self.has_moved.clear();
  }
  fn speed_decrease(&self) -> Point {
    Point::new(SPEED_DECREASE_X * self.dt.secs(), SPEED_DECREASE_Y * self.dt.secs())
  }

  fn is_solid(&self) -> bool {
    self.solid
  }
  fn solidify(&mut self) {
    self.solid = true;
  }
  fn unsolidify(&mut self) {
    self.solid = false;
  }
  fn on_jump_pad(&mut self) {
    self.stop_walking();
  }
}

impl IdGenerator for Child {
  fn id(&self) -> IdType {
    self.id
  }
  fn set_id(&mut self, id: IdType) {
    self.id = id;
  }
}
