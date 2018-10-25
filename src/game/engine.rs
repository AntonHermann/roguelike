// use std::collections::VecDeque;
use roguelike_lib::*;
use super::*;
use crate::*;

#[derive(Debug)]
pub struct Engine {
    tiles: Map<Tile>,
    actor_list: ActorList,
    // action_queue: VecDeque<Action>,
}
impl Engine {
    pub fn new() -> Self {
        let tiles = Map::new_with_border(
            GAME_WIDTH,
            GAME_HEIGHT,
            Tile::Floor,
            Tile::Wall,
        );
        let actor_list = ActorList::new();
        Self {
            tiles,
            actor_list,
            // action_queue: VecDeque::new(),
        }
    }

    pub fn is_cell_occupied(&self, pos: Cell) -> bool {
        self.tiles[pos] != Tile::Floor || // anything except floor counts as occupied
        self.actor_list.actors_coord_to_id.contains_key(&pos)
    }
    pub fn initial_spawn(&mut self, race: Race) {
        let player = Actor::new(race);

        let mut rng = rand_utils::create_rng("ABC");

        // loop until we got a free cell
            // TODO: maybe add a panic after `k` iterations,
            // so this can't just "loop forever" if there are no free cells?
        let pos = loop {
            let pos = rand_utils::random_pos(GAME_WIDTH, GAME_HEIGHT, &mut rng);
            if !self.is_cell_occupied(pos) {
                break pos;
            }
        };

        self.actor_list.spawn_player(player, pos);
    }
    pub fn tick(&mut self) {
        // FIXME:
        unimplemented!()
    }
    pub fn enqueue_action(&mut self, action: Action) {
        // FIXME:
        unimplemented!()
        // self.action_queue.push_back(action);
    }
    pub fn tiles(&self) -> &Map<Tile> {
        &self.tiles
    }
}

#[derive(Debug)]
pub enum Action {
    PlayerMovement(Dir),
}