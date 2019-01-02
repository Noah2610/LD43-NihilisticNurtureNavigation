use ggez::Context;
use noframe::geo::prelude::*;

use settings::res::*;
use super::super::prelude::*;

pub fn new_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx, vec![
                 MISSING_IMAGE.to_string()
  ], vec![
  1000
  ])
}

pub fn new_buttons(ctx: &mut Context, window_size: &Size) -> Vec<Button> {
  vec![
    Button::new_with_origin(
      ctx,
      window_size.center(),
      Size::new(128.0, 64.0),
      Origin::Center,
      ButtonType::PauseResume,
      vec![
      MISSING_IMAGE.to_string()
      ],
      vec![
      1000
      ]
      ),

      Button::new_with_origin(
        ctx,
        Point::combine(vec![&window_size.center(), &Point::new(0.0, 80.0)]),
        Size::new(128.0, 64.0),
        Origin::Center,
        ButtonType::PauseToTitle,
        vec![
        MISSING_IMAGE.to_string()
        ],
        vec![
        1000
        ]
        )
        ]
}
