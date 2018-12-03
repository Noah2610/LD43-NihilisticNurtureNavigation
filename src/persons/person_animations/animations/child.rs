use ggez::Context;
use settings::child::*;
use animation::Animation;
use persons::children::ChildType;

pub fn new_idle_animation(ctx: &mut Context, child_type: &ChildType) -> Animation {
  match child_type {
    ChildType::Larry => Animation::new(
      ctx,
      vec![
      ::join_str(IMAGES, "child_1_left_1.png"),
      ::join_str(IMAGES, "child_1_left_5.png")
      ], vec![
      250,
      250
      ]),
    ChildType::Thing => Animation::new(
      ctx,
      vec![
      ::join_str(IMAGES, "child_2_1.png"),
      ], vec![
      1000
      ]),
    ChildType::Bloat => Animation::new(
      ctx,
      vec![
      ::join_str(IMAGES, "child_3_idle_1.png"),
      ::join_str(IMAGES, "child_3_walking_1.png")
      ], vec![
      250,
      250
      ])
  }
}

pub fn new_walk_animation(ctx: &mut Context, child_type: &ChildType) -> Animation {
  match child_type {
    ChildType::Larry => Animation::new(
      ctx,
      vec![
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
      ], vec![
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
      ]),
      ChildType::Thing => Animation::new(
        ctx,
        vec![
        ::join_str(IMAGES, "child_2_1.png"),
        ::join_str(IMAGES, "child_2_2.png"),
        ::join_str(IMAGES, "child_2_1.png"),
        ::join_str(IMAGES, "child_2_3.png")
        ], vec![
        100,
        100,
        100,
        100
        ]),
        ChildType::Bloat => Animation::new(
          ctx,
          vec![
          ::join_str(IMAGES, "child_3_walking_1.png"),
          ::join_str(IMAGES, "child_3_walking_2.png"),
          ::join_str(IMAGES, "child_3_walking_3.png"),
          ::join_str(IMAGES, "child_3_walking_4.png"),
          ::join_str(IMAGES, "child_3_walking_5.png"),
          ::join_str(IMAGES, "child_3_walking_6.png"),
          ::join_str(IMAGES, "child_3_walking_7.png")
          ], vec![
          50,
          50,
          50,
          50,
          50,
          50,
          50
          ])
  }
}

pub fn new_jump_animation(ctx: &mut Context, child_type: &ChildType) -> Animation {
  match child_type {
    ChildType::Larry => Animation::new(
      ctx,
      vec![
      ::join_str(IMAGES, "child_1_left_4.png"),
      ::join_str(IMAGES, "child_1_right_4.png")
      ], vec![
      100,
      100
      ]),
    ChildType::Thing => Animation::new(
      ctx,
      vec![
      ::join_str(IMAGES, "child_2_3.png"),
      ], vec![
      1000
      ]),
    ChildType::Bloat => Animation::new(
      ctx,
      vec![
      ::join_str(IMAGES, "child_3_jump_1.png")
      ], vec![
      1000
      ])
  }
}

pub fn new_fall_animation(ctx: &mut Context, child_type: &ChildType) -> Animation {
  match child_type {
    ChildType::Larry => Animation::new(
      ctx,
      vec![
      ::join_str(IMAGES, "child_1_falling_1.png")
      ], vec![
      100
      ]),
    ChildType::Thing => Animation::new(
      ctx,
      vec![
      ::join_str(IMAGES, "child_2_falling_1.png"),
      ::join_str(IMAGES, "child_2_falling_2.png")
      ], vec![
      100,
      100
      ]),
    ChildType::Bloat => Animation::new(
      ctx,
      vec![
      ::join_str(IMAGES, "child_3_falling_1.png")
      ], vec![
      1000
      ])
  }
}
