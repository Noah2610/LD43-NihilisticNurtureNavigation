pub mod helpers;

use ggez::{
  GameResult,
  Context,
  event::Keycode
};
use noframe::deltatime::Deltatime;
use noframe::camera::Camera;
use noframe::entity::Entity;

use player::Player;
use children::Child;
use wall::Wall;

pub struct Level {
  camera:        Camera,
  player:        Player,
  children:      Vec<Child>,
  walls:         Vec<Wall>,
  //interactables: Vec<T>
  dt:            Deltatime
}

impl Level {
  pub fn keys_pressed(&mut self, keycodes: &Vec<Keycode>) {

  }

  pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.player.update(_ctx);
    self.dt.update();
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.camera.draw(ctx, &self.player);
    Ok(())
  }
}
