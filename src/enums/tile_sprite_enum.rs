pub enum TileSpriteEnum {
    WallRoundedSolidTopLeft = 0,
    WallRoundedSolidTop = 1,
    WallRoundedSolidTopArch = 2,
    WallRoundedSolidTopRight = 3,
    PlayerIdle = 4,
    PlayerBlock = 5,
    PlayerAttack = 6,
    PlayerBuff = 7,
}

impl TileSpriteEnum {
    pub fn usize(self) -> usize {
        self as usize
    }
}