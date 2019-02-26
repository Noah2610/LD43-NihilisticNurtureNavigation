pub mod player;
pub mod children;
mod person_animations;

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;
use noframe::deltatime::Deltatime;

use gravity::Gravity;
use id_generator::IdGenerator;

pub enum AnimState {
  Walk,
  Idle,
  Jump,
  Fall
}

#[derive(PartialEq)]
pub enum Axis {
  X,
  Y
}

#[derive(PartialEq)]
enum WalkDirection {
  Still,
  Left,
  Right
}

pub trait Person: Entity + Velocity + Gravity + IdGenerator {
  fn reset_dt(&mut self, dt: &Deltatime);
  fn is_solid(&self) -> bool;
  fn solidify(&mut self);
  fn unsolidify(&mut self);
  fn on_jump_pad(&mut self);
  fn moved_axes(&self) -> &Vec<Axis>;
  fn add_moved_axis(&mut self, axis: Axis);
  fn clear_moved_axes(&mut self);
  fn speed_decrease(&self) -> Point;

  // NOTE: This is a very bad method...
  fn on_floor(&self) -> bool {
    let range = 0.0 .. self.gravity_increase().y * 2.0;
    let vel_y = self.velocity().y;
    vel_y >= range.start && vel_y <= range.end  // Inclusive end
  }

  fn moved_on_axis(&mut self, axis: Axis) {
    if !self.moved_axes().iter().any( |a| &axis == a ) {
      self.add_moved_axis(axis);
    }
  }

  fn has_moved(&self, axis: Axis) -> bool {
    self.moved_axes().iter().any( |a| &axis == a )
  }

  fn handle_decrease_velocity(&mut self) {
    let decr_vel = Point::new(
      if !self.has_moved(Axis::X) && self.on_floor() {
        self.speed_decrease().x
      } else { 0.0 },
      if false && !self.has_moved(Axis::Y) {  // TODO I don't think we need to decrease y velocity automatically
        self.speed_decrease().y
      } else { 0.0 }
    );
    self.decrease_velocity(&decr_vel);
    self.clear_moved_axes();
  }
}
