#[derive(Clone, Debug)]
pub enum ButtonType {
  // TITLE
  TitleStart,
  TitleLevelSelect,
  TitleQuit,

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
