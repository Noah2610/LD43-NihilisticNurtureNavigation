use std::collections::hash_map::HashMap;

use ggez::{
  GameResult,
  Context,
  event::Keycode,
  audio,
};
use noframe::geo::prelude::*;

use level::Level;
use settings::level_manager::*;
use settings::res;
use animation::Animation;
use animation::Facing;
use score::Score;

pub struct LevelManager {
  level_index:      usize,
  level:            Option<Level>,
  level_names:      Vec<&'static str>,
  song:             Option<audio::Source>,
  song_names:       Vec<&'static str>,
  background:       Option<Animation>,
  window_size:      Size,
  scores:           HashMap<&'static str, Score>
}

impl LevelManager {
  pub fn new(window_size: Size) -> Self {
    Self {
      level_index: 0,
      level:       None,
      level_names: LEVEL_NAMES.to_vec(),
      song:        None,
      song_names:  SONG_NAMES.to_vec(),
      background:  None,
      window_size,
      scores:      HashMap::new()
    }
  }

  pub fn level(&mut self) -> Option<&mut Level> {
    if let Some(level) = &mut self.level {
      Some(level)
    } else {
      None
    }
  }

  pub fn next_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    // Save the current level's score
    if let Some(level) = &mut self.level {
      if let Some(level_name) = self.level_names.get(self.level_index - 1) {
        self.scores.insert(level_name, level.score().clone());
      }
    }

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
      if muted { song.pause(); }
      self.song = Some( song );
    }
    // Load background animation
    self.background = new_background(ctx, self.level_index);
    if self.level.is_some() {
      self.level_index += 1;
    }
    Ok(())
  }

  pub fn keys_pressed(&mut self, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_pressed(keys);
    }
  }

  pub fn keys_down(&mut self, keys: &Vec<Keycode>) {
    for &key in keys {
      match key {
        controls::MUTE => {
          if let Some(song) = &self.song {
            if song.paused() {
              song.resume();
            } else {
              song.pause();
            }
          }
        }
        _ => ()
      }
    }
    if let Some(level) = &mut self.level {
      level.keys_down(keys);
    }
  }

  pub fn keys_up(&mut self, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_up(keys);
    }
  }

  pub fn mouse_drag(&mut self, xrel: i32, yrel: i32) {
    if let Some(level) = &mut self.level {
      level.camera_mut().move_by(&Point::new(xrel as NumType, yrel as NumType).inverted());
    }
  }

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    let mut next_level = false;
    if let Some(level) = &mut self.level {
      level.update(ctx)?;
      next_level = level.goto_next_level();
    }
    if next_level {
      self.next_level(ctx)?;
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(bg) = &self.background {
      bg.draw(ctx, &Point::new(0.0, 0.0), &self.window_size, &Facing::Right)?;
    }
    if let Some(level) = &mut self.level {
      level.draw(ctx)?;
    }
    Ok(())
  }
}

fn new_background(ctx: &mut Context, n: usize) -> Option<Animation> {
  match n {
    _ => Some(Animation::new(
        ctx,
        vec![::join_str(res::BACKGROUND_IMAGES, "default.png")],
        vec![1000]
    ))
  }
}
