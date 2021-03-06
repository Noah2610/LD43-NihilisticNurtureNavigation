mod helpers;

use ggez::{
  GameResult,
  Context,
  graphics,
  event::Keycode
};
use noframe::deltatime::Deltatime;
use noframe::camera::Camera;
use noframe::entity::Entity;
use noframe::entity::prelude::*;
use noframe::geo::prelude::*;

use self::helpers::*;
use settings::camera::*;
use settings::level::*;
use persons::Person;
use persons::player::Player;
use persons::children::{ Child, ChildType };
use wall::Walls;
use interactables::prelude::*;
use id_generator::prelude::*;
use menu::Menu;
use menu::buttons::ButtonType;
use menu::toolbox::ToolboxMenu;
use score::prelude::*;

pub struct Level {
  json_data:       json::JsonValue,
  level_index:     usize,
  window_rect:     Rect,
  camera:          Camera,
  camera_rect:     Rect,
  player:          Player,
  children:        Vec<Child>,
  walls:           Walls,
  interactables:   InteractablesContainer,
  toolbox:         ToolboxMenu,
  pub next_level:  bool,
  font:            graphics::Font,
  level_name:      String,
  level_name_text: graphics::Text,
  score:           Score,
  prev_score:      ScoreType,
  score_text:      graphics::Text,
  dt:              Deltatime
}

impl Level {
  pub fn new(ctx: &mut Context, window_size: Size, filename: &str, level_index: usize) -> GameResult<Self> {
    new_level(ctx, window_size, filename, level_index)
  }

  pub fn reset(&mut self, ctx: &mut Context) -> GameResult<()> {
    let (
      player,
      children,
      walls,
      interactables
    ) = load_json(ctx, &self.json_data, self.level_index)?;
    self.player        = player;
    self.children      = children;
    self.walls         = walls;
    self.interactables = interactables;
    self.next_level    = false;
    self.score.clear();
    Ok(())
  }

  pub fn camera(&self) -> &Camera {
    &self.camera
  }

  pub fn camera_mut(&mut self) -> &mut Camera {
    &mut self.camera
  }

  pub fn keys_pressed(&mut self, keycodes: &Vec<Keycode>, dt: &Deltatime) {
    for key in keycodes {
      match key {
        Keycode::Up    => self.camera.point_mut().add(&Point::new(0.0, -CAMERA_SPEED * dt.secs())),
        Keycode::Down  => self.camera.point_mut().add(&Point::new(0.0,  CAMERA_SPEED * dt.secs())),
        Keycode::Left  => self.camera.point_mut().add(&Point::new(-CAMERA_SPEED * dt.secs(), 0.0)),
        Keycode::Right => self.camera.point_mut().add(&Point::new( CAMERA_SPEED * dt.secs(), 0.0)),
        _              => ()
      };
    }
    self.player.keys_pressed(keycodes);
  }

  pub fn keys_down(&mut self, keycodes: &Vec<Keycode>, _dt: &Deltatime) {
    for &key in keycodes {
      self.player.key_down(&key);
      match key {
        CENTER_KEY => self.center_camera(),
        SKIP_KEY   => self.next_level(),  // TODO: Temporary! Remove for production version!
        _          => ()
      }
    }
  }

  pub fn keys_up(&mut self, keycodes: &Vec<Keycode>, _dt: &Deltatime) {
    for key in keycodes {
      self.player.key_up(key);
    }
  }

  pub fn mouse_down(&mut self, x: i32, y: i32) {
    self.toolbox.mouse_down(x, y);
  }

  fn center_camera(&mut self) {
    let p = self.window_rect.center();
    self.camera.move_to(&p);
  }

  fn next_level(&mut self) {
    self.add_score();
    self.next_level = true;
  }

  pub fn score(&self) -> &Score {
    &self.score
  }

  pub fn level_name(&self) -> &str {
    &self.level_name
  }

  pub fn reset_dt(&mut self, dt: &Deltatime) {
    //self.dt.reset();
    self.dt = dt.clone();
    self.player.reset_dt(dt);
    self.children.iter_mut()
      .for_each( |child| child.reset_dt(dt) );
  }

  fn add_score(&mut self) {
    if self.is_player_in_goal() {
      self.score.saved_player();
    }
    let children_types: Vec<ChildType> = self.children_in_goal().iter().map( |child| child.child_type ).collect();
    for child_type in children_types {
      self.score.saved_child(child_type);
    }
  }

  fn is_player_in_goal(&self) -> bool {
    if let Some(goal) = &self.interactables.goal {
      goal.get_intersected().iter().any( |&id| self.player.has_id(id) )
    } else {
      false
    }
  }

  fn children_in_goal(&self) -> Vec<&Child> {
    if let Some(goal) = &self.interactables.goal {
      goal.get_intersected().iter()
        .filter_map( |&id| self.children.iter().find( |child| child.has_id(id) ))
        .collect()
    } else {
      Vec::new()
    }
  }

  pub fn update(&mut self, ctx: &mut Context, _dt: &Deltatime) -> GameResult<()> {
    self.update_interactables(ctx)?;
    self.update_children(ctx)?;
    self.update_player(ctx)?;
    self.update_toolbox()?;
    //self.dt.update();
    Ok(())
  }

  fn update_interactables(&mut self, ctx: &mut Context) -> GameResult<()> {
    let mut ids_to_trigger: Vec<IdType> = Vec::new();

    for i in 0 .. self.interactables.switches.len() {
      { let mut switch = &mut self.interactables.switches[i];
        if switch.intersects_round(&self.player) {
          switch.trigger_once(&mut self.player);
        } else {
          switch.set_intersected(&self.player, false);
        }
        for child in &mut self.children {
          if switch.intersects_round(child) {
            switch.trigger_once(child);
          } else {
            switch.set_intersected(&*child, false);
          }
        }
        switch.update(ctx)?; }
      { let switch = &self.interactables.switches[i];
        ids_to_trigger.append(&mut switch.get_interactables_to_trigger()); }
      self.interactables.switches[i].interactables_triggered();
    }

    for jump_pad in &mut self.interactables.jump_pads {
      if ids_to_trigger.contains(&jump_pad.id()) {
        jump_pad.toggle_state();
      }
      if jump_pad.intersects_center(&self.player) {
        jump_pad.trigger_once(&mut self.player);
      } else {
        jump_pad.set_intersected(&self.player, false);
      }
      for child in &mut self.children {
        if jump_pad.intersects_center(child) {
          jump_pad.trigger_once(child);
        } else {
          jump_pad.set_intersected(&*child, false);
        }
      }
      jump_pad.update(ctx)?;
    }

    for door in &mut self.interactables.doors {
      if ids_to_trigger.contains(&door.id()) {
        door.trigger(&mut self.player);  // We don't use the player, but something needs to be passed...
      }
      door.update(ctx)?;
    }

    let mut player_in_solidifier = false;
    for solidifier in &mut self.interactables.solidifiers {
      if solidifier.intersects_round(&self.player) {
        player_in_solidifier = true;
        solidifier.trigger_once(&mut self.player);
      }
      for child in &mut self.children {
        if solidifier.intersects_round(child) {
          solidifier.trigger_once(child);
        }
      }
    }
    if !player_in_solidifier {
      self.player.unsolidify();
    }

    // Check if persons intersect with goal
    if let Some(goal) = &mut self.interactables.goal {
      if goal.intersects_round(&self.player) {
        goal.trigger_once(&mut self.player);
      } else {
        goal.set_intersected(&self.player, false);
      }
      for child in &mut self.children {
        if child.state_is_still() && goal.intersects_round(&*child) {
          goal.trigger_once(child);
        } else {
          goal.set_intersected(&*child, false);
        }
      }
      goal.update(ctx)?;
    }

    Ok(())
  }

  fn update_children(&mut self, ctx: &mut Context) -> GameResult<()> {
    for i in 0 .. self.children.len() {
      let mut new_pos = {
        let child = &self.children[i];
        child.get_move_while( |rect| {
          let intersects_wall = self.walls.walls.iter().any( |wall| {
            rect.intersects_round(wall)
          });
          let intersects_door = self.interactables.solid_doors().iter().any( |&door| {
            rect.intersects_round(door)
          });
          let intersects_oneway = child.velocity().y > 0.0 && self.interactables.one_ways.iter().any( |oneway| {
            rect.intersects_round(oneway)
          });
          let intersects_solid_children = self.children.iter().any( |c| {
            !child.is(c) && c.is_solid() && rect.intersects_round(c)
          });
          let intersects_solid_player = self.player.is_solid() && rect.intersects_round(&self.player);
          !intersects_wall && !intersects_door && !intersects_oneway && !intersects_solid_children && !intersects_solid_player
        })
      };
      let child = &mut self.children[i];
      // Kill x velocity when hitting a wall
      if child.velocity().x != 0.0 && new_pos.x == child.point().x
      {
        child.set_velocity_x(0.0);
        // Stop walking when hitting a wall AND standing on floor
        if child.on_floor() {
          child.stop_walking();
        }
      }
      // Kill y velocity when standing on floor
      if child.velocity().y != 0.0 && new_pos.y == child.point().y {
        child.set_velocity_y(0.0);
      }
      // Move to new position
      if &new_pos != child.point() {
        child.point_mut().set(&new_pos);
      }
      // Child is stuck
      if self.interactables.solid_doors().iter().any( |&door| child.intersects_round(door) ) {
        let x = child.point().x + ((child.size().w * 2.0) * (child.walk_direction_mult() * -1.0));
        child.point_mut().set_x(x);
      }
      child.update(ctx)?;
    }
    Ok(())
  }

  fn child(&self, child_type: ChildType) -> Option<&Child> {
    self.children.iter().find( |c| c.child_type == child_type )
  }

  fn child_mut(&mut self, child_type: ChildType) -> Option<&mut Child> {
    self.children.iter_mut().find( |c| c.child_type == child_type )
  }

  fn update_player(&mut self, ctx: &mut Context) -> GameResult<()> {
    // Player is stuck
    if self.interactables.solid_doors().iter().any( |&door| self.player.intersects_round(door) ) {
      let x = self.player.point().x - (self.player.size().w * 2.0);
      self.player.point_mut().set_x(x);
    }
    // Move
    let new_pos = self.player.get_move_while( |rect| {
      let intersects_wall = self.walls.walls.iter().any( |wall| {
        rect.intersects_round(wall)
      });
      let intersects_door = self.interactables.solid_doors().iter().any( |&door| {
        rect.intersects_round(door)
      });
      let intersects_oneway = self.player.velocity().y > 0.0 && self.interactables.one_ways.iter().any( |oneway| {
        rect.intersects_round(oneway)
      });
      let intersects_solid_children = self.children.iter().any( |child| {
        child.is_solid() && rect.intersects_round(child)
      });
      !intersects_wall && !intersects_door && !intersects_oneway && !intersects_solid_children
    });

    if self.player.velocity().x != 0.0 && new_pos.x == self.player.point().x {
      self.player.set_velocity_x(0.0);
    }
    // Hacky way to check if a solid block is beneath
    if self.player.velocity().y != 0.0 && new_pos.y == self.player.point().y {
      self.player.set_velocity_y(0.0);
      self.player.stop_jumping();
    }
    if &new_pos != self.player.point() {
      self.player.point_mut().set(&new_pos);
    }
    self.player.update(ctx)
  }

  fn update_toolbox(&mut self) -> GameResult<()> {
    if let Some(button_type) = self.toolbox.get_clicked().clone() {
      match button_type {
        ButtonType::NextLevel  => self.next_level(),
        ButtonType::LarryLeft  => self.child_walk_left( ChildType::Larry),
        ButtonType::LarryRight => self.child_walk_right(ChildType::Larry),
        ButtonType::ThingLeft  => self.child_walk_left( ChildType::Thing),
        ButtonType::ThingRight => self.child_walk_right(ChildType::Thing),
        ButtonType::BloatLeft  => self.child_walk_left( ChildType::Bloat),
        ButtonType::BloatRight => self.child_walk_right(ChildType::Bloat),
        _                      => (),
      };
    }
    if let Some(goal) = &self.interactables.goal {
      self.toolbox.set_to_save_amount(goal.get_intersected().len());
    }
    self.toolbox.update()?;
    Ok(())
  }

  fn child_walk_left(&mut self, child_type: ChildType) {
    let mut moved = false;
    if let Some(child) = self.child_mut(child_type) {
      moved = child.try_walk_left();
    }
    if moved {
      self.moved_child(child_type);
    }
  }

  fn child_walk_right(&mut self, child_type: ChildType) {
    let mut moved = false;
    if let Some(child) = self.child_mut(child_type) {
      moved = child.try_walk_right();
    }
    if moved {
      self.moved_child(child_type);
    }
  }

  fn moved_child(&mut self, child_type: ChildType) {
    self.score.moved_child(child_type);
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(goal) = &mut self.interactables.goal {
      self.camera.draw(ctx, goal)?;
    }
    self.draw_walls(ctx)?;
    self.draw_children(ctx)?;
    self.draw_player(ctx)?;
    self.draw_interactables(ctx)?;
    self.draw_level_name(ctx)?;
    self.toolbox.draw(ctx)?;
    Ok(())
  }

  fn draw_walls(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.walls.draw_offset(ctx, &self.camera.top_left().inverted())?;
    Ok(())
  }

  fn draw_interactables(&mut self, ctx: &mut Context) -> GameResult<()> {
    for jump_pad in &mut self.interactables.jump_pads {
      self.camera.draw(ctx, jump_pad)?;
    }
    for switch in &mut self.interactables.switches {
      self.camera.draw(ctx, switch)?;
    }
    for door in &mut self.interactables.doors {
      self.camera.draw(ctx, door)?;
    }
    for oneway in &mut self.interactables.one_ways {
      self.camera.draw(ctx, oneway)?;
    }
    Ok(())
  }

  fn draw_children(&mut self, ctx: &mut Context) -> GameResult<()> {
    for child in &self.children {
      self.camera.draw(ctx, child)?;
    }
    Ok(())
  }

  fn draw_player(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.camera.draw(ctx, &self.player)
  }

  fn draw_level_name(&mut self, ctx: &mut Context) -> GameResult<()> {
    let dest = graphics::Point2::from(
      &(Point::new(-8.0, 8.0) + self.window_rect.top_right())
    );
    let param = graphics::DrawParam {
      dest,
      offset: graphics::Point2::new(1.0, 0.0),
      color:  Some(noframe::color::BLACK.into()),
      .. Default::default()
    };
    graphics::draw_ex(ctx, &self.level_name_text, param)?;
    Ok(())
  }

  // TODO: We don't need to draw the score during a Level
  fn draw_score(&mut self, ctx: &mut Context) -> GameResult<()> {
    let dest = graphics::Point2::from(&self.window_rect.top_left());
    let param = graphics::DrawParam {
      dest,
      color: Some(noframe::color::BLACK.into()),
      .. Default::default()
    };
    let text = self.score_text(ctx)?;
    graphics::draw_ex(ctx, text, param)?;
    Ok(())
  }

  fn score_text(&mut self, ctx: &mut Context) -> GameResult<&graphics::Text> {
    let score = self.score.score();
    if self.prev_score != score {
      self.prev_score = score;
      self.score_text = graphics::Text::new(ctx, &self.score.semantic_score(), &self.font)?;
    }
    Ok(&self.score_text)
  }
}
