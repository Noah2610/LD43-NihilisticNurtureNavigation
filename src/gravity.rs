use noframe::geo::prelude::*;
use noframe::entity::prelude::*;

pub trait Gravity: Entity + Velocity {
  fn gravity_increase(&self) -> Point;

  fn update_gravity(&mut self) {
    let grav_incr = self.gravity_increase().clone();
    self.add_velocity(&grav_incr);
  }
}
