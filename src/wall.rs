use ggez::{
  GameResult,
  Context,
  graphics::{ self, Image, DrawParam, spritebatch::SpriteBatch },
};

use noframe::geo::prelude::*;

use settings::wall::*;

pub struct Walls {
  pub walls:   Vec<Wall>,
  image_size:  Size,
  spritebatch: SpriteBatch,
}

impl Walls {
  pub fn new(ctx: &mut Context) -> Self {
    let image_filepath = &::join_str(IMAGES, "wall_simple.png");
    let image = Image::new(ctx, image_filepath).expect(
      &format!("Couldn't load image for wall: {}", image_filepath)
    );
    Self {
      walls:       Vec::new(),
      image_size:  Size::new(image.width() as NumType, image.height() as NumType),
      spritebatch: SpriteBatch::new(image),
    }
  }

  pub fn push(&mut self, wall: Wall) {
    self.walls.push(wall);
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    for wall in &self.walls {
      let mut param = wall.draw_param();
      param.scale = graphics::Point2::from(&self.scale_for(wall));
      self.spritebatch.add(param);
    }
    let dest_point = graphics::Point2::new(0.0, 0.0);
    graphics::draw(ctx, &self.spritebatch, dest_point, 0.0)?;
    self.spritebatch.clear();
    Ok(())
  }

  pub fn draw_offset(&mut self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    for wall in &self.walls {
      let mut param = wall.draw_param();
      param.scale = graphics::Point2::from(&self.scale_for(wall));
      self.spritebatch.add(param);
    }
    let dest_point = graphics::Point2::from(offset);
    let param = DrawParam { dest: dest_point, .. Default::default() };
    graphics::draw_ex(ctx, &self.spritebatch, param)?;
    self.spritebatch.clear();
    Ok(())
  }

  fn scale_for(&self, wall: &Wall) -> Point {
    Point::new(
      wall.size().w / self.image_size.w,
      wall.size().h / self.image_size.h
    )
  }
}

pub struct Wall {
  point:  Point,
  size:   Size,
  origin: Origin,
}

impl Wall {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point,
      size,
      origin: Origin::TopLeft,
    }
  }

  pub fn draw_param(&self) -> DrawParam {
    DrawParam {
      dest:  graphics::Point2::from(self.point()),
      .. Default::default()
    }
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
