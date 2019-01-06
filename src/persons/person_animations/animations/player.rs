use ggez::Context;
use animation::Animation;
use settings::player::*;

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
  let filepaths_jump: Vec<String> = vec![
    ::join_str(IMAGES, "player_falling_2.png")
  ];
  let interval_ms_jump = vec![ 250 ];
  Animation::new(ctx, filepaths_jump, interval_ms_jump)
}

pub fn new_fall_animation(ctx: &mut Context) -> Animation {
  let filepaths_fall: Vec<String> = vec![
    ::join_str(IMAGES, "player_falling_1.png"),
    ::join_str(IMAGES, "player_falling_2.png"),
    // ::join_str(IMAGES, "player_falling_1.png"),
    // ::join_str(IMAGES, "player_falling_3.png"),
  ];
  let interval_ms_fall = vec![
    200,
    200,
    // 200,
    // 200
  ];
  Animation::new(ctx, filepaths_fall, interval_ms_fall)
}
