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

  // PAUSE
  PauseResume,
  PauseToTitle,
  PauseReset,
}
