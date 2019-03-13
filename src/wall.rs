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

fn image_filename_for_level(n: usize) -> String {
  const DEFAULT: &str = "wall_default.png";
  let chapter = n / 5;
  let sub = n % 5;
  match chapter {
    0 => match sub {
      0 => "wall_1.1.png",
      1 => "wall_1.2.png",
      2 => "wall_1.3.png",
      3 => "wall_1.4.png",
      4 => "wall_1.5.png",
      _ => DEFAULT,
    },
    1 => match sub {
      0 => "wall_2.1.png",
      1 => "wall_2.2.png",
      2 => "wall_2.3.png",
      3 => "wall_2.4.png",
      4 => "wall_2.5.png",
      _ => DEFAULT,
    },
    2 => match sub {
      0 => "wall_3.1.png",
      1 => "wall_3.2.png",
      2 => "wall_3.3.png",
      3 => "wall_3.4.png",
      4 => "wall_3.5.png",
      _ => DEFAULT,
    },
    _ => DEFAULT,
  }.to_string()
}

impl Walls {
  pub fn new(ctx: &mut Context, level_index: usize) -> Self {
    let image_filepath = &::join_str(IMAGES, &image_filename_for_level(level_index));
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
