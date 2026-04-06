use bevy::prelude::KeyCode;

pub enum ControlMappingEnum {
    PlayerMoveUp,
    PlayerMoveDown,
    PlayerMoveLeft,
    PlayerMoveRight,
    AllowCharacterTurn,
}

impl ControlMappingEnum {
    pub fn keycode(&self) -> KeyCode {
        match self {
            ControlMappingEnum::PlayerMoveUp => KeyCode::KeyW,
            ControlMappingEnum::PlayerMoveDown => KeyCode::KeyS,
            ControlMappingEnum::PlayerMoveLeft => KeyCode::KeyA,
            ControlMappingEnum::PlayerMoveRight => KeyCode::KeyD,
            ControlMappingEnum::AllowCharacterTurn => KeyCode::Space,
        }
    }
}