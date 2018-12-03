use ggez::Context;
use animation::Animation;
use settings::interactables::*;
use settings::res::MISSING_IMAGE;

pub fn new_open_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 MISSING_IMAGE.to_string(),
                 ],
                 vec![
                 1000
                 ])
}

pub fn new_closed_animation(ctx: &mut Context) -> Animation {
  new_open_animation(ctx)
}

pub fn new_opening_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 MISSING_IMAGE.to_string(),
                 ], vec![
                 1000
                 ])
}

pub fn new_closing_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 MISSING_IMAGE.to_string(),
                 ], vec![
                 1000
                 ])
}
