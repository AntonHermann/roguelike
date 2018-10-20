extern crate roguelike_lib;
#[allow(unused_imports)]
use roguelike_lib::*;
extern crate termion;

pub type Result<T> = std::result::Result<T, failure::Error>;

mod logging;
mod stage1;

use termion::{
    input::TermRead,
    event::Key,
    raw::IntoRawMode,
    clear,
    cursor,
};
use std::io::{Write, stdin, stdout};
use log::*;

fn main() -> Result<()> {
    logging::init_logger()?;
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    // Hide the cursor.
    write!(stdout, "{}", termion::cursor::Hide)?;
    stdout.flush()?;

    let mut stage1 = stage1::Stage1::new();

    write!(stdout, "{}{}{}",
        cursor::Goto(1,1),
        clear::All,
        stage1)?;

    // GAME LOOP
    for c in stdin.keys() {

        let c = c?;
        if c == Key::Esc || c == Key::Char('q') {
            debug!("{:?} pressed, shutdown", c);
            break;
        } else {
            stage1.handle_key(c)?;
        }

        write!(
            stdout, "{}{}{}",
            cursor::Goto(1,1),
            clear::All,
            stage1
        )?;
        stdout.flush()?;
    }

    write!(stdout, "{}", termion::cursor::Show)?;
    stdout.flush()?;

    Ok(())
}