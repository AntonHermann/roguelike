use std::collections::{HashMap, HashSet};
use roguelike_lib::*;
use super::Actor;

pub type ActorId = u32;

#[derive(Clone, Debug)]
pub struct ActorList {
    // id -> State
    pub actors_by_id: HashMap<ActorId, Actor>,
    // coord -> id
    pub actors_coord_to_id: HashMap<Cell, ActorId>,
    pub actors_dead: HashSet<ActorId>,
    pub actors_counter: ActorId,
    player_id: Option<ActorId>,
}
impl ActorList {
    pub fn new() -> Self {
        // let (map, gen_actors, items) = generate::gen_level(level);
        // let mut actors: HashMap<u32, Actor> = Default::default();
        // let mut actors_pos: HashMap<Cell, u32> = Default::default();
        // let mut actors_counter: ActorId = 0u32;
        // for (coord, astate) in gen_actors {
        //     actors_pos.insert(coord, actors_counter);
        //     actors.insert(actors_counter, astate);
        //     actors_counter += 1;
        // }

        Self {
            actors_by_id:       Default::default(),
            actors_coord_to_id: Default::default(),
            actors_dead:        Default::default(),
            actors_counter:     0,
            player_id:          None,
        }
    }
    pub fn player_id(&self) -> ActorId {
        self.player_id.unwrap()
    }
    pub fn player(&self) -> &Actor {
        &self.actors_by_id[&self.player_id.unwrap()]
    }

    pub fn actors_ids(&self) -> Vec<u32> {
        self.actors_by_id.keys().cloned().collect()
    }
    pub fn actors_alive_ids(&self) -> Vec<u32> {
        self.actors_by_id.keys().filter(|&id| !self.actors_by_id[id].is_dead()).cloned().collect()
    }

    pub fn spawn(&mut self, mut astate: Actor, pos: Cell) -> ActorId {
        if self.actors_coord_to_id.contains_key(&pos) {
            // TODO: Find an alternative place
            unimplemented!();
        }
        let id = self.actors_counter;
        self.actors_counter += 1;

        debug_assert!(!self.actors_coord_to_id.contains_key(&pos));
        self.actors_coord_to_id.insert(pos, id);
        // astate.pre_spawn(self);
        self.actors_by_id.insert(id, astate);
        // self.recalculate_light_map();
        // self.recalculate_noise();

        // self.pre_any_tick();
        // let mut astate = self.actors_by_id.remove(&id).unwrap();
        // astate.post_spawn(self);
        // self.actors_by_id.insert(id, astate);
        // self.post_any_tick();

        id
    }

    // pub fn remove(&mut self, id: ActorId) -> Option<Actor> {
    //     let actor = self.actors_by_id.remove(&id);
    //     let actor = if let Some(actor) = actor {
    //         actor
    //     } else {
    //         return None;
    //     };
    // FIXME:
    //     self.actors_coord_to_id.remove(&actor.pos.coord);
    //     Some(actor)
    // }

    pub fn spawn_player(&mut self, actor: Actor, pos: Cell) -> ActorId {
        assert!(actor.is_player());
        self.player_id = Some(self.spawn(actor, pos));
        self.player_id.unwrap()
    }
}