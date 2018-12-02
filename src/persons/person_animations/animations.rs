pub mod player {
  use ggez::Context;
  use animation::Animation;
  use settings::player::*;
  use settings::child; //::*;

  pub fn new_idle_animation(ctx: &mut Context) -> Animation {
    let filepaths_idle: Vec<String> = vec![
      ::join_str(IMAGES, "player_idle_1.png"),
      ::join_str(IMAGES, "player_idle_2.png"),
    ];
    let update_intervals_ms_idle = vec![
      250,
      250,
    ];
    Animation::new(ctx, filepaths_idle, update_intervals_ms_idle)
  }

  pub fn new_walk_animation(ctx: &mut Context) -> Animation {
    let filepaths_idle: Vec<String> = vec![
      ::join_str(IMAGES, "player_idle_1.png"),
      ::join_str(IMAGES, "player_left_1.png"),
      ::join_str(IMAGES, "player_left_2.png"),
      ::join_str(IMAGES, "player_left_3.png"),
      ::join_str(IMAGES, "player_left_4.png"),
      ::join_str(IMAGES, "player_idle_1.png"),
      ::join_str(IMAGES, "player_right_1.png"),
      ::join_str(IMAGES, "player_right_2.png"),
      ::join_str(IMAGES, "player_right_3.png"),
      ::join_str(IMAGES, "player_right_4.png"),
    ];
    let update_intervals_ms_idle = vec![
      100,
      100,
      100,
      100,
      100,
      100,
      100,
      100,
      100,
      100,
    ];
    Animation::new(ctx, filepaths_idle, update_intervals_ms_idle)
  }

  pub fn new_jump_animation(ctx: &mut Context) -> Animation {
    let filepaths_jump: Vec<String> = vec![ ::join_str(child::IMAGES, "child_1_left_4.png") ];
    let interval_ms_jump = vec![ 250 ];
    Animation::new(ctx, filepaths_jump, interval_ms_jump)
  }

  pub fn new_fall_animation(ctx: &mut Context) -> Animation {
    let filepaths_fall: Vec<String> = vec![
      ::join_str(child::IMAGES, "child_1_falling_1.png"),
      //::join_str(IMAGES, "child_1_left_4.png"),
    ];
    let interval_ms_fall = vec![
      250,
      250
    ];
    Animation::new(ctx, filepaths_fall, interval_ms_fall)
  }
}

pub mod child {
  use ggez::Context;
  use animation::Animation;
  use settings::child::*;

  pub fn new_idle_animation(ctx: &mut Context) -> Animation {
    let filepaths_idle: Vec<String> = vec![
      ::join_str(IMAGES, "child_1_left_1.png"),
      ::join_str(IMAGES, "child_1_left_5.png")
    ];
    let update_intervals_ms_idle = vec![
      250,
      250
    ];
    Animation::new(ctx, filepaths_idle, update_intervals_ms_idle)
  }

  pub fn new_walk_animation(ctx: &mut Context) -> Animation {
    let filepaths_walk: Vec<String> = vec![
      ::join_str(IMAGES, "child_1_left_1.png"),
      ::join_str(IMAGES, "child_1_left_2.png"),
      ::join_str(IMAGES, "child_1_left_3.png"),
      ::join_str(IMAGES, "child_1_left_4.png"),
      ::join_str(IMAGES, "child_1_left_5.png"),
      ::join_str(IMAGES, "child_1_right_1.png"),
      ::join_str(IMAGES, "child_1_right_2.png"),
      ::join_str(IMAGES, "child_1_right_3.png"),
      ::join_str(IMAGES, "child_1_right_4.png"),
      ::join_str(IMAGES, "child_1_right_5.png")
    ];
    let update_intervals_ms_walk = vec![
      80,
      80,
      80,
      80,
      80,
      80,
      80,
      80,
      80,
      80
    ];
    Animation::new(ctx, filepaths_walk, update_intervals_ms_walk)
  }

  pub fn new_jump_animation(ctx: &mut Context) -> Animation {
    let filepaths: Vec<String> = vec![
      ::join_str(IMAGES, "child_1_left_4.png"),
      ::join_str(IMAGES, "child_1_right_4.png")
    ];
    let update_intervals_ms = vec![
      100,
      100
    ];
    Animation::new(ctx, filepaths, update_intervals_ms)
  }

  pub fn new_fall_animation(ctx: &mut Context) -> Animation {
    let filepaths: Vec<String> = vec![
      ::join_str(IMAGES, "child_1_falling_1.png")
    ];
    let update_intervals_ms = vec![
      100
    ];
    Animation::new(ctx, filepaths, update_intervals_ms)
  }
}
