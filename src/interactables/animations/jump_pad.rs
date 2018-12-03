use ggez::Context;
use animation::Animation;
use settings::interactables::*;
use settings::res::MISSING_IMAGE;

pub fn new_main_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 ::join_str(IMAGES, "jump_pad.png"),
                 ], vec![
                 100
                 ])
}

pub fn new_trigger_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 MISSING_IMAGE.to_string(),
                 ], vec![
                 2000
                 ])
}
