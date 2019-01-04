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
  pub const UPS: f32 = 60.0;
  pub const UPDATE_INTERVAL_MS: u64 = (1.0 / UPS * 1000.0) as u64;
}

pub mod res {
  pub const IMAGES:            &str = "/images/";
  pub const BACKGROUND_IMAGES: &str = "/images/backgrounds/";
  pub const LEVELS:            &str = "resources/levels/";
  pub const AUDIO:             &str = "/audio/";
  pub const MISSING_IMAGE:     &str = "/images/missing.png";
  pub const FONTS:             &str = "/fonts/";
}

pub mod camera {
  pub const CAMERA_SPEED: f32 = 500.0;
}

pub mod player {
  use score::ScoreType;
  pub const IMAGES: &str = "/images/player/";
  pub const SPEED_INCREASE: f32 = 600.0;
  pub const SPEED_DECREASE_X: f32 = 600.0;
  pub const SPEED_DECREASE_Y: f32 = 600.0;
  pub const MAX_VELOCITY_X: f32 = 150.0;
  pub const MAX_VELOCITY_Y: f32 = 1000.0;
  pub const JUMP_SPEED: f32 = 325.0;
  pub const JUMP_KILL_VELOCITY: f32 = 250.0;
  pub const GRAVITY_INCREASE: f32 = 800.0;

  pub mod controls {
    use ggez::event::Keycode;
    pub const LEFT:  Keycode = Keycode::A;
    pub const RIGHT: Keycode = Keycode::D;
    pub const JUMP:  Keycode = Keycode::Space;
  }
}

pub mod child {
  use score::ScoreType;
  pub const IMAGES: &str = "/images/children/";
  pub const SPEED_INCREASE: f32 = 400.0;
  pub const SPEED_DECREASE_X: f32 = 600.0;
  pub const SPEED_DECREASE_Y: f32 = 600.0;
  pub const MAX_VELOCITY_X: f32 = 80.0;
  pub const MAX_VELOCITY_Y: f32 = 1000.0;
  pub const GRAVITY_INCREASE: f32 = 800.0;
}

pub mod wall {
  pub const IMAGES: &str = "/images/walls/";
}

pub mod interactables {
  pub const IMAGES: &str = "/images/interactables/";

  pub mod jump_pad {
    pub const JUMP_SPEED: f32 = 600.0;
    // 0.5  => starting (and ending) in center
    // 0.25 => starting 1/4 into it from the left and ending 1/4 before the end
    pub const HITBOX_PERCENT: f32 = 0.4;
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

pub mod fonts {
  pub const TO_SAVE_FONT_SIZE: u32 = 16;
}

pub mod level {
  use ggez::event::Keycode;
  pub const FONT_SIZE: u32 = 16;
  pub const CENTER_KEY: Keycode = Keycode::C;
  pub const SKIP_KEY: Keycode = Keycode::N;  // TODO: Temporary! Only for debugging!
}

pub mod level_manager {
  pub const AUDIO_FORMAT: &str = "ogg";
  pub const LEVEL_NAMES: [&'static str; 3] = ["jump_pad_one_way", "test_one", "test_two"];
  pub const SONG_NAMES:  [&'static str; 3] = ["ingame_1", "ingame_1", "ingame_2"];

  pub mod controls {
    use ggez::event::Keycode;
    pub const MUTE:  Keycode = Keycode::M;
    pub const PAUSE: Keycode = Keycode::P;
    pub const RESET: Keycode = Keycode::R;
  }
}

pub mod score {
  use score::ScoreType;
  pub const PLAYER_SCORE_REWARD: ScoreType = 100;
  pub const CHILD_SCORE_REWARD:  ScoreType = 300;
}
