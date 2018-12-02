pub mod jump_pad {
  use ggez::Context;
  use animation::Animation;
  use settings::interactables::*;

  pub fn new_main_animation(ctx: &mut Context) -> Animation {
    Animation::new(ctx,
                   vec![
                   ::join_str(IMAGES, "jump_pad.png"),
                   ], vec![
                   100
                   ])
  }

  pub fn new_trigger_animation(ctx: &mut Context) -> Animation {
    Animation::new(ctx, vec![], vec![])
  }
}
