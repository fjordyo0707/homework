use std;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use super::curio::Curio;
use super::room::Room;

const MAX_HP: i32 = 25;

pub enum Command {
    Go(String),
    Shoot(String),
}

pub struct Player {
    pub location: Rc<RefCell<Room>>,
    pub hp: i32,
    pub gold: i32,
    won: bool,
}

impl Player {
    pub fn new(location: Rc<RefCell<Room>>) -> Player {
        Player {
            location: location,
            hp: MAX_HP,
            gold: 0,
            won: false,
        }
    }

    pub fn use_curio(&mut self, curio: Curio) {
        match curio {
            Curio::Chest(gold) => {
                println!("You open the chest and gain {} gold.", gold);
                self.gold += gold;
            },
            Curio::SpikeTrap(dmg) => {
                println!("You take {} damage from the spikes.", dmg);
                self.hp -= dmg;
            },
            Curio::Food(heal) => {
                println!("You shove a wall chicken into your gob and heal {} HP.", heal);
                self.hp = std::cmp::min(MAX_HP, self.hp + heal);
            },
            Curio::IronMaiden(sub, dmg) => {
                println!("Dude I love Iron Maiden! This one's pointy, though.");
                println!("You cut yourself on the spikes inside for {} damage.", dmg);
                self.hp -= dmg;
                println!("You open the iron maiden and...");
                self.use_curio(*sub);
            },
            Curio::FallenAdventurer(sub) => {
                println!("You pilfer the corpse and...");
                self.use_curio(*sub);
            },
        }
    }

    /// Execute the given command on the player and board state.
    pub fn act(&mut self, cmd: Command) -> Result<(), ()> {
        match cmd {
            Command::Go(go_string) => {
                self.location = self.find_room(go_string)?;
            }
            Command::Shoot(shoot_string) => {
                let shoot_room = self.find_room(shoot_string)?;
                let is_wumpus = shoot_room.borrow().wumpus;
                match is_wumpus {
                    true => {
                        shoot_room.borrow_mut().wumpus = false;
                        println!("Ya, you kill the wumpus!");
                        self.won = true
                    }
                    false => {
                        println!("Ops, you hit nothing.....");
                    }
                }
            }
        }
        Ok(())
    }

    pub fn room_event(&mut self) {
        // This place is vert stange. But I just finish it.
        // Let me explain!
        // First the self.use_curio function will count one for mut ref on self,
        // so it is not allow to do reference unless you clone a copy of self or 
        // self.location. However, we really need the ref on self or self.location,
        // since we want to use borrow on location and get the contents(Vec) as **ref**
        // (it didn't have copy trait. if we want to borrow to access data, we need 
        // to get ref). Anyway, we need the ref!

        // As we get the ref on self or self.location. we can simply borrow it and ref
        // contents. Then, iterate it with using function use_curio. WooYah, finish this part.

        // let room_contents = self.clone().location.borrow().contents;
        let loc_clone = self.location.clone();
        let room_contents = &loc_clone.borrow().contents;
        for one_curio in room_contents {
            self.use_curio(one_curio.clone());
        }
        if loc_clone.borrow().wumpus {
            println!("Whatever you do, you met wumpus. Just dead :)");
            self.won = false;
            self.hp = 0;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    pub fn is_win(&self) -> bool {
        self.won
    }

    /// Find one of the neighbors of the current room based on its name. Case insensitive.
    fn find_room(&self, room: String) -> Result<Rc<RefCell<Room>>, ()> {
        let halls = &self.location.borrow().halls;

        let mut filtered_halls = halls.iter().map(
            |hall| hall.right.clone()
        ).filter(|hall_right| {
            let adj_room_name = &hall_right.borrow().name;
            adj_room_name.to_lowercase() == room.to_lowercase()
        }).collect::<Vec<Rc<RefCell<Room>>>>();

        filtered_halls.pop().ok_or(())
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "You find yourself in {}.\n\nYou have {} HP and {} gold.",
               self.location.borrow().name, self.hp, self.gold)
    }
}
