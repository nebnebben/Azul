use crate::player::player_constants::FLOOR_POINTS_LOSS;
use crate::tiles::Tiles;

pub struct Floor {
    tiles: Vec<Tiles>
}

impl Floor {
    pub fn new() -> Floor {
        let tiles = Vec::new();
        Floor{tiles}
    }

    pub fn add_tiles(&mut self, new_tiles: Vec<Tiles>) {
        self.tiles.extend(new_tiles);
    }

    pub fn calculate_lost_score(&self) -> i32{
        let number_tiles = self.tiles.len();
        return FLOOR_POINTS_LOSS[..number_tiles].iter().sum();
    }

    pub fn reset(&mut self) {
        self.tiles = Vec::new();
    }
}