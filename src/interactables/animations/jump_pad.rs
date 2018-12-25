use ggez::Context;
use animation::Animation;
use settings::interactables::*;
use settings::res::MISSING_IMAGE;

pub fn new_active_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 format!("{}jumppad_active_{}.png", IMAGES, color),
                 ], vec![
                 1000
                 ])
}

pub fn new_inactive_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 format!("{}jumppad_inactive_{}.png", IMAGES, color),
                 ], vec![
                 1000
                 ])
}

pub fn new_trigger_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 MISSING_IMAGE.to_string(),
                 ], vec![
                 500
                 ])
}
