use ggez::{
  GameResult,
  Context,
  event::Keycode
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;
use noframe::deltatime::Deltatime;

use settings::player::*;
use super::Person;
use super::Axis;
use super::AnimState;
use super::WalkDirection;
use super::person_animations::PersonAnimations;
use animation::Facing;
use gravity::Gravity;
use id_generator::prelude::*;

pub struct Player {
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
  is_jumping:       bool,
  id:               IdType,
  solid:            bool,
  dt:               Deltatime
}

impl Player {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point,
      size,
      origin:           Origin::TopLeft,
      velocity:         Point::new(0.0, 0.0),
      max_velocity:     Point::new(MAX_VELOCITY_X, MAX_VELOCITY_Y),
      has_moved:        Vec::new(),
      animations:       PersonAnimations::new_player_animations(ctx),
      anim_state:       AnimState::Idle,
      walk_direction:   WalkDirection::Still,
      facing:           Facing::Right,
      gravity_increase: Point::new(0.0, GRAVITY_INCREASE),
      is_jumping:       false,
      id:               generate_id(),
      solid:            false,
      dt:               Deltatime::new()
    }
  }

  pub fn keys_pressed(&mut self, keycodes: &Vec<Keycode>) {
    if self.is_solid() { return; }
    for keycode in keycodes {
      if let Some(point) = match keycode {
        &controls::LEFT => {
          if !self.has_moved(Axis::X) {
            self.moved_on_axis(Axis::X);
            Some(Point::new( -SPEED_INCREASE, 0.0 ))
          } else { None }
        }
        &controls::RIGHT => {
          if !self.has_moved(Axis::X) {
            self.moved_on_axis(Axis::X);
            Some(Point::new( SPEED_INCREASE, 0.0 ))
          } else { None }
        }
        _ => None
      } {
        self.add_velocity(&point);
      }
    }
  }

  pub fn key_down(&mut self, keycode: &Keycode) {
    if self.is_solid() { return; }
    if let &controls::JUMP = keycode {
      self.jump();
    }
  }

  pub fn key_up(&mut self, keycode: &Keycode) {
    if self.is_solid() { return; }
    if let &controls::JUMP = keycode {
      if self.is_jumping && self.velocity.y < 0.0 {
        self.add_velocity(&Point::new(0.0, JUMP_KILL_VELOCITY));
        if self.velocity.y > 0.0 {
          self.set_velocity_y(0.0);
        }
      }
    }
  }

  fn jump(&mut self) {
    if self.is_jumping { return; }
    self.is_jumping = true;
    self.add_velocity(&Point::new(0.0, -JUMP_SPEED));
  }

  pub fn stop_jumping(&mut self) {
    self.is_jumping = false;
  }

  fn moved_on_axis(&mut self, axis: Axis) {
    if !self.has_moved.iter().any( |a| &axis == a ) {
      self.has_moved.push(axis);
    }
  }

  fn has_moved(&self, axis: Axis) -> bool {
    self.has_moved.iter().any( |a| &axis == a )
  }

  fn handle_decrease_velocity(&mut self) {
    let decr_vel = Point::new(
      if !self.has_moved(Axis::X) {
        SPEED_DECREASE_X
      } else { 0.0 },
      if false && !self.has_moved(Axis::Y) {  // TODO I don't think we need to decrease y velocity automatically
        SPEED_DECREASE_Y
      } else { 0.0 }
    );
    self.decrease_velocity(&decr_vel);
    self.has_moved.clear();
  }

  fn handle_anim_state(&mut self) {
    self.anim_state = match self.velocity.as_tup() {
      (_x, y) if y <  0.0 => AnimState::Jump,
      (_x, y) if y >  0.0 => AnimState::Fall,
      (x, _y) if x != 0.0 => AnimState::Walk,
      _                   => AnimState::Idle
    };
  }

  fn handle_walk_direction(&mut self) {
    self.walk_direction = match self.velocity.x {
      x if x > 0.0 => {
        self.facing = Facing::Right;
        WalkDirection::Right
      },
      x if x < 0.0 => {
        self.facing = Facing::Left;
        WalkDirection::Left
      },
      _ => WalkDirection::Still,
    };
  }
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
    self.handle_anim_state();
    self.animations.get_by_state_mut(&self.anim_state).update()?;
    self.handle_decrease_velocity();
    self.handle_walk_direction();
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

impl Velocity for Player {
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

impl Movement for Player {}

impl Gravity for Player {
  fn gravity_increase(&self) -> &Point {
    &self.gravity_increase
  }
}

impl Person for Player {
  fn is_solid(&self) -> bool {
    self.solid
  }
  fn solidify(&mut self) {
    self.solid = true;
  }
}

impl IdGenerator for Player {
  fn id(&self) -> IdType {
    self.id
  }
  fn set_id(&mut self, id: IdType) {
    self.id = id;
  }
}
