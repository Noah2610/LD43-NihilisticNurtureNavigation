pub mod meta {
  use noframe::geo::size::Size;
  pub const NAME:         &str = "Nihilistic Nurture Navigation";
  pub const WINDOW_TITLE: &str = NAME;
  pub const AUTHORS:      &str = "noahro, hoichael, williwiderstand";
  // pub const WINDOW_SIZE:  Size = Size { w: 1280.0, h: 720.0 };
  pub const WINDOW_SIZE:  Size = Size { w: 1542.0, h: 900.0 };
}

pub mod game {
  use noframe::color::Color;
  pub const BG_COLOR: Color = [0.33, 0.33, 0.33, 1.0];
  pub const UPS: f32 = 60.0;
  pub const UPDATE_INTERVAL_MS: u64 = (1.0 / UPS * 1000.0) as u64;
  pub const MUTED: bool = false;
  pub const VOLUME: f32 = 0.5;
  pub const SAVEFILE: &str = "save.json";
}

pub mod res {
  pub const IMAGES:            &str = "/images/";
  pub const BACKGROUND_IMAGES: &str = "/images/backgrounds/";
  pub const LEVELS:            &str = "resources/levels/";
  pub const AUDIO:             &str = "/audio/";
  pub const MISSING_IMAGE:     &str = "/images/missing.png";
  pub const FONTS:             &str = "/fonts/";
  pub mod fonts {
    pub const DEFAULT: &str = "/fonts/vcr_osd_mono.ttf";
  }
}

pub mod camera {
  pub const CAMERA_SPEED: f32 = 500.0;
}

pub mod player {
  pub const NAME: &str = "The Dude";
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
  pub const IMAGES: &str = "/images/children/";
  pub const SPEED_INCREASE: f32 = 400.0;
  pub const SPEED_DECREASE_X: f32 = 600.0;
  pub const SPEED_DECREASE_Y: f32 = 600.0;
  pub const MAX_VELOCITY_X: f32 = 80.0;
  pub const MAX_VELOCITY_Y: f32 = 1000.0;
  pub const GRAVITY_INCREASE: f32 = 800.0;

  pub mod names {
    pub const LARRY: &str = "Larry";
    pub const BLOAT: &str = "Bloat";
    pub const THING: &str = "The Thing";
  }

  pub mod shorts {
    pub const LARRY: &str = "larry";
    pub const BLOAT: &str = "bloat";
    pub const THING: &str = "thing";
  }
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
    pub const HITBOX_PERCENT:  f32 = 0.4;
    pub const X_VELOCITY_MULT: f32 = 0.6;
  }
}

pub mod menus {
  pub const IMAGES: &str = "/images/menus/";
  pub mod title {
    pub const TOTAL_SCORE_FONT_SIZE: u32 = 12;
    pub mod level_select {
      pub const FONT_SIZE: u32 = 24;
    }
    pub mod controls {
      use ggez::event::Keycode;
      pub const PLAY:         Keycode = Keycode::Return;
      pub const LEVEL_SELECT: Keycode = Keycode::L;
      pub const BACK:         Keycode = Keycode::Backspace;
    }
  }
  pub mod stats {
    use noframe::color::{ self, Color };
    pub const FONT_SIZE_SCORE: u32 = 32;
    pub const FONT_SIZE_SAVED: u32 = 24;
    pub const FONT_COLOR: Color = color::BLACK;
  }
  pub mod pause {
    use noframe::color::Color;
    pub const TITLE_FONT_SIZE:  u32   = 32;
    pub const TITLE_BG_COLOR:   Color = [0.5, 0.5, 0.5, 1.0];
    pub const TITLE_FONT_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
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
  pub const NAME_FONT_SIZE: u32 = 12;
  pub const CENTER_KEY: Keycode = Keycode::C;
  pub const SKIP_KEY: Keycode = Keycode::N;  // TODO: Temporary! Only for debugging!
}

pub mod level_manager {
  pub const AUDIO_FORMAT: &str = "ogg";
  pub const LEVEL_NAMES: [&'static str; 15] = [
    "01_one",
    "02_two",
    "03_three",
    "04_four",
    "05_five",
    "06_six",
    "07_seven",
    "08_eight",
    "09_nine",
    "10_ten",
    "11_eleven",
    "12_twelve",
    "13_thirteen",
    "14_fourteen",
    "15_fifteen",
  ];
  pub const SONG_NAMES:  [&'static str; 15] = [
    "ingame_1.1.fl",
    "ingame_1.1.fl",
    "ingame_1.1.fl",
    "ingame_1.2.fl",
    "ingame_1.2.fl",

    "ingame_2.5.fl",
    "ingame_2.5.fl",
    "ingame_2.5.fl",
    "ingame_2.fl",
    "ingame_2.fl",

    "ingame_3",  // TODO: ingame_3.fl.ogg is missing
    "ingame_3",
    "ingame_3",
    "ingame_4.fl",
    "ingame_4.fl",
  ];

  pub const HIGHSCORE_FONT_SIZE: u32 = 12;

  pub mod controls {
    use ggez::event::Keycode;
    pub const MUTE:     Keycode = Keycode::M;
    pub const PAUSE:    Keycode = Keycode::P;
    pub const RESET:    Keycode = Keycode::R;
    pub const NEXT:     Keycode = Keycode::Return;
    pub const TO_TITLE: Keycode = Keycode::Backspace;
  }
}

pub mod score {
  use noframe::color::Color;
  use score::ScoreType;
  pub const PLAYER_SCORE_REWARD: ScoreType = 10;
  pub const CHILD_SCORE_REWARD:  ScoreType = 30;
  pub const SCORE_CHAR_LEN:      u8        = 2;
  pub const SCORE_COLOR:         Color     = [0.8, 0.1, 0.1, 1.0];
  pub const HIGHSCORE_COLOR:     Color     = [0.7, 0.2, 0.1, 1.0];
  pub const NEW_HIGHSCORE_COLOR: Color     = [0.1, 0.5, 0.1, 1.0];
}

pub mod color_rect {
  use noframe::color::*;
  pub const DEFAULT_COLOR: Color = BLACK;
}
