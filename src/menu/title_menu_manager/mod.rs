pub mod prelude {
  pub use super::TitleMenuManager;
  pub use super::title::TitleMenu;
  pub use super::level_select::LevelSelectMenu;
}

pub mod title;
pub mod level_select;

use ggez::{
  Context,
  GameResult,
};
use noframe::geo::prelude::*;

use self::title::TitleMenu;
use self::level_select::LevelSelectMenu;
use menu::prelude::*;

#[derive(Debug)]
enum MenuType {
  Title,
  LevelSelect,
}

pub struct TitleMenuManager {
  current:           MenuType,
  pub load_level:    Option<usize>,
  pub title:         TitleMenu,
  pub level_select:  LevelSelectMenu,
}

impl TitleMenuManager {
  pub fn new(ctx: &mut Context, size: Size) -> GameResult<Self> {
    Ok(Self {
      current:           MenuType::Title,
      load_level:        None,
      title:             TitleMenu::new(ctx, size.clone()),
      level_select:      LevelSelectMenu::new(ctx, size.clone())?,
    })
  }

  pub fn show_level_select(&mut self) {
    self.title.show_level_select();
  }

  pub fn get_clicked(&self) -> &Option<ButtonType> {
    match &self.current {
      MenuType::Title       => self.title.get_clicked(),
      MenuType::LevelSelect => self.level_select.get_clicked(),
    }
  }

  pub fn mouse_down(&mut self, x: i32, y: i32) {
    match &mut self.current {
      MenuType::Title       => self.title.mouse_down(x, y),
      MenuType::LevelSelect => self.level_select.mouse_down(x, y),
    };
  }

  pub fn update(&mut self) -> GameResult<()> {
    if let Some(clicked) = self.get_clicked().clone() {
      match clicked {
        ButtonType::TitleLevelSelect    => self.current    = MenuType::LevelSelect,
        ButtonType::LevelSelectBack     => self.current    = MenuType::Title,
        ButtonType::LevelSelectLevel(i) => self.load_level = Some(i),
        _                            => (),
      };
    }
    match &self.current {
      MenuType::Title       => self.title.update()?,
      MenuType::LevelSelect => self.level_select.update()?,
    };
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    match &self.current {
      MenuType::Title       => self.title.draw(ctx)?,
      MenuType::LevelSelect => self.level_select.draw(ctx)?,
    };
    Ok(())
  }
}
