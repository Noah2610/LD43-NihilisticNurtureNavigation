pub mod meta {
  use noframe::geo::size::Size;

  pub const NAME:         &str = "Nihilistic Nurture Navigation";
  pub const WINDOW_TITLE: &str = NAME;
  pub const AUTHORS:      &str = "noahro, hoichael, williwiderstand";
  pub const WINDOW_SIZE:  Size = Size { w: 1280.0, h: 720.0 };
}

pub mod game {
  use noframe::color::Color;
  pub const BG_COLOR: Color = [0.33, 0.33, 0.33, 1.0];
  pub const FPS: f32 = 60.0;
  pub const UPDATE_INTERVAL_MS: u64 = (1.0 / FPS * 1000.0) as u64;
}

pub mod res {
  pub const IMAGES:        &str = "/images/";
  pub const LEVELS:        &str = "resources/levels/";
  pub const AUDIO:         &str = "/audio/";
  pub const MISSING_IMAGE: &str = "/images/missing.png";
}

pub mod camera {
  pub const CAMERA_SPEED: f32 = 10.0;
}

pub mod player {
  pub const IMAGES: &str = "/images/player/";
  // pub const SPEED_INCREASE: f32 = 50.0;
  pub const SPEED_INCREASE: f32 = 50.0;
  pub const SPEED_DECREASE_X: f32 = 50.0;
  pub const SPEED_DECREASE_Y: f32 = 50.0;
  pub const MAX_VELOCITY_X: f32 = 150.0;
  pub const MAX_VELOCITY_Y: f32 = 1000.0;
  pub const JUMP_SPEED: f32 = 300.0;
  pub const JUMP_KILL_VELOCITY: f32 = 250.0;
  //pub const MAX_JUMP_SPEED: f32 = 200.0;
  pub const GRAVITY_INCREASE: f32 = 20.0;

  pub mod controls {
    use ggez::event::Keycode;
    pub const LEFT:  Keycode = Keycode::A;
    pub const RIGHT: Keycode = Keycode::D;
    pub const JUMP:  Keycode = Keycode::Space;
  }
}

pub mod child {
  pub const IMAGES: &str = "/images/children/";
  pub const SPEED_INCREASE: f32 = 10.0;
  pub const SPEED_DECREASE_X: f32 = 20.0;
  pub const SPEED_DECREASE_Y: f32 = 20.0;
  pub const MAX_VELOCITY_X: f32 = 80.0;
  pub const MAX_VELOCITY_Y: f32 = 1000.0;
  pub const GRAVITY_INCREASE: f32 = 20.0;
}

pub mod wall {
  pub const IMAGES: &str = "/images/walls/";
}

pub mod interactables {
  pub const IMAGES: &str = "/images/interactables/";

  pub mod jump_pad {
    pub const JUMP_SPEED: f32 = 1000.0;
  }
}

pub mod menus {
  pub const IMAGES: &str = "/images/menus/";
  pub mod title {
  }
}

pub mod buttons {
  pub const IMAGES: &str = "/images/buttons/";
  pub mod title {
  }
}
