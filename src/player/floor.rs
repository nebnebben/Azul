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

    pub fn add_tiles(&mut self, mut new_tiles: Vec<Tiles>) {
        if self.tiles.len() == 7 {
            return;
        }
        let number_tiles_to_use = (7-self.tiles.len()).min(new_tiles.len());
        self.tiles.extend(new_tiles.drain(..number_tiles_to_use))
    }

    pub fn calculate_lost_score(&self) -> i32{
        let number_tiles = self.tiles.len();
        return FLOOR_POINTS_LOSS[..number_tiles].iter().sum();
    }

    pub fn reset(&mut self) {
        self.tiles = Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_floor() {
        let floor = Floor::new();
        assert!(floor.tiles.is_empty());
    }

    #[test]
    fn test_add_tiles_when_under_limit() {
        let mut floor = Floor::new();
        let tiles_to_add = vec![Tiles::Red, Tiles::Red, Tiles::Yellow];
        floor.add_tiles(tiles_to_add.clone());
        assert_eq!(floor.tiles, tiles_to_add);
    }

    #[test]
    fn test_add_tiles_when_over_limit() {
        let mut floor = Floor::new();
        let tiles_to_add = vec![Tiles::Red; 10];
        floor.add_tiles(tiles_to_add.clone());
        assert_eq!(floor.tiles, vec![Tiles::Red; 7]);
    }

    #[test]
    fn test_calculate_lost_score() {
        let mut floor = Floor::new();
        let tiles_to_add = vec![Tiles::Red; 7];
        floor.add_tiles(tiles_to_add.clone());

        // Calculate the expected lost score
        let expected_lost_score = 14;
        assert_eq!(floor.calculate_lost_score(), expected_lost_score);
    }

    #[test]
    fn test_reset() {
        let mut floor = Floor::new();
        let tiles_to_add = vec![Tiles::Red, Tiles::Red, Tiles::Yellow];
        floor.add_tiles(tiles_to_add);
        floor.reset();
        assert!(floor.tiles.is_empty());
    }
}
