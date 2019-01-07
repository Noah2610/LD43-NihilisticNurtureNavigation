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
