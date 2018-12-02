use ggez::{
  GameResult,
  Context
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;
use noframe::deltatime::Deltatime;

use settings::child::*;
use super::Axis;
use super::AnimState;
use super::WalkDirection;
use super::person_animations::PersonAnimations;
use animation::Facing;

pub struct Child {
  point:          Point,
  size:           Size,
  origin:         Origin,
  velocity:       Point,
  max_velocity:   Point,
  has_moved:      Vec<Axis>,
  animations:     PersonAnimations,
  anim_state:     AnimState,
  walk_direction: WalkDirection,
  facing:         Facing,
  dt:             Deltatime
}

impl Child {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point,
      size,
      origin:         Origin::TopLeft,
      velocity:       Point::new(0.0, 0.0),
      max_velocity:   Point::new(MAX_SPEED, MAX_JUMP_SPEED),
      has_moved:      Vec::new(),
      animations:     PersonAnimations::new_child_animations(ctx),
      anim_state:     AnimState::Idle,
      walk_direction: WalkDirection::Right,
      facing:         Facing::Right,
      dt:             Deltatime::new()
    }
  }

  fn handle_walk(&mut self) {
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

  fn handle_decrease_velocity(&mut self) {
    let decr_vel = Point::new(
      if !self.has_moved(Axis::X) {
        SPEED_DECREASE_X
      } else { 0.0 },
      if !self.has_moved(Axis::Y) {
        SPEED_DECREASE_Y
      } else { 0.0 }
    );
    self.decrease_velocity(&decr_vel);
    self.has_moved.clear();
  }

  fn moved_on_axis(&mut self, axis: Axis) {
    if !self.has_moved.iter().any( |a| &axis == a ) {
      self.has_moved.push(axis);
    }
  }

  fn has_moved(&self, axis: Axis) -> bool {
    self.has_moved.iter().any( |a| &axis == a )
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
    self.animations.handle_state(&self.anim_state)?;
    self.handle_walk();
    self.handle_decrease_velocity();
    self.handle_facing();
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

impl Movement for Child { }
