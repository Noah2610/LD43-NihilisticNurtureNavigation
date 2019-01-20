use ggez::Context;
use animation::Animation;
use settings::interactables::*;

pub fn new_main_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 ::join_str(IMAGES, "oneway_1.png")
                 ], vec![
                 1000
                 ])
}
