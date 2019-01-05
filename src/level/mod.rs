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
use wall::Wall;
use interactables::prelude::*;
use id_generator::prelude::*;
use menu::Menu;
use menu::buttons::ButtonType;
use menu::toolbox::ToolboxMenu;
use score::prelude::*;

pub struct InteractablesContainer {
  pub jump_pads:   Vec<JumpPad>,
  pub switches:    Vec<Switch>,
  pub doors:       Vec<Door>,
  pub one_ways:    Vec<OneWay>,
  pub solidifiers: Vec<Solidifier>,
  pub goal:        Option<Goal>
}

impl InteractablesContainer {
  pub fn new() -> Self {
    Self {
      jump_pads:   Vec::new(),
      switches:    Vec::new(),
      doors:       Vec::new(),
      one_ways:    Vec::new(),
      solidifiers: Vec::new(),
      goal:        None
    }
  }

  pub fn solid_doors(&self) -> Vec<&Door> {
    self.doors.iter().filter( |door| door.is_solid() ).collect()
  }
}

pub struct Level {
  json_data:       json::JsonValue,
  window_rect:     Rect,
  camera:          Camera,
  camera_rect:     Rect,
  player:          Player,
  children:        Vec<Child>,
  walls:           Vec<Wall>,
  interactables:   InteractablesContainer,
  toolbox:         ToolboxMenu,
  next_level:      bool,
  font:            graphics::Font,
  level_name:      String,
  level_name_text: graphics::Text,
  score:           Score,
  prev_score:      ScoreType,
  score_text:      graphics::Text,
  dt:              Deltatime
}

impl Level {
  pub fn new(ctx: &mut Context, window_size: Size, filename: &str) -> GameResult<Self> {
    new_level(ctx, window_size, filename)
  }

  pub fn reset(&mut self, ctx: &mut Context) -> GameResult<()> {
    let (
      player,
      children,
      walls,
      interactables
    ) = load_json(ctx, &self.json_data)?;
    self.player        = player;
    self.children      = children;
    self.walls         = walls;
    self.interactables = interactables;
    Ok(())
  }

  pub fn camera(&self) -> &Camera {
    &self.camera
  }

  pub fn camera_mut(&mut self) -> &mut Camera {
    &mut self.camera
  }

  pub fn keys_pressed(&mut self, keycodes: &Vec<Keycode>) {
    for key in keycodes {
      match key {
        Keycode::Up    => self.camera.point_mut().add(&Point::new(0.0, -CAMERA_SPEED * self.dt.secs())),
        Keycode::Down  => self.camera.point_mut().add(&Point::new(0.0,  CAMERA_SPEED * self.dt.secs())),
        Keycode::Left  => self.camera.point_mut().add(&Point::new(-CAMERA_SPEED * self.dt.secs(), 0.0)),
        Keycode::Right => self.camera.point_mut().add(&Point::new( CAMERA_SPEED * self.dt.secs(), 0.0)),
        _              => ()
      };
    }
    self.player.keys_pressed(keycodes);
  }

  pub fn keys_down(&mut self, keycodes: &Vec<Keycode>) {
    for &key in keycodes {
      self.player.key_down(&key);
      match key {
        CENTER_KEY => self.center_camera(),
        SKIP_KEY   => self.next_level(),  // TODO: Temporary! Remove for production version!
        _          => ()
      }
    }
  }

  pub fn keys_up(&mut self, keycodes: &Vec<Keycode>) {
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

  pub fn reset_dt(&mut self) {
    self.dt.reset();
    self.player.reset_dt();
    self.children.iter_mut()
      .for_each( |child| child.reset_dt() );
  }

  fn add_score(&mut self) {
    if self.is_player_in_goal() {
      // self.score.add_for(&self.player);
      // self.score.saved_player();
      self.score.saved_player();
    }
    // let children_score = self.children_in_goal().iter().fold(0, |acc, c| acc + c.score());
    // self.score.add(children_score);
    let children_types: Vec<ChildType> = self.children_in_goal().iter().map( |child| child.child_type.clone() ).collect();
    for child_type in children_types {
      self.score.saved_child(child_type);
    }
    // self.children_in_goal().iter().for_each( |child| self.score.saved_child(child.child_type.clone()) );
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

  pub fn should_goto_next_level(&self) -> bool {
    self.next_level
  }

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.update_interactables(ctx)?;
    self.update_children(ctx)?;
    self.update_player(ctx)?;
    self.update_toolbox()?;
    self.dt.update();
    Ok(())
  }

  fn update_interactables(&mut self, ctx: &mut Context) -> GameResult<()> {
    let mut ids_to_trigger: Vec<IdType> = Vec::new();

    for i in 0 .. self.interactables.switches.len() {
      { let mut switch = &mut self.interactables.switches[i];
        if switch.intersects(&self.player) {
          switch.trigger_once(&mut self.player);
        } else {
          switch.set_intersected(&self.player, false);
        }
        for child in &mut self.children {
          if switch.intersects(child) {
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
      if solidifier.intersects(&self.player) {
        player_in_solidifier = true;
        solidifier.trigger_once(&mut self.player);
      }
      for child in &mut self.children {
        if solidifier.intersects(child) {
          solidifier.trigger_once(child);
        }
      }
    }
    if !player_in_solidifier {
      self.player.unsolidify();
    }

    if let Some(goal) = &mut self.interactables.goal {
      if goal.intersects(&self.player) {
        goal.trigger_once(&mut self.player);
      } else {
        goal.set_intersected(&self.player, false);
      }
      for child in &mut self.children {
        if goal.intersects(&*child) {
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
      let new_pos = {
        let child = &self.children[i];
        child.get_move_while( |rect| {
          let intersects_wall = self.walls.iter().any( |wall| {
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
      if child.velocity().x != 0.0 && new_pos.x == child.point().x {
        child.set_velocity_x(0.0);
        child.stop_walking();
      }
      if child.velocity().y != 0.0 && new_pos.y == child.point().y {
        child.set_velocity_y(0.0);
      }
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

  fn larry(&mut self) -> Option<&mut Child> {
    self.children.iter_mut().find( |c| c.child_type == ChildType::Larry )
  }

  fn thing(&mut self) -> Option<&mut Child> {
    self.children.iter_mut().find( |c| c.child_type == ChildType::Thing )
  }

  fn bloat(&mut self) -> Option<&mut Child> {
    self.children.iter_mut().find( |c| c.child_type == ChildType::Bloat )
  }

  fn update_player(&mut self, ctx: &mut Context) -> GameResult<()> {
    // Player is stuck
    if self.interactables.solid_doors().iter().any( |&door| self.player.intersects_round(door) ) {
      let x = self.player.point().x - (self.player.size().w * 2.0);
      self.player.point_mut().set_x(x);
    }
    // Move
    let new_pos = self.player.get_move_while( |rect| {
      let intersects_wall = self.walls.iter().any( |wall| {
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
        ButtonType::LarryLeft  => if let Some(child) = self.larry() { child.walk_left()  },
        ButtonType::LarryRight => if let Some(child) = self.larry() { child.walk_right() },
        ButtonType::ThingLeft  => if let Some(child) = self.thing() { child.walk_left()  },
        ButtonType::ThingRight => if let Some(child) = self.thing() { child.walk_right() },
        ButtonType::BloatLeft  => if let Some(child) = self.bloat() { child.walk_left()  },
        ButtonType::BloatRight => if let Some(child) = self.bloat() { child.walk_right() },
        _                      => ()
      };
    }
    if let Some(goal) = &self.interactables.goal {
      self.toolbox.set_to_save_amount(goal.get_intersected().len());
    }
    self.toolbox.update()?;
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.draw_walls(ctx)?;
    self.draw_children(ctx)?;
    self.draw_player(ctx)?;
    self.draw_interactables(ctx)?;
    self.draw_level_name(ctx)?;
    self.toolbox.draw(ctx)?;
    Ok(())
  }

  fn draw_walls(&mut self, ctx: &mut Context) -> GameResult<()> {
    for wall in &self.walls {
      self.camera.draw(ctx, wall)?;
    }
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
    let dest = graphics::Point2::from(&self.window_rect.top_right());
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
      self.score_text = graphics::Text::new(ctx, &self.score.semantic(), &self.font)?;
    }
    Ok(&self.score_text)
  }
}
