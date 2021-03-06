pub mod prelude {
  pub use super::TitleMenuManager;
  pub use super::title::TitleMenu;
  pub use super::level_select::LevelSelectMenu;
  pub use super::thank_you::ThankYouMenu;
}

pub mod title;
pub mod level_select;
pub mod thank_you;

use ggez::{
  Context,
  GameResult,
};
use noframe::geo::prelude::*;

use self::title::TitleMenu;
use self::level_select::LevelSelectMenu;
use self::thank_you::ThankYouMenu;
use menu::prelude::*;

#[derive(Debug, PartialEq)]
enum MenuType {
  Title,
  LevelSelect,
  ThankYou,
}

pub struct TitleMenuManager {
  current:           MenuType,
  pub load_level:    Option<usize>,
  pub title:         TitleMenu,
  pub level_select:  LevelSelectMenu,
  pub thank_you:     Option<ThankYouMenu>,
}

impl TitleMenuManager {
  pub fn new(ctx: &mut Context, window_size: Size) -> GameResult<Self> {
    Ok(Self {
      current:           MenuType::Title,
      load_level:        None,
      title:             TitleMenu::new(ctx, window_size.clone()),
      level_select:      LevelSelectMenu::new(ctx, window_size.clone())?,
      thank_you:         None,
    })
  }

  pub fn in_title_menu(&self) -> bool {
    self.current == MenuType::Title
  }

  pub fn in_level_select_menu(&self) -> bool {
    self.current == MenuType::LevelSelect
  }

  pub fn in_thank_you_menu(&self) -> bool {
    self.current == MenuType::ThankYou
  }

  pub fn to_title_menu(&mut self) {
    self.current = MenuType::Title;
  }

  pub fn to_level_select_menu(&mut self) {
    self.current = MenuType::LevelSelect;
  }

  pub fn to_thank_you_menu(&mut self) {
    self.current = MenuType::ThankYou;
  }

  pub fn load_thank_you(&mut self, ctx: &mut Context, window_size: &Size) -> GameResult<()> {
    if self.thank_you.is_none() {
      self.thank_you = Some(ThankYouMenu::new(ctx, window_size)?);
    }
    self.current = MenuType::ThankYou;
    Ok(())
  }

  pub fn show_level_select(&mut self) {
    self.title.show_level_select();
  }

  pub fn is_level_select_available(&self) -> bool {
    self.title.is_level_select_available()
  }

  pub fn get_clicked(&self) -> &Option<ButtonType> {
    use self::MenuType::*;
    match &self.current {
      Title       => self.title.get_clicked(),
      LevelSelect => self.level_select.get_clicked(),
      ThankYou    => if let Some(ty) = &self.thank_you {
        ty.get_clicked()
      } else { &None }
    }
  }

  pub fn mouse_down(&mut self, x: i32, y: i32) {
    use self::MenuType::*;
    match &mut self.current {
      Title       => self.title.mouse_down(x, y),
      LevelSelect => self.level_select.mouse_down(x, y),
      ThankYou    => if let Some(ty) = &mut self.thank_you {
        ty.mouse_down(x, y);
      }
    };
  }

  pub fn update(&mut self) -> GameResult<()> {
    use self::ButtonType::*;
    use self::MenuType::*;
    let mut new_current = None;
    if let Some(clicked) = self.get_clicked().clone() {
      match clicked {
        TitleLevelSelect    => new_current = Some(MenuType::LevelSelect),
        LevelSelectBack     => new_current = Some(MenuType::Title),
        LevelSelectLevel(i) => self.load_level = Some(i),
        ThankYouBack        => new_current = Some(MenuType::Title),
        _                   => (),
      };
    }
    match &self.current {
      Title       => self.title.update()?,
      LevelSelect => self.level_select.update()?,
      ThankYou    => if let Some(ty) = &mut self.thank_you {
        ty.update()?;
      },
    };
    if let Some(new) = new_current {
      self.current = new;
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    use self::MenuType::*;
    match &self.current {
      Title       => self.title.draw(ctx)?,
      LevelSelect => self.level_select.draw(ctx)?,
      ThankYou    => if let Some(ty) = &mut self.thank_you {
        ty.draw(ctx)?;
      },
    };
    Ok(())
  }
}
