extern crate roguelike_lib;
#[allow(unused_imports)]
use roguelike_lib::*;
extern crate termion;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub const GAME_WIDTH:  i32 = 80;
pub const GAME_HEIGHT: i32 = 24;

mod logging;
mod ui;
mod game;

use crate::ui::Ui;
#[allow(unused_imports)]
use log::*;

fn main() -> Result<()> {
    logging::init()?;

    let mut ui = Ui::new()?;

    ui.run()
}