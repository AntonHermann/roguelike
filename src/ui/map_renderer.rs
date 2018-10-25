use roguelike_lib::*;
use termion::color::Rgb as Color;
use crate::*;
use crate::game::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
struct DisplayCharacter {
    ch: char,
    bg: Option<Color>,
    fg: Option<Color>,
    bold: bool,
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct ExtendedCharacter {
    base: DisplayCharacter,
    // known: bool,
    // visible: bool,
    // light: u32,
}
impl Default for DisplayCharacter {
    fn default() -> Self {
        Self {
            ch: ' ',
            bg: None,
            fg: None,
            bold: false,
        }
    }
}
impl Default for ExtendedCharacter {
    fn default() -> Self {
        Self {
            base: Default::default()
        }
    }
}

pub struct MapRenderer {
    base: Map<ExtendedCharacter>,
    actors: Map<DisplayCharacter>,
}
impl MapRenderer {
    pub fn new() -> Self {
        let w = GAME_WIDTH;
        let h = GAME_HEIGHT;
        Self {
            base: Map::new(w, h),
            actors: Map::new(w, h),
        }
    }
    pub fn update_base(&mut self, tiles: &Map<Tile>) {
        for x in 0..tiles.width() {
            for y in 0..tiles.height() {
                let dc = DisplayCharacter {
                    ch: tiles[(x,y)].ch(),
                    .. Default::default()
                };
                let edc = ExtendedCharacter {
                    base: dc,
                    .. Default::default()
                };
                self.base[(x,y)] = edc;
            }
        }
    }
    pub fn update_actors(&mut self, actors: &ActorList) {
        for (pos, actor_id) in actors.actors_coord_to_id.iter() {
            let actor = actors.actors_by_id.get(actor_id)
                .expect("corrupted ActorList: actors_coord_to_id contains an actor actors_by_id doesn't know");
                // TODO: this currently doesn't take into account whether
                // the player can see the actor
            
            let fg = if actor.is_player() {
                Color(0, 255, 255)
            } else {
                Color(85, 85, 85)
            };

            let dc = DisplayCharacter {
                ch: race_to_char(actor.race),
                fg: Some(fg),
                bg: None,
                bold: false,
            };
            self.actors[*pos] = dc;
        }
    }
}
impl fmt::Display for MapRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use termion::{
            style,
            color::*,
        };

        let w = self.base.width();
        let h = self.base.height();
        write!(f, "w:{},h:{}\r\n", w, h)?;
        for y in 0..h {
            
            for x in 0..w {
                // TODO: include actors
                let base_edc = self.base[(x,y)];
                let bg = base_edc.base.bg.unwrap_or(Color(0,0,0));
                let fg = base_edc.base.fg.unwrap_or(Color(255,255,255));
                if base_edc.base.bold {
                    style::Bold.fmt(f)?;
                }
                write!(
                    f, "{}{}{}{}",
                    Bg(bg),
                    Fg(fg),
                    base_edc.base.ch,
                    style::Reset
                )?;
            }
            write!(f, "\r\n")?;
        }
        Ok(())
    }
}

pub fn race_to_char(race: Race) -> char {
    match race {
        Race::Human | Race::Elf | Race::Dwarf => '@',
        // Race::Rat =>  'r',
        // Race::Goblin => 'g',
        // Race::Troll => 'T',
    }
}