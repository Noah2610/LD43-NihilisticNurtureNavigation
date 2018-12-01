use ggez::{
  GameResult,
  Context,
  graphics::{ self, Image }
};

use noframe::geo::prelude::*;
use noframe::entity::Entity;

use settings::wall::*;

pub struct Wall {
  point:  Point,
  size:   Size,
  origin: Origin,
  image:  Image
}

impl Wall {
  pub fn new(ctx: &mut Context, point: Point, size: Size, image_filenames: Vec<&str>) -> Self {
    let image_filepath = &::join_str(IMAGES, image_filenames[0]);
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

impl Entity for Wall {
  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    let dest_point = graphics::Point2::from(self.point());
    graphics::draw(ctx, &self.image, dest_point, 0.0)?;
    Ok(())
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    let dest_point = graphics::Point2::from(
      &Point::combine(vec![self.point(), offset])
    );
    let param = graphics::DrawParam { dest: dest_point, scale: graphics::Point2::from(&self.scale()) , .. Default::default() };
    graphics::draw_ex(ctx, &self.image, param)?;
    Ok(())
  }
}
