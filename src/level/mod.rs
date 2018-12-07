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
use noframe::entity::prelude::*;
use noframe::geo::prelude::*;

use settings::res;
use settings::camera::*;
use persons::Person;
use persons::player::Player;
use persons::children::Child;
use persons::children::ChildType;
use wall::Wall;
use interactables::prelude::*;
use id_generator::prelude::*;
use menu::Menu;
use menu::ButtonType;
use menu::toolbox::ToolboxMenu;

struct InteractablesContainer {
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
  window_rect:   Rect,
  camera:        Camera,
  camera_rect:   Rect,
  player:        Player,
  children:      Vec<Child>,
  walls:         Vec<Wall>,
  interactables: InteractablesContainer,
  toolbox:       ToolboxMenu,
  dt:            Deltatime
}

impl Level {
  pub fn new(ctx: &mut Context, window_size: Size, level_name: &str) -> GameResult<Self> {
    let level_name = &::join_str(level_name, ".json");
    let level_filepath = &::join_str(res::LEVELS, level_name);
    let mut level_file = File::open(level_filepath)?;
    let mut json_raw = String::new();
    level_file.read_to_string(&mut json_raw)?;
    let data = match json::parse(&json_raw) {
      Ok(d)  => d,
      Err(e) => return Err(ggez::GameError::from(e.to_string()))
    };

    let size = if data.has_key("size") {
      let err_msg = "Couldn't load level JSON data: size (root)";
      Size::new(data["size"]["w"].as_f32().expect(err_msg), data["size"]["h"].as_f32().expect(err_msg))
    } else { panic!("Level JSON size (root) attribute not present") };

    let mut player_opt = None;
    let mut children = Vec::new();
    let mut walls = Vec::new();
    let mut interactables = InteractablesContainer::new();

    data["instances"].members().for_each( |data| {
      let point_opt = if data.has_key("position") {
        let err_msg = "Couldn't load level JSON data: position";
        Some(Point::new(data["position"]["x"].as_f32().expect(err_msg), data["position"]["y"].as_f32().expect(err_msg)))
      } else { None };
      let size_opt = if data.has_key("size") {
        let err_msg = "Couldn't load level JSON data: size";
        Some(Size::new(data["size"]["w"].as_f32().expect(err_msg), data["size"]["h"].as_f32().expect(err_msg)))
      } else { None };
      let ( state_opt, id_opt, color_opt, triggers_opt ) = if data.has_key("additional") {
        (
          if data["additional"].has_key("state") {
            Some( data["additional"]["state"].as_str().expect("Couldn't load level JSON data: state") )
          } else { None },
          if data["additional"].has_key("id") {
            Some( data["additional"]["id"].as_u32().expect("Couldn't load level JSON data: id") )
          } else { None },
          if data["additional"].has_key("color") {
            Some( data["additional"]["color"].as_str().expect("Couldn't load level JSON data: color") )
          } else { None },
          if data["additional"].has_key("triggers") {
            Some( data["additional"]["triggers"].members()
                  .map( |id| id.as_u32().expect("Couldn't load level JSON data: triggers id") )
                  .collect::<Vec<IdType>>() )
          } else { None }
        )
      } else { ( None, None, None, None ) };

      match data["type"].as_str().expect("Couldn't load level JSON data: type") {
        "Player" => {
          let err_msg = "Couldn't load level JSON data: Player";
          player_opt = Some(Player::new(ctx, point_opt.expect(err_msg), size_opt.expect(err_msg)));
        },

        "Child" => {
          let err_msg = "Couldn't load level JSON data: Child";
          children.push(Child::new(
              ctx,
              point_opt.expect(err_msg),
              size_opt.expect(err_msg),
              ChildType::Larry
          ));
        },

        "LarryChild" => {
          let err_msg = "Couldn't load level JSON data: Child Larry";
          children.push(Child::new(
              ctx,
              point_opt.expect(err_msg),
              size_opt.expect(err_msg),
              ChildType::Larry
          ));
        },
        "ThingChild" => {
          let err_msg = "Couldn't load level JSON data: Child Thing";
          children.push(Child::new(
              ctx,
              point_opt.expect(err_msg),
              size_opt.expect(err_msg),
              ChildType::Thing
          ));
        },
        "BloatChild" => {
          let err_msg = "Couldn't load level JSON data: Child Bloat";
          children.push(Child::new(
              ctx,
              point_opt.expect(err_msg),
              size_opt.expect(err_msg),
              ChildType::Bloat
          ));
        },

        "Wall" => {
          let err_msg = "Couldn't load level JSON data: Wall";
          walls.push(Wall::new(ctx, point_opt.expect(err_msg), size_opt.expect(err_msg)));
        }

        "JumpPadInteractable" => {
          let err_msg = "Couldn't load level JSON data: Interactable JumpPad";
          interactables.jump_pads.push(
            JumpPad::new(
              ctx,
              point_opt.expect(err_msg),
              size_opt.expect(err_msg)
            )
          );
        }

        "SwitchInteractable" => {
          let err_msg = "Couldn't load level JSON data: Interactable Switch";
          interactables.switches.push(
            Switch::new(
              ctx,
              point_opt.expect(err_msg),
              size_opt.expect(err_msg),
              id_opt.expect(err_msg),
              color_opt.expect(err_msg),
              triggers_opt.expect(err_msg)
            )
          );
        }

        "DoorInteractable" => {
          let err_msg = "Couldn't load level JSON data: Interactable Door";
          let state = match state_opt.expect("Couldn't load level JSON data: Interactable Door must have State") {
            "Open"    => door::State::Open,
            "Closed"  => door::State::Closed,
            "Opening" => door::State::Opening,
            "Closing" => door::State::Closing,
            _         => panic!("Interactable Door: Invalid state: {}", state_opt.unwrap())
          };
          interactables.doors.push(
            Door::new(
              ctx,
              point_opt.expect(err_msg),
              size_opt.expect(err_msg),
              id_opt.expect(err_msg),
              color_opt.expect(err_msg),
              state
            )
          );
        }

        "OneWayInteractable" => {
          let err_msg = "Couldn't load level JSON data: Interactable OneWay";
          interactables.one_ways.push(
            OneWay::new(
              ctx,
              point_opt.expect(err_msg),
              size_opt.expect(err_msg)
            )
          )
        }

        "SolidifierInteractable" => {
          let err_msg = "Couldn't load level JSON data: Interactable Solidifier";
          interactables.solidifiers.push(
            Solidifier::new(
              ctx,
              point_opt.expect(err_msg),
              size_opt.expect(err_msg)
            )
          )
        }

        "GoalInteractable" => {
          let err_msg = "Couldn't load level JSON data: Interactable Goal";
          interactables.goal = Some(Goal::new(
              ctx,
              point_opt.expect(err_msg),
              size_opt.expect(err_msg)
          ))
        }

        _ => {}
      }
    });

    let player = if let Some(player) = player_opt {
      player
    } else {
      return Err(ggez::GameError::from("Couldn't load player".to_string()));
    };

    let mut lvl = Level {
      window_rect: Rect::new(Point::new(0.0, 0.0), size.clone(), Origin::TopLeft),
      camera:      Camera::new(window_size.clone()),
      camera_rect: Rect::new(Point::new(0.0, 0.0), size, Origin::TopLeft),
      player,
      children,
      walls,
      interactables,
      toolbox:     ToolboxMenu::new(ctx, Point::new(0.0, window_size.h - 96.0), Size::new(window_size.w, 64.0)),
      dt:          Deltatime::new()
    };

    let point = lvl.player.center();
    lvl.camera.move_to(&point);

    Ok(lvl)
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
    for key in keycodes {
      self.player.key_down(key);
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

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.update_interactables(ctx)?;
    self.update_children(ctx)?;
    self.update_player(ctx)?;
    self.update_toolbox()?;
    self.dt.update();
    Ok(())
  }

  fn update_interactables(&mut self, ctx: &mut Context) -> GameResult<()> {
    for jump_pad in &mut self.interactables.jump_pads {
      if jump_pad.intersects(&self.player) {
        jump_pad.trigger_once(&mut self.player);
      } else {
        jump_pad.set_intersected(&self.player, false);
      }
      for child in &mut self.children {
        if jump_pad.intersects(child) {
          jump_pad.trigger_once(child);
        } else {
          jump_pad.set_intersected(&*child, false);
        }
      }
      jump_pad.update(ctx)?;
    }

    let mut door_ids_to_trigger: Vec<IdType> = Vec::new();

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
        door_ids_to_trigger.append(&mut switch.get_interactables_to_trigger()); }
      self.interactables.switches[i].interactables_triggered();
    }

    for door in &mut self.interactables.doors {
      if door_ids_to_trigger.contains(&door.id()) {
        door.trigger(&mut self.player);
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

  fn get_door_by_id_mut(&mut self, id: IdType) -> Option<&mut Door> {
    self.interactables.doors.iter_mut().find( |ref door| door.has_id(id) )
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
        ButtonType::NextLevel  => println!("NEXT LEVEL"),  // TODO
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
    self.toolbox.draw(ctx)?;
    self.toolbox.closeups.iter_mut()
      .for_each( |closeup| closeup.draw(ctx) );
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
}
