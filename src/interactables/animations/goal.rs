use ggez::Context;
use animation::Animation;
use settings::res::MISSING_IMAGE;
use settings::interactables::IMAGES;

pub fn new_main_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 ::join_str(IMAGES, "goal0.png"),
                 ::join_str(IMAGES, "goal1.png"),
                 ::join_str(IMAGES, "goal2.png"),
                 ::join_str(IMAGES, "goal3.png"),
                 ], vec![
                 3000,
                 3000,
                 3000,
                 3000,
                 ])
}

pub fn new_trigger_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 MISSING_IMAGE.to_string(),
                 ], vec![
                 1000
                 ])
}
