mod animations;

use ggez::{
  GameResult,
  Context,
};

use animation::Animation;
use super::AnimState;
use self::animations::{ player, child };
use settings::player::*;

pub struct PersonAnimations {
  idle_anim:    Animation,
  walk_anim:    Animation,
  jump_anim:    Animation,
  fall_anim:    Animation,
}

impl PersonAnimations {
  pub fn new_player_animations(ctx: &mut Context) -> Self {
    Self {
      idle_anim: player::new_idle_animation(ctx),
      walk_anim: player::new_walk_animation(ctx),
      jump_anim: player::new_jump_animation(ctx),
      fall_anim: player::new_fall_animation(ctx)
    }
  }

  pub fn new_child_animations(ctx: &mut Context) -> Self {
    Self {
      idle_anim:    child::new_idle_animation(ctx),
      walk_anim:    child::new_walk_animation(ctx),
      jump_anim:    child::new_jump_animation(ctx),
      fall_anim:    child::new_fall_animation(ctx)
    }
  }

  pub fn get_by_state(&self, state: &AnimState) -> &Animation {
    match state {
      AnimState::Idle => &self.idle_anim,
      AnimState::Walk => &self.walk_anim,
      AnimState::Jump => &self.jump_anim,
      AnimState::Fall => &self.fall_anim
    }
  }

  pub fn get_by_state_mut(&mut self, state: &AnimState) -> &mut Animation {
    match state {
      AnimState::Idle => &mut self.idle_anim,
      AnimState::Walk => &mut self.walk_anim,
      AnimState::Jump => &mut self.jump_anim,
      AnimState::Fall => &mut self.fall_anim
    }
  }
}
