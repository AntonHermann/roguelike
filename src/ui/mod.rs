use crate::Result;
use roguelike_lib::*;
use crate::game::*;
use std::{
    io::{self, Write},
};
use termion::{
    raw::{IntoRawMode, RawTerminal},
    input::{TermRead, Keys},
    event::Key,
    AsyncReader,
};

mod map_renderer;
use self::map_renderer::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsMode {
    Help,
    Intro,
    Quit,
    PickRace,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Fullscreen(FsMode),
}

pub struct Ui {
    stdin_keys: Keys<AsyncReader>,
    stdout: RawTerminal<io::Stdout>,

    /// Which view is shown?
    mode: Mode,

    /// Runs the actual game
    engine: Engine,

    /// Renders the map and the objects on it
    map_renderer: MapRenderer,

    /// Whether some part of the game requested exit
    exit: bool,

    /// Whether the player spawned already
    player_spawned: bool,

    /// Whether something has changed since last redraw
    needs_redraw: bool,
}
impl Ui {
    pub fn new() -> Result<Self> {
        let stdin = termion::async_stdin();
        let stdin_keys = stdin.keys();
        let mut stdout = io::stdout().into_raw_mode()?;

        // Hide the cursor.
        write!(stdout, "{}", termion::cursor::Hide)?;

        let ui = Self {
            stdin_keys,
            stdout,
            mode: Mode::Fullscreen(FsMode::Intro),
            engine: Engine::new(),
            map_renderer: MapRenderer::new(),
            exit: false,
            player_spawned: false,
            needs_redraw: true,
        };
        Ok(ui)
    }
    pub fn run(&mut self) -> Result<()> {
        while !self.exit {
            self.engine.tick();
            self.maybe_redraw()?;
            self.handle_inputs()?;
        }
        Ok(())
    }
    fn mode_switch_to(&mut self, new_mode: Mode) {
        self.mode = new_mode;
        self.needs_redraw = true;
    }

    pub fn initial_spawn(&mut self, race: Race) {
        self.engine.initial_spawn(race);
        // let player_id = self.engine.current_location().player_id();
        // self.engine_change(player_id);
        // self.update_changes();
        self.player_spawned = true;
    }
}

// === DRAWING === //
impl Ui {
    pub fn maybe_redraw(&mut self) -> Result<()> {
        if self.needs_redraw {
            self.redraw()?;
            self.needs_redraw = false;
        }
        Ok(())
    }
    pub fn redraw(&mut self) -> Result<()> {
        match self.mode {
            Mode::Normal => {
                // TODO: distinguish between more modes
                // self.map_renderer.

                self.draw_map()?;
                // self.draw_log()
                // self.draw_stats()
            },
            Mode::Fullscreen(fs_mode) => {
                match fs_mode {
                    FsMode::Help => self.draw_help()?,
                    FsMode::Intro => self.draw_intro()?,
                    FsMode::Quit => self.draw_quit()?,
                    FsMode::PickRace => self.draw_pickrace()?,
                }
            },
        }
        self.stdout.flush()?;
        Ok(())
    }
    pub fn draw_intro(&mut self) -> Result<()> {
        use termion::{clear, cursor, style};
        write!(
            self.stdout,
            "{}{}This game is in early development{}Press {}?{} for ingame help",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Goto(5, 3),
            style::Bold,
            style::Reset
        )?;
        Ok(())
    }
    pub fn draw_help(&mut self) -> Result<()> {
        use termion::{clear, cursor};
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;

        let lines = &[
            "Move/attack: hjklui",
            "Strafe/attack: Shift + h/l",
            "Charge: c",
            "Wait: .",
            "Descend: >",
            "Go to: G (only '>' follow-up implemented)",
            "Examine: x",
            "Pick item in front: ,",
            "Look at Inventory: I",
            "Equip/Use: E",
            "Drop: D",
            "Ranged/Throw: f (not fully working)",
            "Quit: ESC/q",
        ];
        for line in lines {
            write!(self.stdout, "{}\r\n", line)?;
        }

        Ok(())
    }
    pub fn draw_pickrace(&mut self) -> Result<()> {
        use termion::{clear, cursor};
        write!(self.stdout, "{}{}Pick your race:", clear::All, cursor::Goto(1, 1))?;

        let races = &["a) Human", "b) Elf", "c) Dwarf"];
        for (i, race) in races.iter().enumerate() {
            write!(self.stdout, "{}{}", cursor::Goto(3, 3 + i as u16), race)?;
        }
        Ok(())
    }
    pub fn draw_quit(&mut self) -> Result<()> {
        use termion::{clear, cursor, style};

        write!(self.stdout, "{}{}{}Quit. Are you sure?{}",
            clear::All,
            cursor::Goto(1, 1),
            style::Bold,
            style::Reset
        )?;
        Ok(())
    }

    pub fn draw_map(&mut self) -> Result<()> {
        use termion::{clear, cursor};
        self.map_renderer.update_base(self.engine.tiles());
        self.map_renderer.update_actors();

        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            self.map_renderer
        )?;
        Ok(())
    }
}

// === INPUT HANDLING === //
impl Ui {
    fn handle_inputs(&mut self) -> Result<()> {
        loop {
            if let Some(k) = self.stdin_keys.next() {
                self.handle_input_key_mode_specific(k?);
            } else {
                return Ok(())
            }
        }
    }
    fn handle_input_key_mode_specific(&mut self, key: termion::event::Key) {
        match self.mode {
            Mode::Fullscreen(fs_mode) => match fs_mode {
                FsMode::Quit  => {
                    match key {
                        Key::Char('y') | Key::Char('Y') => self.exit = true,
                        _ => self.mode_switch_to(Mode::Normal),
                    }
                },
                FsMode::Intro => {
                    match key {
                        _ => self.mode_switch_to(Mode::Fullscreen(FsMode::PickRace)),
                    }
                },
                FsMode::PickRace => {
                    match key {
                        Key::Char('a') => { // Human
                            self.initial_spawn(Race::Human);
                            self.mode_switch_to(Mode::Normal);
                        },
                        Key::Char('b') => { // Elf
                            self.initial_spawn(Race::Elf);
                            self.mode_switch_to(Mode::Normal);
                        },
                        Key::Char('c') => { // Dwarf
                            self.initial_spawn(Race::Dwarf);
                            self.mode_switch_to(Mode::Normal);
                        },
                        _ => {},
                    }
                },
                _ => { self.mode_switch_to(Mode::Normal) },
            },
            Mode::Normal => {
                // y k u
                //  \|/
                // h-.-l
                //  /|\
                // b j n
                let mut walk = |d: Dir| {
                    let action = Action::PlayerMovement(d);
                    self.engine.enqueue_action(action);
                };
                // TODO: http://journal.stuffwithstuff.com/2014/07/15/a-turn-based-game-loop/
                // stores the action on the player instead of
                // inserting it to the engine action queue
                match key {
                    Key::Char('y') => walk(Dir::NW),
                    Key::Char('k') => walk(Dir::N ),
                    Key::Char('u') => walk(Dir::NE),
                    Key::Char('h') => walk(Dir::W),
                    Key::Char('l') => walk(Dir::E),
                    Key::Char('b') => walk(Dir::SW),
                    Key::Char('j') => walk(Dir::S ),
                    Key::Char('n') => walk(Dir::SE),
                    Key::Char('?') => {
                        self.mode_switch_to(Mode::Fullscreen(FsMode::Help))
                    },
                    Key::Char('q') | Key::Esc => {
                        self.mode_switch_to(Mode::Fullscreen(FsMode::Quit))
                    },
                    _ => {},
                }
            },
            _ => {}, // TODO:
        }
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show);
    }
}