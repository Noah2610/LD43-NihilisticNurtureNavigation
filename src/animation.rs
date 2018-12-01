use std::time::{ Instant, Duration };

use ggez::{
  GameResult,
  Context,
  graphics::{ self, Image }
};

use noframe::geo::prelude::*;

pub struct Animation {
  images:                    Vec<Image>,
  image_index:               usize,
  image_update_intervals_ms: Vec<u64>,
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
      last_update: Instant::now()
    }
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
      self.image_index = 0;
    } else {
      self.image_index += 1;
    }
  }

  pub fn draw(&self, ctx: &mut Context, point: &Point, size: &Size) -> GameResult<()>{
    let image = self.current_image();
    let dest = graphics::Point2::from(point);
    let scale = Point::new(
      size.w / image.width()  as NumType,
      size.h / image.height() as NumType
    );
    let param = graphics::DrawParam {
      dest,
      scale: graphics::Point2::from(&scale),
      .. Default::default()
    };
    graphics::draw_ex(ctx, image, param)
  }

  pub fn draw_offset(&self, ctx: &mut Context, point: &Point, size: &Size, offset: &Point) -> GameResult<()> {
    let image = self.current_image();
    let dest = graphics::Point2::from(
      &Point::combine(vec![point, offset])
    );
    let scale = Point::new(
      size.w / image.width()  as NumType,
      size.h / image.height() as NumType
    );
    let param = graphics::DrawParam {
      dest,
      scale: graphics::Point2::from(&scale),
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
