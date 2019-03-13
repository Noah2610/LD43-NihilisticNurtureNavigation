mod helpers;

use std::collections::hash_map::HashMap;

use ggez::{
  GameResult,
  Context,
  graphics,
  event::Keycode,
  audio,
};
use json::JsonValue;
use noframe::geo::prelude::*;
use noframe::deltatime::Deltatime;

use self::helpers::*;
use level::Level;
use settings::level_manager::*;
use settings::res;
use settings::game::{ MUTED, VOLUME };
use settings::score::HIGHSCORE_COLOR;
use animation::Animation;
use animation::Facing;
use score::Score;
use menu::buttons::prelude::*;
use menu::pause::prelude::*;
use menu::stats::prelude::*;

struct ToTitleParams {
  pub beat_level:   bool,
  pub to_thank_you: bool,
}

pub struct LevelManager {
  level_index:      usize,
  level:            Option<Level>,
  level_names:      Vec<&'static str>,
  song:             Option<audio::Source>,
  song_names:       Vec<&'static str>,
  background:       Option<Animation>,
  window_size:      Size,
  scores:           HashMap<usize, Score>,
  paused:           bool,
  pause_menu:       PauseMenu,
  pause_button:     Button,
  stats_menu:       Option<StatsMenu>,
  final_stats_menu: Option<StatsMenu>,
  highscore_font:   graphics::Font,
  highscore_text:   Option<StatsText>,
  pub to_title:     bool,
  pub to_thank_you: bool,
  pub beat_game:    bool,
  pub save_data:    Option<JsonValue>,
  dt:               Deltatime,
}

impl LevelManager {
  pub fn new(ctx: &mut Context, window_size: Size) -> Self {
    Self {
      level_index:      0,
      level:            None,
      level_names:      LEVEL_NAMES.to_vec(),
      song:             None,
      song_names:       SONG_NAMES.to_vec(),
      background:       None,
      window_size:      window_size.clone(),
      scores:           HashMap::new(),
      paused:           false,
      pause_menu:       PauseMenu::new(ctx, window_size.clone()),
      pause_button:     new_pause_button(ctx, &window_size),
      stats_menu:       None,
      final_stats_menu: None,
      highscore_font:   graphics::Font::new(ctx, res::fonts::DEFAULT, HIGHSCORE_FONT_SIZE).expect("New highscore font"),
      highscore_text:   None,
      to_title:         false,
      to_thank_you:     false,
      beat_game:        false,
      save_data:        None,
      dt:               Deltatime::new(),
    }
  }

  pub fn load_level_json(&mut self, json: &JsonValue) {
    // Set level_index
    if let Some(level_index) = json["level_index"].as_usize() {
      self.level_index = level_index;
    }
    // Delete existing scores (there shouldn't be any, as this should only be called at the start of the game)
    self.scores = HashMap::new();
    // Load scores from json
    for (name, level_json) in json["levels"].entries() {
      if let Some(score) = Score::from_json(&level_json["score"]) {
        if let Some(index) = self.level_names.iter().position( |lvlname| lvlname == &name ) {
          self.scores.insert(index, score);
        }
      }
    }
  }

  fn save(&mut self) {
    let mut data = object!{
      "level_index" => self.level_index,
      "levels"      => object!{},
    };
    for (&index, score) in &self.scores {
      let name = self.level_names[index];
      data["levels"][name] = object!{};
      if let Some(score_json) = score.as_json() {
        data["levels"][name]["score"] = score_json;
      }
    }
    self.save_data = Some(data);
  }

  pub fn level(&mut self) -> Option<&mut Level> {
    if let Some(level) = &mut self.level {
      Some(level)
    } else {
      None
    }
  }

  pub fn load_level(&mut self, ctx: &mut Context, level_index: usize) -> GameResult<()> {
    self.level_index = level_index;
    self.next_level(ctx)?;
    Ok(())
  }

  pub fn total_score(&self) -> Score {
    Score::from(self.scores.values().collect::<Vec<&Score>>())
  }

  fn insert_level_score(&mut self) {
    let curr_level_index_opt = self.get_current_level_index();
    if let Some(level) = &mut self.level {
      if let Some(curr_level_index) = curr_level_index_opt {
        let score = level.score();
        let mut insert_new_score = true;
        if let Some(curr_score) = self.scores.get(&curr_level_index) {
          if score > curr_score {
            insert_new_score = true;   // Update existing score for level, if the new score is higher
          } else {
            insert_new_score = false;  // Don't update existing score for level, if the old score is higher
          }
        }
        if insert_new_score {
          self.scores.insert(curr_level_index, score.clone());
        }
      }
    }
  }

  pub fn next_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    // Remove highscore text from previous level
    self.highscore_text = None;

    // Remove StatsMenu from previous level
    self.stats_menu = None;

    // Load the next level
    if let Some(level_name) = self.level_names.get(self.level_index) {
      self.level = Some( Level::new(ctx, self.window_size.clone(), level_name, self.level_index)? );
    } else {
      self.level = None;
    }
    // Load audio
    let mut muted = false;
    if let Some(song_name) = self.song_names.get(self.level_index) {
      let mut is_same = false;
      let mut curr_song_stopped = true;
      if let Some(curr_song) = &self.song {
        if let Some(curr_level_index) = self.get_current_level_index() {
          if let Some(curr_song_name) = self.song_names.get(curr_level_index) {
            is_same = curr_song_name == song_name;
          }
        }
        muted = curr_song.paused();
        curr_song_stopped = curr_song.stopped();
        if !is_same {
          curr_song.stop();
        }
      }
      if !is_same || curr_song_stopped {
        let mut song = audio::Source::new(ctx, format!("{}{}.{}", res::AUDIO, song_name, AUDIO_FORMAT))?;
        song.set_volume(VOLUME);
        song.set_repeat(true);
        song.play()?;
        if muted || MUTED { song.pause(); }
        self.song = Some( song );
      }
    }
    // Load background animation
    self.background = new_background(ctx, self.level_index);
    if self.level.is_some() {
      self.level_index += 1;
    } else {
      self.beat_final_level(ctx)?;
    }

    // Load the highscore for this level (if available)
    self.set_highscore_text(ctx)?;

    Ok(())
  }

  fn set_highscore_text(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(highscore) = self.highscore().map( |s| s.clone() ) {
      self.highscore_text = Some(StatsText::new(
          graphics::Text::new(ctx, &highscore.semantic_highscore(), &self.highscore_font)?,
          Point::new(self.window_size.w / 2.0, 8.0),
          TextOrigin::Center,
          Some(HIGHSCORE_COLOR)
      ));
    }
    Ok(())
  }

  fn beat_final_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.beat_game = true;
    self.save();
    self.final_stats_menu = Some(
      StatsMenu::new(
        ctx,
        self.window_size.clone(),
        self.total_score(),
        None,  // TODO highscore
        true
      )?
    );
    Ok(())
  }

  pub fn keys_pressed(&mut self, _ctx: &mut Context, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_pressed(keys, &self.dt);
    }
  }

  pub fn keys_down(&mut self, ctx: &mut Context, keys: &Vec<Keycode>) {
    for &key in keys {
      match key {
        controls::MUTE  => self.toggle_mute(),
        controls::PAUSE => self.toggle_pause(),
        controls::RESET => self.reset_level(ctx).expect("Should reset level"),
        controls::NEXT =>
          if self.paused {
            self.toggle_pause();
          } else if self.stats_menu.is_some() {
            self.next_level(ctx).expect("Should load next level")
          },
        controls::TO_TITLE =>
          if self.paused || self.stats_menu.is_some() || self.final_stats_menu.is_some() {
            let has_stats_menu = self.stats_menu.is_some();
            let has_final_stats_menu = self.final_stats_menu.is_some();
            if has_final_stats_menu {
              self.to_thank_you();
            } else {
              self.to_title(ToTitleParams { beat_level: has_stats_menu, to_thank_you: false });
            }
          },
        _ => (),
      }
    }

    if let Some(level) = &mut self.level {
      level.keys_down(keys, &self.dt);
    }
  }

  pub fn keys_up(&mut self, _ctx: &mut Context, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_up(keys, &self.dt);
    }
  }

  pub fn mouse_down(&mut self, x: i32, y: i32) {
    if self.paused {
      self.pause_menu.mouse_down(x, y);
    }
    if self.level.is_some() && self.pause_button.intersects_point(&Point::new(x as NumType, y as NumType)) {
      self.toggle_pause();
    }
    if let Some(stats_menu) = &mut self.stats_menu {
      stats_menu.mouse_down(x, y);
    }
    if let Some(final_stats) = &mut self.final_stats_menu {
      final_stats.mouse_down(x, y);
    }
    if let Some(level) = &mut self.level {
      level.mouse_down(x, y);
    }
  }

  pub fn mouse_drag(&mut self, xrel: i32, yrel: i32) {
    if let Some(level) = &mut self.level {
      level.camera_mut().move_by(&Point::new(xrel as NumType, yrel as NumType).inverted());
    }
  }

  fn get_current_level_index(&self) -> Option<usize> {
    if self.level_index > 0 {
      Some(self.level_index - 1)
    } else {
      None
    }
  }

  fn toggle_mute(&mut self) {
    if let Some(song) = &self.song {
      if song.paused() {
        song.resume();
      } else {
        song.pause();
      }
    }
  }

  fn toggle_pause(&mut self) {
    if self.stats_menu.is_some() || self.final_stats_menu.is_some() { return; }
    if self.paused {
      if let Some(level) = &mut self.level {
        level.reset_dt(&self.dt);
      }
      self.paused = false;
    } else {
      self.paused = true;
    }
  }

  fn reset_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.final_stats_menu.is_some() { return Ok(()); }
    if let Some(level) = &mut self.level {
      level.reset(ctx)?;
    }
    self.paused = false;
    self.stats_menu = None;
    self.set_highscore_text(ctx)?;
    Ok(())
  }

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.update_pause_menu(ctx)?;
    if self.level.is_some() {
      self.pause_button.update()?;
    }
    self.update_stats_menu(ctx)?;
    self.update_final_stats_menu()?;
    self.update_level(ctx)?;
    self.dt.update();
    Ok(())
  }

  fn update_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.to_title || self.paused || self.stats_menu.is_some() || self.final_stats_menu.is_some() {
      return Ok(());
    }
    let mut next_level = false;
    let highscore_opt = self.highscore().map( |s| s.clone() );
    if let Some(level) = &mut self.level {
      level.update(ctx, &self.dt)?;
      if level.next_level {
        level.next_level = false;
        next_level = true;
        self.stats_menu = Some(StatsMenu::new(
            ctx,
            self.window_size.clone(),
            level.score().clone(),
            highscore_opt.map( |s| s.clone() ),
            false
        )?);
      }
    }
    if next_level {
      // NOTE: insert_level_score() THEN save()
      self.insert_level_score();
      self.save();
    }
    Ok(())
  }

  fn highscore(&self) -> Option<&Score> {
    if let Some(level_index) = self.get_current_level_index() {
      self.scores.get(&level_index)
    } else { None }
  }

  fn update_pause_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(clicked) = self.pause_menu.get_clicked().clone() {
      match clicked {
        ButtonType::PauseResume => {
          self.toggle_pause();
        }
        ButtonType::PauseToTitle => {
          self.to_title(ToTitleParams { beat_level: false, to_thank_you: false });
        }
        ButtonType::PauseReset => {
          self.reset_level(ctx)?;
        }
        _ => ()
      }
    }

    self.pause_menu.update()?;
    Ok(())
  }

  fn update_stats_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    let mut next_level = false;
    let mut reset      = false;
    let mut to_title   = false;
    if let Some(stats_menu) = &mut self.stats_menu {
      if let Some(clicked) = stats_menu.get_clicked() {
        match clicked {
          ButtonType::StatsNext    => next_level = true,
          ButtonType::StatsReset   => reset      = true,
          ButtonType::StatsToTitle => to_title   = true,
          _ => ()
        }
      }
      stats_menu.update()?;
    }
    if next_level {
      self.next_level(ctx)?;
    }
    if reset {
      self.reset_level(ctx)?;
    }
    if to_title {
      self.to_title(ToTitleParams { beat_level: true, to_thank_you: false });
    }
    Ok(())
  }

  fn update_final_stats_menu(&mut self) -> GameResult<()> {
    let mut to_thank_you = false;
    if let Some(final_stats) = &mut self.final_stats_menu {
      if let Some(ButtonType::StatsToThankYou) = final_stats.get_clicked() {
        to_thank_you = true;
      }
      final_stats.update()?;
    } else {
      return Ok(());
    }
    if to_thank_you {
      self.to_thank_you();
    }
    Ok(())
  }

  fn to_thank_you(&mut self) {
    self.reset();
    self.to_title(ToTitleParams { beat_level: false, to_thank_you: true });
  }

  fn reset(&mut self) {
    self.level_index      = 0;
    self.level            = None;
    self.song             = None;
    self.background       = None;
    //self.scores           = HashMap::new();
    self.paused           = false;
    self.stats_menu       = None;
    self.final_stats_menu = None;
    self.to_title         = false;
  }

  fn to_title(&mut self, params: ToTitleParams) {
    if !params.beat_level && self.level_index > 0 {
      self.level_index -= 1;
    }
    if let Some(level) = &mut self.level {
      level.next_level = false;
    }
    if let Some(song) = &mut self.song {
      song.stop();
    }
    self.stats_menu       = None;
    self.final_stats_menu = None;
    self.paused           = false;
    self.to_title         = true;
    if params.to_thank_you {
      self.to_thank_you = true;
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.draw_level(ctx)?;
    if self.paused {
      self.pause_menu.draw(ctx)?;
    }
    if self.level.is_some() {
      self.pause_button.draw(ctx)?;
    }
    if let Some(final_stats) = &mut self.final_stats_menu {
      final_stats.draw(ctx)?;
    } else if let Some(stats_menu) = &mut self.stats_menu {
      stats_menu.draw(ctx)?;
    } else if let Some(highscore) = &self.highscore_text {
      highscore.draw(ctx)?;
    }
    Ok(())
  }

  fn draw_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(bg) = &self.background {
      bg.draw(ctx, &Point::new(0.0, 0.0), &self.window_size, &Facing::Right)?;
    }
    if let Some(level) = &mut self.level {
      level.draw(ctx)?;
    }
    Ok(())
  }
}
