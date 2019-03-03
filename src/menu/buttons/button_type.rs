use std::fmt;

#[derive(Clone, Debug)]
pub enum ButtonType {
  // TITLE
  TitleStart,
  TitleLevelSelect,
  TitleQuit,

  // TITLE/LEVEL_SELECT
  LevelSelectBack,
  LevelSelectLevel(usize),

  // TITLE/THANK_YOU
  ThankYouBack,

  // INGAME
  NextLevel,
  LarryLeft,
  LarryRight,
  ThingLeft,
  ThingRight,
  BloatLeft,
  BloatRight,
  IngamePause,

  // PAUSE
  PauseResume,
  PauseToTitle,
  PauseReset,

  // STATS
  StatsNext,
  StatsReset,
  StatsToTitle,
}

impl fmt::Display for ButtonType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::ButtonType::*;
    write!(f, "{}", match self {
      TitleStart          => String::from("TitleStart"),
      TitleLevelSelect    => String::from("TitleLevelSelect"),
      TitleQuit           => String::from("TitleQuit"),

      LevelSelectBack     => String::from("LevelSelectBack"),
      LevelSelectLevel(n) => format!("LevelSelectLevel #{}", n),

      ThankYouBack        => String::from("ThankYouBack"),

      NextLevel           => String::from("NextLevel"),
      LarryLeft           => String::from("LarryLeft"),
      LarryRight          => String::from("LarryRight"),
      ThingLeft           => String::from("ThingLeft"),
      ThingRight          => String::from("ThingRight"),
      BloatLeft           => String::from("BloatLeft"),
      BloatRight          => String::from("BloatRight"),
      IngamePause         => String::from("IngamePause"),

      PauseResume         => String::from("PauseResume"),
      PauseToTitle        => String::from("PauseToTitle"),
      PauseReset          => String::from("PauseReset"),

      StatsNext           => String::from("StatsNext"),
      StatsReset          => String::from("StatsReset"),
      StatsToTitle        => String::from("StatsToTitle"),
    })
  }
}
