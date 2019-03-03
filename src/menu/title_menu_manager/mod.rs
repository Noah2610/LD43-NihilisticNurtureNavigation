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

#[derive(Debug)]
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

  pub fn load_thank_you(&mut self, ctx: &mut Context, window_size: &Size) -> GameResult<()> {
    self.thank_you = Some(ThankYouMenu::new(ctx, window_size)?);
    self.current = MenuType::ThankYou;
    Ok(())
  }

  pub fn show_level_select(&mut self) {
    self.title.show_level_select();
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
    if let Some(clicked) = self.get_clicked().clone() {
      match clicked {
        TitleLevelSelect    => self.current    = MenuType::LevelSelect,
        LevelSelectBack     => self.current    = MenuType::Title,
        LevelSelectLevel(i) => self.load_level = Some(i),
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
