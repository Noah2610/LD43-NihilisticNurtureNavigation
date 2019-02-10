use ggez::Context;
use animation::Animation;
use settings::res::MISSING_IMAGE;
use settings::interactables::IMAGES;

pub fn new_zero_animation(ctx: &mut Context) -> Animation {
  Animation::new(
    ctx,
    vec![::join_str(IMAGES, "goal0.png")],
    vec![1000],
  )
}

pub fn new_one_animation(ctx: &mut Context) -> Animation {
  Animation::new(
    ctx,
    vec![::join_str(IMAGES, "goal1.png")],
    vec![1000],
  )
}

pub fn new_two_animation(ctx: &mut Context) -> Animation {
  Animation::new(
    ctx,
    vec![::join_str(IMAGES, "goal2.png")],
    vec![1000],
  )
}

pub fn new_three_animation(ctx: &mut Context) -> Animation {
  Animation::new(
    ctx,
    vec![::join_str(IMAGES, "goal3.png")],
    vec![1000],
  )
}

pub fn new_four_animation(ctx: &mut Context) -> Animation {
  Animation::new(
    ctx,
    vec![::join_str(IMAGES, "goal4.png")],
    vec![1000],
  )
}
