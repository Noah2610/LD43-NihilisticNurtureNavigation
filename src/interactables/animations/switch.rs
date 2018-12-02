use ggez::Context;
use animation::Animation;
use settings::interactables::*;
use settings::res::MISSING_IMAGE;

pub fn new_on_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 ::join_str(IMAGES, "switch_on.png")
                 ],
                 vec![
                 1000
                 ])
}

pub fn new_off_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 MISSING_IMAGE.to_string(),
                 ], vec![
                 1000
                 ])
}

pub fn new_turning_on_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 MISSING_IMAGE.to_string(),
                 ], vec![
                 1000
                 ])
}

pub fn new_turning_off_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 MISSING_IMAGE.to_string(),
                 ], vec![
                 1000
                 ])
}
