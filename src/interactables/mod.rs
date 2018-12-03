mod animations;

pub mod jump_pad;
pub mod switch;
pub mod door;

pub mod prelude {
  pub use super::Interactable;
  pub use super::jump_pad::{ JumpPad, self };
  pub use super::switch::{ Switch, self };
  pub use super::door::{ Door, self };
}

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;

use persons::Person;
use id_generator::IdType;

pub trait Interactable: Entity {
  fn get_intersected(&self) -> &Vec<IdType>;
  fn add_intersected(&mut self, id: IdType);
  fn rm_intersected_at(&mut self, index: usize);
  fn trigger<T: Person>(&mut self, person: &mut T);

  fn set_intersected<T: Person>(&mut self, person: &T, state: bool) {
    if state {
      if !self.is_intersected(person) {
        self.add_intersected(person.id());
      }
    } else {
      if let Some(index) = self.get_intersected().iter().position( |&id| person.has_id(id) ) {
        self.rm_intersected_at(index);
      }
    }
  }

  fn is_intersected<T: Person>(&self, person: &T) -> bool {
    self.get_intersected().iter()
      .any( |&p| person.has_id(p) )
  }

  fn trigger_once<T: Person>(&mut self, person: &mut T) {
    if self.is_intersected(person) { return; }
    self.set_intersected(person, true);
    self.trigger(person);
  }
}
