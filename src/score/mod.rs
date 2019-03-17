mod child_moves_counter;

use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;
use std::ops;

use json::JsonValue;

use settings::score::*;
use settings::player;
use persons::children::ChildType;
use self::child_moves_counter::ChildMovesCounter;

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
        if data.has_key("saved") {  //                     vvvvvvvv  as ScoreType
          times_saved_children.insert(child, data["saved"].as_u32().unwrap_or(0));
        }
        if data.has_key("moves") {  //                     vvvvvvvv  as ScoreType
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
    let mut semantic = "Saved".to_string();
    // Player
    self.append_semantic_score_for_player(&mut semantic);
    // Children
    self.append_semantic_score_for_children(&mut semantic);
    // Remove/replace trailing comma
    if let Some(last) = semantic.pop() {
      if last == ',' {
        semantic.push('!');
      } else {
        semantic.push(last);
      }
    }
    // Replace final comma with semantic ", and"
    if let Some(index) = semantic.rfind(',') {
      let mut last = semantic.split_off(index);
      last.remove(0);
      semantic.push_str(&format!(", and {}", last));
    }
    semantic
  }

  fn append_semantic_score_for_player(&self, semantic: &mut String) {
    use settings::player::NAME;
    let s;
    if self.times_saved_player == 1 {
      s = format!(" {},", NAME);
    } else if self.times_saved_player > 1 {
      s = format!(" {} {} times,", NAME, self.times_saved_player);
    } else {
      s = "".to_string();
    }
    semantic.push_str(&s);
  }

  fn append_semantic_score_for_children(&self, semantic: &mut String) {
    for (child, &times) in &self.times_saved_children {
      let s;
      if times == 1 {
        s = format!(" {},", child.name());
      } else if times > 1 {
        s = format!(" {} {} times,", child.name(), times);
      } else {
        s = "".to_string();
      }
      semantic.push_str(&s);
    }
  }

  pub fn semantic_highscore(&self) -> String {
    format!("Highscore: {}", self)
  }

  pub fn semantic_player(&self) -> Option<String> {
    if let Some(semantic) = self.semantic_score_for(self.times_saved_player, PLAYER_SCORE_REWARD, None) {
      Some(format!("{}: {}", player::NAME, semantic))
    } else { None }
  }

  pub fn semantic_children(&self) -> Vec<String> {
    use self::ChildType::*;
    let mut children = [Larry, Thing, Bloat];
    children.sort();
    children.iter()
      .filter_map( |&child| {
        let saved = self.times_saved_child(child).unwrap_or(0);
        if let Some(semantic) = self.semantic_score_for(saved, CHILD_SCORE_REWARD, self.moves_counter.moves_for(child)) {
          Some(format!("{}: {}", child.name(), semantic))
        } else { None }
      })
    .collect()
  }

  pub fn any(&self) -> bool {
    self.score() > 0
  }

  fn semantic_score_for(&self, times_saved: ScoreType, score_reward: ScoreType, moves_given: Option<ScoreType>) -> Option<String> {
    let score_saved   = times_saved * score_reward;
    let moves_given_n = moves_given.unwrap_or(0);
    if times_saved == 0 && moves_given_n == 0 {
      return None;
    }
    let score = score_saved as i32 - moves_given_n as i32;
    let mut score_str = format!("{}", score);
    let score_len = score_str.len() as u8;
    if score_len < SCORE_CHAR_LEN {
      for _i in 0 .. SCORE_CHAR_LEN - score_len {
        score_str.insert(0, '0');
      }
    }
    let mut semantic = String::new();
    let mut with_equals = false;
    if times_saved > 1 {
      with_equals = true;
      semantic.push_str(
        &format!("{} x {}", times_saved, score_reward)
      );
    } else if times_saved == 1 {
      semantic.push_str(
        &format!("{}", score_reward)
      );
    } else {
      semantic.push('0');
    }
    if let Some(moves) = moves_given {
      with_equals = true;
      semantic.push_str(
        &format!(" - {}", moves)
      );
    }
    if with_equals {
      semantic.push_str(
        &format!(" = {}", score_str)
      );
    }
    Some(semantic)
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
    self.times_saved_children.iter().find( |(&k, _)| k == child ).map( |o| *o.1 )
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

// IMPLEMENTATIONS

impl From<Vec<&Score>> for Score {
  fn from(scores: Vec<&Score>) -> Self {
    let mut score_acc = Score::new();
    for score in scores {
      score_acc += score;
    }
    score_acc
  }
}

impl ops::AddAssign<&Score> for Score {
  fn add_assign(&mut self, other: &Score) {
    use self::ChildType::*;
    // player
    self.times_saved_player += other.times_saved_player();
    for &child in &[Larry, Thing, Bloat] {
      // children
      if let Some(other_saved) = other.times_saved_child(child) {
        *self.times_saved_children.entry(child).or_insert(0) += other_saved;
      }
      // moves
      self.moves_counter += &other.moves_counter;
    }
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
