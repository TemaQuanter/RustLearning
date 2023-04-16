use mini_game::hero::{Orc, Hero};
use std::cmp::min;

struct Dwarf {
    healthpoints: u32,
    damage_per_hit: u32
} // end struct Dwarf

impl Hero for Dwarf {
    fn healthpoints(&self) -> u32 {
        self.healthpoints
    } // end healthpoints()

    fn get_damage(&mut self, healthpoints: u32) {
        self.healthpoints -= min(self.healthpoints, healthpoints);
    } // end get_damage()

    fn damage_per_hit(&self) -> u32 {
        self.damage_per_hit
    } // end damage_per_hit()

    fn heal(&mut self, healthpoints: u32) {
        self.healthpoints += healthpoints;
    } // end damage_per_hit()

    fn is_alive(&self) -> bool {
        self.healthpoints > 0
    } // end is_alive()
} // end impl Hero for Dwarf

impl Dwarf {
    fn mine_treasures() {
        println!("I'm a Dwarf and I'm mining treasures!");
    } // end mine_treasures()
} // end impl Dwarf

fn main() {
    let mut heros: Vec<Box<dyn Hero>> = Vec::new();
    
    heros.push(Box::new(Dwarf {
        healthpoints: 32,
        damage_per_hit: 33
    }));

    heros.push(Box::new(Orc {
        healthpoints: 70,
        damage_per_hit: 5
    }));

    heros.push(Box::new(Dwarf {
        healthpoints: 35,
        damage_per_hit: 2
    }));

    heros.push(Box::new(Orc {
        healthpoints: 15,
        damage_per_hit: 16
    }));

    heros.push(Box::new(Orc {
        healthpoints: 92,
        damage_per_hit: 7
    }));


    for mut hr in heros.into_iter() {
        println!();
        println!("This heros has {} healthpoins", hr.healthpoints());
        println!("This hero injures themselves and loses {} healthpoints", hr.damage_per_hit());
        hr.get_damage(hr.damage_per_hit());
        println!("This now the hero has {} healthpoints", hr.healthpoints());
        
        match hr.is_alive() {
            true => {
                println!("This hero is stil alive!");
                println!("The hero can heal themselves by 3 healthpoins");
                hr.heal(3);
                println!("Now the hero has {} healthpoints!", hr.healthpoints());
            } // end true
            false => {
                println!("The hero is dead:(");
            } // end false
        } // end match
        
        println!();
    } // end for

} // end main()
