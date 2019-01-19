mod helpers;

use std::collections::hash_map::HashMap;
use std::time::{ Instant, Duration };

use ggez::{
  GameResult,
  Context,
  event::Keycode,
  audio,
};
use noframe::geo::prelude::*;
use climer::time::{ Time, TimeBuilder };

use self::helpers::*;
use level::Level;
use settings::level_manager::*;
use settings::res;
use settings::game::MUTED;
use animation::Animation;
use animation::Facing;
use score::Score;
use menu::buttons::prelude::*;
use menu::pause::prelude::*;
use menu::stats::prelude::*;

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
  pub to_title:     bool,
  pub beat_game:    bool,

  level_start:      Option<Instant>,
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
      to_title:         false,
      beat_game:        false,

      level_start:      None,
    }
  }

  pub fn time(&self) -> Option<impl std::fmt::Display> {
    if let Some(start) = self.level_start {
      let dur = Instant::now().duration_since(start);
      let time = TimeBuilder::new()
        .seconds(dur.as_secs())
        .build();
      Some(time)
    } else {
      None
    }
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

  pub fn next_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    // Save the current level's score
    if let Some(level) = &mut self.level {
      let prev_level_index_opt = if self.level_index > 0 { Some(self.level_index - 1) } else { None };
      if let Some(prev_level_index) = prev_level_index_opt {
        if !self.scores.contains_key(&prev_level_index) {
          self.scores.insert(prev_level_index, level.score().clone());
        }
      }
    }

    self.stats_menu = None;

    // Load the next level
    if let Some(level_name) = self.level_names.get(self.level_index) {
      self.level = Some( Level::new(ctx, self.window_size.clone(), level_name)? );
    } else {
      self.level = None;
    }
    // Load audio
    let mut muted = false;
    if let Some(song) = &self.song {
      muted = song.paused();
      song.stop();
    }
    if let Some(song_name) = self.song_names.get(self.level_index) {
      let mut song = audio::Source::new(ctx, format!("{}{}.{}", res::AUDIO, song_name, AUDIO_FORMAT))?;
      song.set_volume(0.5);
      song.set_repeat(true);
      song.play()?;
      if muted || MUTED { song.pause(); }
      self.song = Some( song );
    }
    // Load background animation
    self.background = new_background(ctx, self.level_index);
    if self.level.is_some() {
      self.level_index += 1;
    } else {
      self.beat_final_level(ctx)?;
    }

    self.level_start = Some(Instant::now());

    Ok(())
  }

  fn beat_final_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.beat_game = true;
    self.final_stats_menu = Some(
      StatsMenu::new(
        ctx,
        self.window_size.clone(),
        Score::from(self.scores.values().collect::<Vec<&Score>>()),
        true
      )?
    );
    Ok(())
  }

  pub fn keys_pressed(&mut self, _ctx: &mut Context, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_pressed(keys);
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
          if self.paused || self.stats_menu.is_some() {
            let has_stats_menu = self.stats_menu.is_some();
            self.to_title(has_stats_menu);
          },
        _ => (),
      }
    }

    if let Some(level) = &mut self.level {
      level.keys_down(keys);
    }
  }

  pub fn keys_up(&mut self, _ctx: &mut Context, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_up(keys);
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
        level.reset_dt();
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
      self.level_start = Some(Instant::now());
    }
    self.paused = false;
    self.stats_menu = None;
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
    Ok(())
  }

  fn update_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.to_title || self.paused || self.stats_menu.is_some() || self.final_stats_menu.is_some() {
      return Ok(());
    }
    if let Some(level) = &mut self.level {
      level.update(ctx)?;
      if level.next_level {
        self.stats_menu = Some(StatsMenu::new(ctx, self.window_size.clone(), level.score().clone(), false)?);
      }
    }
    Ok(())
  }

  fn update_pause_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(clicked) = self.pause_menu.get_clicked().clone() {
      match clicked {
        ButtonType::PauseResume => {
          self.toggle_pause();
        }
        ButtonType::PauseToTitle => {
          self.to_title(false);
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
      self.to_title(true);
    }
    Ok(())
  }

  fn update_final_stats_menu(&mut self) -> GameResult<()> {
    let mut reset_and_to_title = false;
    if let Some(final_stats) = &mut self.final_stats_menu {
      if let Some(ButtonType::StatsToTitle) = final_stats.get_clicked() {
        reset_and_to_title = true;
      }
      final_stats.update()?;
    } else {
      return Ok(());
    }
    if reset_and_to_title {
      self.reset();
      self.to_title(false);
    }
    Ok(())
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

  fn to_title(&mut self, beat_level: bool) {
    if !beat_level && self.level_index > 0 {
      self.level_index -= 1;
    }
    if let Some(level) = &mut self.level {
      level.next_level = false;
    }
    if let Some(song) = &mut self.song {
      song.stop();
    }
    self.stats_menu = None;
    self.paused     = false;
    self.to_title   = true;
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.draw_level(ctx)?;
    if self.paused {
      self.pause_menu.draw(ctx)?;
    }
    if self.level.is_some() {
      self.pause_button.draw(ctx)?;
    }
    if let Some(stats_menu) = &mut self.stats_menu {
      stats_menu.draw(ctx)?;
    }
    if let Some(final_stats) = &mut self.final_stats_menu {
      final_stats.draw(ctx)?;
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
