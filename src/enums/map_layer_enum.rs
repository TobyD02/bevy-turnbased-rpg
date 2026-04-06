pub enum MapLayerEnum {
    MapLayerBackground = 0,
    MapLayerTiles = 1,
    MapLayerCharacters = 2,
    MapLayerPlayers = 3,
}

impl MapLayerEnum {
    pub fn float (self) -> f32 {
        self as i32 as f32
    }
}