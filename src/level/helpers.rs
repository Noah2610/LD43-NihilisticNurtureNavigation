use std::fs::File;
use std::io::prelude::*;

use ggez::{
  GameResult,
  Context,
  graphics,
};
use noframe::deltatime::Deltatime;
use noframe::geo::prelude::*;
use noframe::camera::Camera;

use super::Level;
use settings::level::*;
use settings::res;
use id_generator::IdType;
use interactables::prelude::*;
use persons::player::Player;
use persons::children::{ Child, ChildType };
use wall::{ Wall, Walls };
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

pub fn new_level(ctx: &mut Context, window_size: Size, filename: &str, level_index: usize) -> GameResult<Level> {
  let font = graphics::Font::new(ctx, res::fonts::DEFAULT, NAME_FONT_SIZE)?;
  let level_name = ::semantic(filename);
  let level_name_text = graphics::Text::new(ctx, &::semantic(filename), &font)?;
  let score_text = graphics::Text::new(ctx, "Score: 0", &font)?;

  let filename = &::join_str(filename, ".json");
  let level_filepath = &::join_str(res::LEVELS, filename);
  let mut level_file = File::open(level_filepath)?;
  let mut json_raw = String::new();
  level_file.read_to_string(&mut json_raw)?;
  let data = match json::parse(&json_raw) {
    Ok(d)  => d,
    Err(e) => return Err(ggez::GameError::from(e.to_string()))
  };

  let (player, children, walls, interactables) = load_json(ctx, &data, level_index)?;
  let toolbox = ToolboxMenu::new(
    ctx,
    Point::new(0.0, window_size.h - 96.0),
    Size::new(window_size.w, 64.0),
    children.iter().map( |c| c.child_type.clone() ).collect()
  );

  let mut lvl = Level {
    json_data:   data,
    level_index,
    window_rect: Rect::new(Point::new(0.0, 0.0), window_size.clone(), Origin::TopLeft),
    camera:      Camera::new(window_size.clone()),
    camera_rect: Rect::new(Point::new(0.0, 0.0), window_size.clone(), Origin::TopLeft),
    player,
    children,
    walls,
    interactables,
    toolbox,
    next_level:  false,
    font,
    level_name,
    level_name_text,
    score:       Score::new(),
    prev_score:  0,
    score_text,
    dt:          Deltatime::new()
  };

  let point = lvl.player.center();
  lvl.camera.move_to(&point);

  Ok(lvl)
}

pub fn load_json(ctx: &mut Context, data: &json::JsonValue, level_index: usize) -> GameResult<(Player, Vec<Child>, Walls, InteractablesContainer)> {
  let mut player_opt = None;
  let mut children = Vec::new();
  let mut walls = Walls::new(ctx, level_index);
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
    let ( state_opt, id_opt, color_opt, triggers_opt, strength_opt ) = if data.has_key("additional") {
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
        } else { None },
        if data["additional"].has_key("strength") {
          Some( data["additional"]["strength"].as_f32().expect("Couldn't load level JSON data: strength") )
        } else { None }
      )
    } else { ( None, None, None, None, None ) };

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
        let state = match state_opt.expect("Couldn't load level JSON data: Interactable JumpPad must have State") {
          "Active"   => jump_pad::State::Active,
          "Inactive" => jump_pad::State::Inactive,
          "Trigger"  => jump_pad::State::Trigger,
          _          => panic!("Interactable JumpPad: Invalid state: {}", state_opt.unwrap())
        };
        interactables.jump_pads.push(
          JumpPad::new(
            ctx,
            point_opt.expect(err_msg),
            size_opt.expect(err_msg),
            id_opt.expect(err_msg),
            color_opt.expect(err_msg),
            state,
            strength_opt
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

  Ok((player, children, walls, interactables))
}
