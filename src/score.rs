use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

use json::JsonValue;

use settings::score::*;
use settings::player;
use persons::children::ChildType;

pub type ScoreType = u32;

pub mod prelude {
  pub use super::Score;
  pub use super::ScoreType;
}

#[derive(Debug, Clone)]
pub struct Score {
  times_saved_player:   ScoreType,
  times_saved_children: HashMap<ChildType, ScoreType>,
}

impl Score {
  pub fn new() -> Self {
    Self {
      times_saved_player:   0,
      times_saved_children: HashMap::new(),
    }
  }

  pub fn from_json(json: &JsonValue) -> Option<Self> {
    if !json.is_object() { return None; }
    Some(Self {
      times_saved_player: json["player"].as_u32().unwrap_or(0),  // as ScoreType
      times_saved_children: json["children"].entries()
        .filter_map( |(name, times)| if let Some(child) = ChildType::from_short(name) {
          Some((child, times.as_u32().unwrap_or(0)))  // as ScoreType
        } else { None })
        .collect(),
    })
  }

  pub fn as_json(&self) -> Option<JsonValue> {
    if self.score() == 0 { return None; }
    let mut data = object!{
      "children" => object!{},
    };
    if self.times_saved_player > 0 {
      data["player"] = self.times_saved_player.into();
    }
    for (child, &times) in &self.times_saved_children {
      if times > 0 {
        data["children"][child.short()] = times.into();
      }
    }
    Some(data)
  }

  pub fn score(&self) -> ScoreType {
    (self.times_saved_player * PLAYER_SCORE_REWARD) +
      (self.times_saved_children.values().sum::<ScoreType>() * CHILD_SCORE_REWARD)
  }

  pub fn semantic_score(&self) -> String {
    format!("Score: {}", self.score())
  }

  pub fn semantic_highscore(&self) -> String {
    format!("Highscore: {}", self.score())
  }

  pub fn semantic_player(&self) -> Option<String> {
    if self.times_saved_player > 0 {
      Some(format!(
          "{}: {}",
          player::NAME,
          self.semantic_score_for(self.times_saved_player, PLAYER_SCORE_REWARD)
      ))
    } else { None }
  }

  pub fn semantic_children(&self) -> Vec<String> {
    self.times_saved_children().iter()
      .map( |(child, &n)| format!("{}: {}", child.name(), self.semantic_score_for(n, CHILD_SCORE_REWARD)) )
      .collect()
  }

  pub fn any(&self) -> bool {
    self.score() > 0
  }

  fn semantic_score_for(&self, times_saved: ScoreType, score_reward: ScoreType) -> String {
    let mut score = format!("{}", times_saved * score_reward);
    let score_len = score.len() as u8;
    if score_len < SCORE_CHAR_LEN {
      for _i in 0 .. SCORE_CHAR_LEN - score_len {
        score.insert(0, '0');
      }
    }
    if times_saved > 1 {
      format!("{} x {} = {}", times_saved, score_reward, score)
    } else {
      format!("+{}", score)
    }
  }

  pub fn times_saved_player(&self) -> ScoreType {
    self.times_saved_player
  }

  pub fn saved_player(&mut self) {
    self.times_saved_player += 1;
  }

  pub fn times_saved_children(&self) -> &HashMap<ChildType, ScoreType> {
    &self.times_saved_children
  }

  pub fn saved_child(&mut self, child_type: ChildType) {
    *self.times_saved_children.entry(child_type).or_insert(0) += 1;
  }

  pub fn clear(&mut self) {
    self.times_saved_player = 0;
    self.times_saved_children.clear();
  }
}

impl From<Vec<&Score>> for Score {
  fn from(scores: Vec<&Score>) -> Self {
    let mut score_acc = Score::new();
    for score in scores {
      (0 .. score.times_saved_player())
        .for_each( |_i| score_acc.saved_player() );
      for (child_type, saved) in score.times_saved_children() {
        (0 .. *saved)
          .for_each( |_i| score_acc.saved_child(child_type.clone()) )
      }
    }
    score_acc
  }
}

impl PartialEq for Score {
  fn eq(&self, other: &Score) -> bool {
    self.score() == other.score()
  }
}

impl PartialOrd for Score {
  fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
    Some(self.score().cmp(&other.score()))
  }
}

impl fmt::Display for Score {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.score())
  }
}
