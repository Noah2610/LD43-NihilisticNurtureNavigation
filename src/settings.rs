pub mod meta {
  use noframe::geo::size::Size;

  pub const NAME:         &str = "Nihilistic Nurture Navigation";
  pub const WINDOW_TITLE: &str = NAME;
  pub const AUTHORS:      &str = "noahro, hoichael, williwiderstand";
  pub const WINDOW_SIZE:  Size = Size { w: 960.0, h: 540.0 };
}

pub mod game {
  use noframe::color::Color;
  pub const BG_COLOR: Color = [0.33, 0.33, 0.33, 1.0];
  pub const FPS: f32 = 60.0;
  pub const UPDATE_INTERVAL_MS: u64 = (1.0 / FPS * 1000.0) as u64;
}

pub mod controls {
  use ggez::event::Keycode;
  pub const LEFT:  Keycode = Keycode::A;
  pub const RIGHT: Keycode = Keycode::D;
  pub const JUMP:  Keycode = Keycode::W;
}
