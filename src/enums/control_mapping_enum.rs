use bevy::prelude::KeyCode;

pub enum ControlMappingEnum {
    PlayerMoveUp,
    PlayerMoveDown,
    PlayerMoveLeft,
    PlayerMoveRight,
    PlayerEndTurn,
    AllowCharacterTurn,
}

impl ControlMappingEnum {
    pub fn keycode(&self) -> KeyCode {
        match self {
            ControlMappingEnum::PlayerMoveUp => KeyCode::KeyW,
            ControlMappingEnum::PlayerMoveDown => KeyCode::KeyS,
            ControlMappingEnum::PlayerMoveLeft => KeyCode::KeyA,
            ControlMappingEnum::PlayerMoveRight => KeyCode::KeyD,
            ControlMappingEnum::PlayerEndTurn => KeyCode::Enter,
            ControlMappingEnum::AllowCharacterTurn => KeyCode::Space,
        }
    }
}