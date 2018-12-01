use ggez::{
  GameResult,
  Context,
};

use animation::Animation;
use super::super::AnimState;
use settings::player::*;

pub struct PlayerAnimations {
  idle_anim:    Animation,
  walk_anim:    Animation,
  jump_anim:    Animation,
  fall_anim:    Animation,
}

impl PlayerAnimations {
  pub fn new(ctx: &mut Context) -> Self {
    let img_filepaths_idle: Vec<String> = vec![ ::join_str(IMAGES, "child1_1.png") ];
    let img_interval_ms_idle = vec![ 250 ];
    let img_filepaths_walk: Vec<String> = vec![
      ::join_str(IMAGES, "child1_1.png"),
      ::join_str(IMAGES, "child1_2.png"),
      ::join_str(IMAGES, "child1_3.png"),
      ::join_str(IMAGES, "child1_4.png")
    ];
    let img_interval_ms_walk = vec![
      250,
      250,
      250,
      250
    ];
    let img_filepaths_jump: Vec<String> = vec![ ::join_str(IMAGES, "child1_4.png") ];
    let img_interval_ms_jump = vec![ 250 ];
    let img_filepaths_fall: Vec<String> = vec![ ::join_str(IMAGES, "child1_3.png") ];
    let img_interval_ms_fall = vec![ 250 ];

    let idle_anim = Animation::new(ctx, img_filepaths_idle, img_interval_ms_idle);
    let walk_anim = Animation::new(ctx, img_filepaths_walk, img_interval_ms_walk);
    let jump_anim = Animation::new(ctx, img_filepaths_jump, img_interval_ms_jump);
    let fall_anim = Animation::new(ctx, img_filepaths_fall, img_interval_ms_fall);

    Self {
      idle_anim:    idle_anim,
      walk_anim:    walk_anim,
      jump_anim:    jump_anim,
      fall_anim:    fall_anim,
    }
  }

  pub fn handle_state(&mut self, state: &AnimState) -> GameResult<()> {
    match state {
      AnimState::Idle => &mut self.idle_anim,
      AnimState::Walk => &mut self.walk_anim,
      AnimState::Jump => &mut self.jump_anim,
      AnimState::Fall => &mut self.fall_anim
    } .update()
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
