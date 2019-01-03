const MAX_FRAMES: usize = 10;

use std::time::{ Instant, Duration };

pub struct FramesCounter {
  avg:            f32,
  frames:         Vec<usize>,
  current_frames: usize,
  last_update:    Instant,
}

impl FramesCounter {
  pub fn new() -> Self {
    Self {
      avg:            0.0,
      frames:         Vec::new(),
      current_frames: 0,
      last_update:    Instant::now(),
    }
  }

  pub fn avg(&self) -> f32 {
    self.avg
  }

  pub fn current(&self) -> usize {
    *self.frames.last().unwrap_or(&0)
  }

  pub fn calculate_fps(&mut self) {
    self.avg = self.frames.iter().sum::<usize>() as f32 / self.frames.len() as f32;
  }

  pub fn update(&mut self) {
    let now = Instant::now();
    if now - self.last_update < Duration::from_secs(1) {
      self.current_frames += 1;
      return;
    }
    if self.frames.len() >= MAX_FRAMES {
      self.frames.remove(0);
    }
    self.frames.push(self.current_frames);
    self.calculate_fps();
    self.current_frames = 0;
    self.last_update = now;
  }
}
