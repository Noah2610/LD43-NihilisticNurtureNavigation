mod helpers;

use std::collections::HashMap;

use json::JsonValue;

use settings::score::*;
use settings::player;
use persons::children::ChildType;
use self::helpers::*;

pub type ScoreType = u32;

pub mod prelude {
  pub use super::Score;
  pub use super::ScoreType;
}

#[derive(Debug, Clone)]
pub struct Score {
  times_saved_player:   ScoreType,
  times_saved_children: HashMap<ChildType, ScoreType>,
  moves_counter:        ChildMovesCounter,
}

impl Score {
  pub fn new() -> Self {
    Self {
      times_saved_player:   0,
      times_saved_children: HashMap::new(),
      moves_counter:        ChildMovesCounter::new(),
    }
  }

  pub fn from_json(json: &JsonValue) -> Option<Self> {
    if !json.is_object() { return None; }

    let mut times_saved_children = HashMap::new();
    let mut times_moved_children = HashMap::new();

    for (name, data) in json["children"].entries() {
      if let Some(child) = ChildType::from_short(name) {
        if data.has_key("saved") {     //                  vvvvvvvv         as ScoreType
          times_saved_children.insert(child, data["saved"].as_u32().unwrap_or(0));
        }
        if data.has_key("moves") {  //                         vvvvvvvv  as ScoreType
          times_moved_children.insert(child, data["moves"].as_u32().unwrap_or(0));
        }
      }
    }

    Some(Self {
      times_saved_player: json["player"]["saved"].as_u32().unwrap_or(0),  // as ScoreType
      times_saved_children,
      moves_counter: ChildMovesCounter::with(times_moved_children),
    })
  }

  pub fn as_json(&self) -> Option<JsonValue> {
    if self.score() == 0 { return None; }
    let mut data = object!{
      "children" => object!{},
    };
    if self.times_saved_player > 0 {
      data["player"] = object!{
        "saved" => self.times_saved_player,
      };
    }
    for (child, &saved) in &self.times_saved_children {
      if saved > 0 {
        if !data["children"][child.short()].is_object() {
          data["children"][child.short()] = object!{};
        }
        data["children"][child.short()]["saved"] = saved.into();
      }
    }
    for (child, &moves) in self.moves_counter.moves() {
      if moves > 0 {
        if !data["children"][child.short()].is_object() {
          data["children"][child.short()] = object!{};
        }
        data["children"][child.short()]["moves"] = moves.into();
      }
    }
    Some(data)
  }

  pub fn score(&self) -> ScoreType {
    let saved = (self.times_saved_player * PLAYER_SCORE_REWARD) +
      (self.times_saved_children.values().sum::<ScoreType>() * CHILD_SCORE_REWARD);
    let moves = self.moves_counter.total();
    if saved >= moves {
      saved - moves
    } else { 0 }
  }

  pub fn semantic_score(&self) -> String {
    format!("Score: {}", self)
  }

  pub fn semantic_highscore(&self) -> String {
    format!("Highscore: {}", self)
  }

  pub fn semantic_player(&self) -> Option<String> {
    if self.times_saved_player > 0 {
      Some(format!(
          "{}: {}",
          player::NAME,
          self.semantic_score_for(self.times_saved_player, PLAYER_SCORE_REWARD, None)
      ))
    } else { None }
  }

  pub fn semantic_children(&self) -> Vec<String> {
    self.times_saved_children().iter()
      .map( |(&child, &n)| {
        format!(
          "{}: {}", child.name(),
          self.semantic_score_for(n, CHILD_SCORE_REWARD, self.moves_counter.moves_for(child))
        )
      }).collect()
  }

  pub fn any(&self) -> bool {
    self.score() > 0
  }

  fn semantic_score_for(&self, times_saved: ScoreType, score_reward: ScoreType, moves_given: Option<ScoreType>) -> String {
    let mut score = format!("{}", times_saved * score_reward - moves_given.unwrap_or(0));
    let score_len = score.len() as u8;
    if score_len < SCORE_CHAR_LEN {
      for _i in 0 .. SCORE_CHAR_LEN - score_len {
        score.insert(0, '0');
      }
    }
    let mut semantic = String::new();
    let mut with_equals = false;
    if times_saved > 1 {
      with_equals = true;
      semantic.push_str(
        &format!("{} x {}", times_saved, score_reward)
      );
    } else {
      semantic.push_str(
        &format!("{}", score_reward)
      );
    }
    if let Some(moves) = moves_given {
      with_equals = true;
      semantic.push_str(
        &format!(" - {}", moves)
      );
    }
    if with_equals {
      semantic.push_str(
        &format!(" = {}", score)
      );
    }
    semantic
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

  pub fn times_saved_child(&self, child: ChildType) -> Option<ScoreType> {
    self.times_saved_children.iter().find( |(&k, v)| k == child ).map( |o| *o.1 )
  }

  pub fn saved_child(&mut self, child: ChildType) {
    *self.times_saved_children.entry(child).or_insert(0) += 1;
  }

  pub fn moved_child(&mut self, child: ChildType) {
    self.moves_counter.moved(child);
  }

  pub fn child_moves(&self) -> &HashMap<ChildType, ScoreType> {
    self.moves_counter.moves()
  }

  pub fn child_moves_for(&self, child: ChildType) -> Option<ScoreType> {
    self.moves_counter.moves_for(child)
  }

  pub fn clear(&mut self) {
    self.times_saved_player = 0;
    self.times_saved_children.clear();
    self.moves_counter.clear();
  }
}
