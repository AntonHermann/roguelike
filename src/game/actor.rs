use roguelike_lib::*;
use super::consts::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub max_hp: i32,
}
impl Stats {
    pub fn new(race: Race) -> Self {
        match race {
            Race::Human => HUMAN_STATS,
            Race::Elf   => ELF_STATS,
            Race::Dwarf => DWARF_STATS,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Actor {
    pub race: Race,
    pub hp: i32,
    pub is_player: bool,
    // pub pos: Cell,
    pub base_stats: Stats,
}
impl Actor {
    pub fn new(race: Race) -> Self {
        let base_stats = Stats::new(race);

        Self {
            race,
            hp: base_stats.max_hp,
            is_player: false,
            base_stats
        }
    }
    pub fn _new_player(race: Race) -> Self {
        Self {
            is_player: true,
            .. Self::new(race)
        }
    }
    pub fn is_player(&self) -> bool {
        self.is_player
    }
    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}