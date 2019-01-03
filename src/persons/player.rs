use std::time::{ Instant, Duration };

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
use score::prelude::*;

struct MultiJump {
  count:             u8,
  max_count:         u8,
  strength:          NumType,
  direction:         Option<WalkDirection>,
  rotation:          f32,
  rotation_increase: f32,
  wait:              Duration,
  final_jump:        bool,
  last_update:       Instant,
}

impl MultiJump {
  pub fn new(strength: NumType) -> Self {
    Self {
      count:             0,
      max_count:         3,
      strength,
      direction:         None,
      rotation:          0.0,
      rotation_increase: 5.0,
      wait:              Duration::from_millis(500),
      final_jump:        false,
      last_update:       Instant::now(),
    }
  }

  pub fn jumped(&mut self, dir: &WalkDirection) {
    if let WalkDirection::Still = dir {
      self.reset();
      return;
    }
    if let None = self.direction {
      self.direction = Some(dir.clone());
    }
    if let Some(jump_dir) = self.direction.clone() {
      if &jump_dir == dir {
        self.count += 1;
        if self.count >= self.max_count {
          self.final_jump = true;
          self.reset();
        } else {
          self.final_jump = false;
        }
      } else {
        self.reset();
      }
    }
  }

  pub fn reset(&mut self) {
    self.count = 0;
    self.direction = None;
    self.rotation = 0.0;
  }

  pub fn direction(&self) -> &Option<WalkDirection> {
    &self.direction
  }

  pub fn strength(&self) -> NumType {
    self.count as NumType * self.strength
  }

  pub fn rotation(&self) -> f32 {
    self.rotation
  }

  pub fn update(&mut self, dt: f32) {
    let now = Instant::now();
    if self.final_jump {
      self.rotation = (self.rotation + (self.rotation_increase * dt)) % (2.0 * std::f32::consts::PI);
    }
    if now - self.last_update > self.wait {
      self.reset();
    }
    self.last_update = now;
  }
}

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
  has_jumped:       bool,
  id:               IdType,
  solid:            bool,
  triple_jump:      MultiJump,
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
      has_jumped:       false,
      id:               generate_id(),
      solid:            false,
      triple_jump:      MultiJump::new(TRIPLE_JUMP_STRENGTH),
      dt:               Deltatime::new(),
    }
  }

  pub fn keys_pressed(&mut self, keycodes: &Vec<Keycode>) {
    for keycode in keycodes {
      if let Some(point) = match keycode {
        &controls::LEFT => {
          if !self.has_moved(Axis::X) {
            self.moved_on_axis(Axis::X);
            Some(Point::new( -SPEED_INCREASE * self.dt.secs(), 0.0 ))
          } else { None }
        }
        &controls::RIGHT => {
          if !self.has_moved(Axis::X) {
            self.moved_on_axis(Axis::X);
            Some(Point::new( SPEED_INCREASE * self.dt.secs(), 0.0 ))
          } else { None }
        }
        &controls::JUMP => {
          if !self.has_jumped && self.on_floor() {
            self.jump();
          }
          None
        }
        _ => None
      } {
        self.add_velocity(&point);
      }
    }
  }

  pub fn key_down(&mut self, keycode: &Keycode) {
  }

  pub fn key_up(&mut self, keycode: &Keycode) {
    if let &controls::JUMP = keycode {
      self.has_jumped = false;
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
    self.has_jumped = true;
    self.is_jumping = true;
    let strength = -(JUMP_SPEED + self.triple_jump.strength());
    self.add_velocity(&Point::new(0.0, strength));
    self.triple_jump.jumped(&self.walk_direction);
  }

  pub fn stop_jumping(&mut self) {
    self.is_jumping = false;
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

  fn update_triple_jump(&mut self) {
    if !self.on_floor() {
      if let Some(triple_dir) = self.triple_jump.direction().clone() {
        if self.walk_direction != triple_dir {
          self.triple_jump.reset();
        }
      }
      self.triple_jump.update(self.dt.secs());
    }
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
    self.update_triple_jump();
    self.dt.update();
    Ok(())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    self.animations.get_by_state(&self.anim_state)
      .draw_rotate(ctx, &self.point, &self.size, &self.facing, self.triple_jump.rotation())
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    self.animations.get_by_state(&self.anim_state)
      .draw_offset_rotate(ctx, &self.point, &self.size, &self.facing, offset, self.triple_jump.rotation())
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
  fn gravity_increase(&self) -> Point {
    self.gravity_increase.mult_axes_by(self.dt.secs())
  }
}

impl Person for Player {
  fn reset_dt(&mut self) {
    self.dt.reset();
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
  fn stop_walking(&mut self) {}
}

impl IdGenerator for Player {
  fn id(&self) -> IdType {
    self.id
  }
  fn set_id(&mut self, id: IdType) {
    self.id = id;
  }
}
