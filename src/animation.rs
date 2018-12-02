use std::time::{ Instant, Duration };

use ggez::{
  GameResult,
  Context,
  graphics::{ self, Image }
};

use noframe::geo::prelude::*;

pub enum Facing {
  Left,
  Right
}

impl Facing {
  pub fn num(&self) -> i8 {
    match self {
      Facing::Right =>  1,
      Facing::Left  => -1
    }
  }

  pub fn default() -> Self {
    Facing::Right
  }

  pub fn offset_for_draw_param(&self) -> f32 {
    match self {
      Facing::Right => 0.0,
      Facing::Left  => 1.0
    }
  }
}

pub struct Animation {
  images:                    Vec<Image>,
  image_index:               usize,
  image_update_intervals_ms: Vec<u64>,
  times_played:              u32,
  last_update:               Instant,
}

impl Animation {
  pub fn new(ctx: &mut Context, image_filepaths: Vec<String>, image_update_intervals_ms: Vec<u64>) -> Self {
    let images = image_filepaths.iter().map( |filepath| {
      Image::new(ctx, filepath).expect(&format!("Couldn't load image: {}", filepath))
    }).collect::<Vec<Image>>();

    Self {
      images,
      image_index: 0,
      image_update_intervals_ms,
      times_played: 0,
      last_update: Instant::now()
    }
  }

  pub fn played(&self) -> u32 {
    self.times_played
  }

  pub fn reset(&mut self) {
    self.image_index = 0;
    self.times_played = 0;
  }

  pub fn update(&mut self) -> GameResult<()> {
    let now = Instant::now();
    if now - self.last_update < Duration::from_millis(self.current_update_interval()) {
      return Ok(());
    }
    self.next_image();
    self.last_update = now;
    Ok(())
  }

  fn next_image(&mut self) {
    if self.image_index >= self.images.len() - 1 {
      self.times_played += 1;
      self.image_index = 0;
    } else {
      self.image_index += 1;
    }
  }

  pub fn draw(&self, ctx: &mut Context, point: &Point, size: &Size, facing: &Facing) -> GameResult<()>{
    let image = self.current_image();
    let dest = graphics::Point2::from(point);
    let scale = Point::new(
      size.w / image.width()  as NumType * facing.num() as NumType,
      size.h / image.height() as NumType
    );
    let param = graphics::DrawParam {
      dest,
      scale: graphics::Point2::from(&scale),
      offset: graphics::Point2::new(facing.offset_for_draw_param(), 0.0),
      .. Default::default()
    };
    graphics::draw_ex(ctx, image, param)
  }

  pub fn draw_offset(&self, ctx: &mut Context, point: &Point, size: &Size, facing: &Facing, offset: &Point) -> GameResult<()> {
    let image = self.current_image();
    let dest = graphics::Point2::from(
      &Point::combine(vec![point, offset])
    );
    let scale = Point::new(
      size.w / image.width()  as NumType * facing.num() as NumType,
      size.h / image.height() as NumType
    );
    let param = graphics::DrawParam {
      dest,
      scale: graphics::Point2::from(&scale),
      offset: graphics::Point2::new(facing.offset_for_draw_param(), 0.0),
      .. Default::default()
    };
    graphics::draw_ex(ctx, image, param)
  }

  fn current_image(&self) -> &Image {
    &self.images.get(self.image_index).expect("image_index shouldn't be out of bounds")
  }

  fn current_update_interval(&self) -> u64 {
    *self.image_update_intervals_ms.get(self.image_index).expect("image_index shouldn't be out of bounds")
  }
}
