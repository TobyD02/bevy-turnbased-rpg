use bevy::prelude::*;

#[derive(Component)]
pub struct StatsComponent {
    str: f32,
    dex: f32,
    con: f32,
    wis: f32,
    cha: f32,
    int: f32
}

impl Default for StatsComponent {
    fn default() -> Self {
        Self {
            str: 10.,
            dex: 10.,
            con: 10.,
            wis: 10.,
            cha: 10.,
            int: 10.
        }
    }
}

impl StatsComponent {
    pub fn roll_initiative(&self) -> i32 {
        self.dex as i32
    }
}