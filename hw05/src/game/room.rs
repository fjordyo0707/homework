use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {
    // TODO: Implement the necessary methods for Rooms.
    pub fn new() -> Self {
        Room {
            name: "".to_string(),
            contents: Vec::new(),
            halls: Vec::new(),
            wumpus: false,
        }
    }

    pub fn neighbors_string(&self) -> String {
        // TODO: Implement
        // This place is referred from 
        // https://github.com/blogscot/the-darkest-dungeon/blob/master/src/game/room.rs 
        // need to familiar with the usage!!!
        self.halls.iter().map(
            |hall| hall.right.borrow().name.clone()
        ).collect::<Vec<String>>().join(", ")
    }
}
