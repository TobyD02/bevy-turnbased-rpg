use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalRng;

#[derive(Component)]
pub struct StatsComponent {
    str: f32,
    dex: f32,
    con: f32,
    wis: f32,
    cha: f32,
    int: f32,
}

impl Default for StatsComponent {
    fn default() -> Self {
        Self {
            str: 10.,
            dex: 10.,
            con: 10.,
            wis: 10.,
            cha: 10.,
            int: 10.,
        }
    }
}

impl StatsComponent {
    pub fn get_int_mod(&self) -> i32 {
        (self.int as i32 - 10) / 2
    }
    pub fn get_cha_mod(&self) -> i32 {
        (self.cha as i32 - 10) / 2
    }
    pub fn get_wis_mod(&self) -> i32 {
        (self.wis as i32 - 10) / 2
    }
    pub fn get_con_mod(&self) -> i32 {
        (self.con as i32 - 10) / 2
    }
    pub fn get_dex_mod(&self) -> i32 {
        (self.dex as i32 - 10) / 2
    }
    pub fn get_str_mod(&self) -> i32 {
        (self.str as i32 - 10) / 2
    }

    pub fn roll_initiative(&self, next_rand_u32: u32) -> i32 {
        self.get_dex_mod() + (next_rand_u32  % 20) as i32 + 1
    }
}
