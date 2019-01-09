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
    write!(f, "{}", match self {
      ButtonType::TitleStart          => "TitleStart",
      ButtonType::TitleLevelSelect    => "TitleLevelSelect",
      ButtonType::TitleQuit           => "TitleQuit",

      ButtonType::LevelSelectBack     => "LevelSelectBack",
      ButtonType::LevelSelectLevel(n) => "LevelSelectLevel",

      ButtonType::NextLevel           => "NextLevel",
      ButtonType::LarryLeft           => "LarryLeft",
      ButtonType::LarryRight          => "LarryRight",
      ButtonType::ThingLeft           => "ThingLeft",
      ButtonType::ThingRight          => "ThingRight",
      ButtonType::BloatLeft           => "BloatLeft",
      ButtonType::BloatRight          => "BloatRight",
      ButtonType::IngamePause         => "IngamePause",

      ButtonType::PauseResume         => "PauseResume",
      ButtonType::PauseToTitle        => "PauseToTitle",
      ButtonType::PauseReset          => "PauseReset",

      ButtonType::StatsNext           => "StatsNext",
      ButtonType::StatsReset          => "StatsReset",
      ButtonType::StatsToTitle        => "StatsToTitle",
    })
  }
}
