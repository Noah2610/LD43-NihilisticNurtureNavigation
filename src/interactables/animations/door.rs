use ggez::Context;
use animation::Animation;
use settings::interactables::*;
use settings::res::MISSING_IMAGE;

pub fn new_open_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 format!("{}{}{}.png", IMAGES, "door_open_", color)
                 ],
                 vec![
                 1000
                 ])
}

pub fn new_closed_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 format!("{}{}{}.png", IMAGES, "door_closed_", color)
                 ],
                 vec![
                 1000
                 ])
}

pub fn new_opening_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 format!("{}{}{}.png", IMAGES, "door_3_", color),
                 format!("{}{}{}.png", IMAGES, "door_3_", color),
                 format!("{}{}{}.png", IMAGES, "door_2_", color),
                 format!("{}{}{}.png", IMAGES, "door_1_", color),
                 ], vec![
                 25,
                 25,
                 25,
                 25
                 ])
}

pub fn new_closing_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 format!("{}{}{}.png", IMAGES, "door_1_", color),
                 format!("{}{}{}.png", IMAGES, "door_1_", color),
                 format!("{}{}{}.png", IMAGES, "door_3_", color),
                 format!("{}{}{}.png", IMAGES, "door_2_", color),
                 ], vec![
                 25,
                 25,
                 25,
                 25
                 ])
}
