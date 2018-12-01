use std::fs::File;
use std::io::prelude::*;

use ggez::{
  GameResult,
  Context,
  event::Keycode
};
use noframe::deltatime::Deltatime;
use noframe::camera::Camera;
use noframe::entity::Entity;
use noframe::entity::traits::movement::Movement;
use noframe::geo::prelude::*;

use settings::res;
use people::player::Player;
use people::children::Child;
use wall::Wall;

pub struct Level {
  window_rect:   Rect,
  camera:        Camera,
  camera_rect:   Rect,
  player:        Player,
  children:      Vec<Child>,
  walls:         Vec<Wall>,
  //interactables: Vec<T>
  dt:            Deltatime
}

impl Level {
  pub fn new(ctx: &mut Context, level_name: &str) -> GameResult<Self> {
    let level_name = &::join_str(level_name, ".json");
    let level_filepath = &::join_str(res::LEVELS, level_name);
    println!("{}", level_filepath);
    let mut level_file = File::open(level_filepath)?;
    let mut json_raw = String::new();
    level_file.read_to_string(&mut json_raw)?;
    let data = match json::parse(&json_raw) {
      Ok(d)  => d,
      Err(e) => return Err(ggez::GameError::from(e.to_string()))
    };  // I don't think this is idomatic rust...

    let size = if data.has_key("size") {
      let err_msg = "Couldn't load level JSON data: size (root)";
      Size::new(data["size"]["w"].as_f32().expect(err_msg), data["size"]["h"].as_f32().expect(err_msg))
    } else { panic!("Level JSON size (root) attribute not present") };
    let mut player_opt = None;
    let mut children = Vec::new();
    let mut walls = Vec::new();

    data["instances"].members().for_each( |data| {
      let point_opt = if data.has_key("position") {
        let err_msg = "Couldn't load level JSON data: position";
        Some(Point::new(data["position"]["x"].as_f32().expect(err_msg), data["position"]["y"].as_f32().expect(err_msg)))
      } else { None };
      let size_opt = if data.has_key("size") {
        let err_msg = "Couldn't load level JSON data: size";
        Some(Size::new(data["size"]["w"].as_f32().expect(err_msg), data["size"]["h"].as_f32().expect(err_msg)))
      } else { None };

      match data["type"].as_str().expect("Couldn't load level JSON data: type") {
        "Player" => {
          let err_msg = "Couldn't load level JSON data: Player";
          player_opt = Some(Player::new(ctx, point_opt.expect(err_msg), size_opt.expect(err_msg)));
        },
        "Child" => {
          let err_msg = "Couldn't load level JSON data: Child";
          children.push(Child::new(ctx, point_opt.expect(err_msg), size_opt.expect(err_msg)))
        },
        "Wall" => {
          let err_msg = "Couldn't load level JSON data: Wall";
          walls.push(Wall::new(ctx, point_opt.expect(err_msg), size_opt.expect(err_msg)));
        }
        _ => {}
      }
    });

    let player = if let Some(player) = player_opt {
      player
    } else {
      return Err(ggez::GameError::from("Couldn't load player".to_string()));
    };

    Ok(Level {
      window_rect: Rect::new(Point::new(0.0, 0.0), size.clone(), Origin::TopLeft),
      camera:      Camera::new(),
      camera_rect: Rect::new(Point::new(0.0, 0.0), size, Origin::TopLeft),
      player,
      children,
      walls,
      dt:          Deltatime::new()
    })
  }

  pub fn keys_pressed(&mut self, keycodes: &Vec<Keycode>) {
    self.player.keys_pressed(keycodes);
  }

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.update_children(ctx)?;
    self.update_player(ctx)?;
    self.dt.update();
    Ok(())
  }

  fn update_children(&mut self, ctx: &mut Context) -> GameResult<()> {
    for child in &mut self.children {
      child.update(ctx)?;
    }
    Ok(())
  }

  fn update_player(&mut self, ctx: &mut Context) -> GameResult<()> {
    let new_pos = self.player.get_move_while(
      |rect| !self.walls.iter().any( |wall| rect.intersects_round(wall) )
    );
    if &new_pos != self.player.point() {
      self.player.point_mut().set(&new_pos);
    }
    self.player.update(ctx)
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    for wall in &self.walls {
      self.camera.draw(ctx, wall)?;
    }
    for child in &self.children {
      self.camera.draw(ctx, child)?;
    }
    self.camera.draw(ctx, &self.player)
  }
}
