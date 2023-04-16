pub mod hero {
    use std::cmp::min;
    pub trait Hero {
        fn healthpoints(&self) -> u32;
        fn heal(&mut self, healthpoints: u32);
        fn get_damage(&mut self, healthpoints: u32);
        fn is_alive(&self) -> bool;

        fn damage_per_hit(&self) -> u32;
    } // end trait Hero

    pub struct Orc {
        pub healthpoints: u32,
        pub damage_per_hit: u32,
    } // end struct Orc

    impl Hero for Orc {
        fn healthpoints(&self) -> u32 {
            self.healthpoints
        } // end get_healthpoints()

        fn heal(&mut self, healthpoints: u32) {
            self.healthpoints += healthpoints;
        } // end heal()

        fn get_damage(&mut self, healthpoints: u32) {
            self.healthpoints -= min(self.healthpoints, healthpoints);
        } // end get_damage()

        fn is_alive(&self) -> bool {
            self.healthpoints > 0
        } // end is_alive()

        fn damage_per_hit(&self) -> u32 {
            self.damage_per_hit
        } // end damage_per_hit()
    } // end impl Hero for Orc

    impl Orc {
        pub fn mine_gold() {
            println!("I'm an Orc and I'm mining gold!");
        } // end mine_gold()
    } // end impl Orc
} // end mod hero