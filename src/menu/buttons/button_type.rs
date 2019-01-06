#[derive(Clone, Debug)]
pub enum ButtonType {
  // TITLE
  Start,

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
