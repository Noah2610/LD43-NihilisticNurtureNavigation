use ggez::{
  GameResult,
  Context,
  graphics::{ self, Image, DrawParam, spritebatch::SpriteBatch },
};

use noframe::geo::prelude::*;
use noframe::entity::Entity;

use settings::wall::*;

pub struct Walls {
  pub walls:       Vec<Wall>,
  spritebatch: SpriteBatch,
}

impl Walls {
  pub fn new(ctx: &mut Context) -> Self {
    let image_filepath = &::join_str(IMAGES, "tile4.1.1.png");
    let image = Image::new(ctx, image_filepath).expect(
      &format!("Couldn't load image for wall: {}", image_filepath)
    );
    Self {
      walls:       Vec::new(),
      spritebatch: SpriteBatch::new(image),
    }
  }

  pub fn push(&mut self, wall: Wall) {
    self.walls.push(wall);
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    for wall in &mut self.walls {
      self.spritebatch.add(wall.draw_param());
    }
    let dest_point = graphics::Point2::new(0.0, 0.0);
    graphics::draw(ctx, &self.spritebatch, dest_point, 0.0)?;
    self.spritebatch.clear();
    Ok(())
  }

  pub fn draw_offset(&mut self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    for wall in &mut self.walls {
      self.spritebatch.add(wall.draw_param());
    }
    let dest_point = graphics::Point2::from(offset);
    let param = DrawParam { dest: dest_point, .. Default::default() };
    graphics::draw_ex(ctx, &self.spritebatch, param)?;
    self.spritebatch.clear();
    Ok(())
  }
}

pub struct Wall {
  point:  Point,
  size:   Size,
  origin: Origin,
  image:  Image
}

impl Wall {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    let image_filepath = &::join_str(IMAGES, "tile4.1.1.png");
    let image = Image::new(ctx, image_filepath).expect(
      &format!("Couldn't load image for wall: {}", image_filepath)
    );
    Self {
      point,
      size,
      origin: Origin::TopLeft,
      image
    }
  }

  pub fn draw_param(&self) -> DrawParam {
    DrawParam {
      dest:  graphics::Point2::from(self.point()),
      scale: graphics::Point2::from(&self.scale()),
      .. Default::default()
    }
  }

  fn scale(&self) -> Point {
    Point::new(
      self.size().w / self.image.width()  as NumType,
      self.size().h / self.image.height() as NumType
    )
  }
}

impl Mask for Wall {
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
