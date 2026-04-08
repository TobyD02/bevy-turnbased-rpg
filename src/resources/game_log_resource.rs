use bevy::prelude::*;

#[derive(Resource)]
pub struct GameLogResource {
    logs: Vec<String>
}

impl Default for GameLogResource {
    fn default() -> Self {
        Self {
            logs: Vec::new()
        }
    }
}

impl GameLogResource {
    pub fn log(&mut self, message: String) {
        self.logs.push(message)
    }

    pub fn get_logs(&self) -> &Vec<String> {
        &self.logs
    }
    pub fn get_logs_display_string(&self) -> String {
        self.logs
            .iter()
            .rev()
            .take(100)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    }
}