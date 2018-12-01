use ggez::{
  GameResult,
  Context,
  graphics::{ self, Image },
  event::Keycode
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;
use noframe::deltatime::Deltatime;

use settings::player::*;

#[derive(PartialEq)]
enum Axis {
  X,
  Y
}

pub struct Player {
  point:        Point,
  size:         Size,
  origin:       Origin,
  image:        Image,
  velocity:     Point,
  max_velocity: Point,
  has_moved:    Vec<Axis>,
  dt:           Deltatime
}

impl Player {
  pub fn new(ctx: &mut Context, point: Point, size: Size, image_filename: &str) -> Self {
    let image_filepath = &::join_str(IMAGES, image_filename);
    let image = Image::new(ctx, image_filepath).expect(
      &format!("Couldn't load image for player: {}", image_filepath)
    );
    Self {
      point,
      size,
      origin: Origin::TopLeft,
      image,
      velocity: Point::new(0.0, 0.0),
      max_velocity: Point::new(MAX_SPEED, MAX_JUMP_SPEED),
      has_moved: Vec::new(),
      dt: Deltatime::new()
    }
  }

  pub fn keys_pressed(&mut self, keycodes: &Vec<Keycode>) {
    for keycode in keycodes {
      if let Some(point) = match keycode {
        &controls::LEFT => {
          if !self.has_moved(Axis::X) {
            self.moved_on_axis(Axis::X);
            Some(Point::new( -SPEED_INCREASE, 0.0 ))
          } else { None }
        }
        &controls::RIGHT => {
          if !self.has_moved(Axis::X) {
            self.moved_on_axis(Axis::X);
            Some(Point::new( SPEED_INCREASE, 0.0 ))
          } else { None }
        }
        &controls::JUMP => {
          if !self.has_moved(Axis::Y) {
            self.moved_on_axis(Axis::Y);
            Some(Point::new( 0.0, SPEED_INCREASE ))
          } else { None }
        }
        _ => None
      } {
        self.add_velocity(&point);
      }
    }
  }

  fn moved_on_axis(&mut self, axis: Axis) {
    if !self.has_moved.iter().any( |a| &axis == a ) {
      self.has_moved.push(axis);
    }
  }

  fn has_moved(&self, axis: Axis) -> bool {
    self.has_moved.iter().any( |a| &axis == a )
  }

  fn handle_velocity(&mut self) {
    let decr = SPEED_DECREASE; //* self.dt.secs();
    let decr_vel = Point::new(
      if !self.has_moved(Axis::X) {
        decr
      } else { 0.0 },
      if !self.has_moved(Axis::Y) {
        decr
      } else { 0.0 }
    );
    self.decrease_velocity(&decr_vel);
    self.has_moved.clear();
  }
}

impl Mask for Player {
  fn point(&self) -> &Point {
    &self.point
  }
  fn point_mut(&mut self) -> &mut Point {
    &mut self.point
  }
  fn size(&self) -> &Size {
    &self.size
  }
  fn origin(&self) -> &Origin {
    &self.origin
  }
}

impl Entity for Player {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.handle_velocity();
    self.dt.update();
    Ok(())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    let dest_point = graphics::Point2::from(self.point());
    graphics::draw(ctx, &self.image, dest_point, 0.0)?;
    Ok(())
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    let dest_point = graphics::Point2::from(
      &Point::combine(vec![self.point(), offset])
    );
    graphics::draw(ctx, &self.image, dest_point, 0.0)?;
    Ok(())
  }
}

impl Velocity for Player {
  fn velocity(&self) -> &Point {
    &self.velocity
  }
  fn usable_velocity(&self) -> Point {
    self.velocity.mult_axes_by(self.dt.secs())
  }
  fn velocity_mut(&mut self) -> &mut Point {
    &mut self.velocity
  }
  fn max_velocity(&self) -> Point {
    self.max_velocity.clone()
  }
}

impl Movement for Player { }
