use ggez::{
  GameResult,
  Context,
  event::Keycode
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;
use noframe::deltatime::Deltatime;

use animation::Animation;
use super::AnimState;
use settings::child::*;

pub struct Child {
  point:        Point,
  size:         Size,
  origin:       Origin,
  velocity:     Point,
  max_velocity: Point,
  idle_anim:    Animation,
  walk_anim:    Animation,
  anim_state:   AnimState,
  dt:           Deltatime
}

impl Child {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    let image_filepaths_idle: Vec<String> = vec![
      ::join_str(IMAGES, "child_1_left_1.png"),
      ::join_str(IMAGES, "child_1_left_5.png")
    ];
    let image_update_intervals_ms_idle = vec![
      400,
      400
    ];
    let image_filepaths_walk: Vec<String> = vec![
      ::join_str(IMAGES, "child_1_left_1.png"),
      ::join_str(IMAGES, "child_1_left_2.png"),
      ::join_str(IMAGES, "child_1_left_3.png"),
      ::join_str(IMAGES, "child_1_left_4.png"),
      ::join_str(IMAGES, "child_1_left_5.png"),
      ::join_str(IMAGES, "child_1_right_1.png"),
      ::join_str(IMAGES, "child_1_right_2.png"),
      ::join_str(IMAGES, "child_1_right_3.png"),
      ::join_str(IMAGES, "child_1_right_4.png"),
      ::join_str(IMAGES, "child_1_right_5.png")
    ];
    let image_update_intervals_ms_walk = vec![
      80,
      80,
      80,
      80,
      80,
      80,
      80,
      80,
      80,
      80
    ];

    let idle_anim = Animation::new(ctx, image_filepaths_idle, image_update_intervals_ms_idle);
    let walk_anim = Animation::new(ctx, image_filepaths_walk, image_update_intervals_ms_walk);

    Self {
      point,
      size,
      origin: Origin::TopLeft,
      velocity: Point::new(0.0, 0.0),
      max_velocity: Point::new(MAX_SPEED, MAX_JUMP_SPEED),
      idle_anim,
      walk_anim,
      anim_state: AnimState::Idle,
      dt: Deltatime::new()
    }
  }
}

impl Mask for Child {
  fn point(&self) -> &Point {
    &self.point
  }
  fn point_mut(&mut self) -> &mut Point {
    &mut self.point
  }
  fn size(&self) -> &Size {
    &self.size
  }
  fn origin(&self) -> &Origin {
    &self.origin
  }
}

impl Entity for Child {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.walk_anim.update()?;
    self.dt.update();
    Ok(())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    self.walk_anim.draw(ctx, &self.point, &self.size)
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    self.walk_anim.draw_offset(ctx, &self.point, &self.size, offset)
  }
}

impl Velocity for Child {
  fn velocity(&self) -> &Point {
    &self.velocity
  }
  fn usable_velocity(&self) -> Point {
    self.velocity.mult_axes_by(self.dt.secs())
  }
  fn velocity_mut(&mut self) -> &mut Point {
    &mut self.velocity
  }
  fn max_velocity(&self) -> Point {
    self.max_velocity.clone()
  }
}

impl Movement for Child { }
