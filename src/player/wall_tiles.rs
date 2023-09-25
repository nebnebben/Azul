use std::collections::HashSet;
use crate::tiles::Tiles;
use crate::player::player_constants::PATTERN_TILES;

pub struct WallTiles {
    // tiles placed
    tile_pattern: [[bool; 5]; 5],
    // types of tiles that haven't been used
    unfilled_tiles: [HashSet<Tiles>; 5]
}

impl WallTiles {
    pub fn new() -> WallTiles {
        let tile_pattern = core::array::from_fn(|_| [false; 5]);
        let mut unfilled_tiles = core::array::from_fn(|_| HashSet::new());
        for (i, row) in PATTERN_TILES.iter().enumerate() {
            for tile in row {
                unfilled_tiles[i].insert(*tile);
            }
        }
        WallTiles{tile_pattern, unfilled_tiles}
    }

    pub fn place_tile(&mut self, placed_tile: Tiles, row: usize) {
        if !self.unfilled_tiles[row].contains(&placed_tile) {
            panic!("There was already a tile of that type on the row!");
        }

        let column_index = PATTERN_TILES[row].iter()
            .position(|&tile| tile == placed_tile).unwrap();

        self.tile_pattern[row][column_index] = true;
        self.unfilled_tiles[row].remove(&placed_tile);
    }

    pub fn calculate_score_from_placed_tile(&self, placed_tile: Tiles, row_index: usize) -> i32 {
        let column_index = PATTERN_TILES[row_index].iter()
            .position(|tile| *tile == placed_tile).unwrap();

        let mut contiguous_row_tiles = 0;
        contiguous_row_tiles += (column_index+1..5).take_while(|&i| self.tile_pattern[row_index][i]).count();
        contiguous_row_tiles += (0..column_index).rev().take_while(|&i| self.tile_pattern[row_index][i]).count();

        let row_tile_points = if contiguous_row_tiles > 0 {contiguous_row_tiles + 1} else {0};

        let mut contiguous_column_tiles = 0;
        contiguous_column_tiles += (row_index+1..5).take_while(|&i| self.tile_pattern[i][column_index]).count();
        contiguous_column_tiles += (0..row_index).rev().take_while(|&i| self.tile_pattern[i][column_index]).count();

        let column_tile_points = if contiguous_column_tiles > 0 {contiguous_column_tiles + 1} else {0};

        return  (row_tile_points + column_tile_points).max(1) as i32;
    }

    pub fn get_unplaced_tiles_for_row(&self, row: usize) -> HashSet<Tiles> {
        return self.unfilled_tiles[row].clone();
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::player_constants::PATTERN_TILES;
    use crate::tiles::Tiles::{Black, DeepBlue, IceBlue, Red, Yellow};

    #[test]
    fn test_new_wall_tiles() {
        let wall_tiles = WallTiles::new();
        assert_eq!(wall_tiles.unfilled_tiles[0], HashSet::from([DeepBlue, Yellow, Red, Black, IceBlue]));
    }

    #[test]
    fn test_place_tile() {
        let mut wall_tiles = WallTiles::new();
        let placed_tile = Tiles::Red;
        wall_tiles.place_tile(placed_tile, 0);

        assert!(!wall_tiles.unfilled_tiles[0].contains(&placed_tile));
        assert!(wall_tiles.tile_pattern[0][2],  "{}", true)
    }

    #[test]
    #[should_panic(expected = "There was already a tile of that type on the row!")]
    fn test_place_tile_panic() {
        let mut wall_tiles = WallTiles::new();
        let placed_tile = DeepBlue;
        wall_tiles.place_tile(placed_tile, 0);
        wall_tiles.place_tile(placed_tile, 0); // This should panic
    }

    #[test]
    fn test_calculate_score_from_contiguous_tiles_in_column() {
        let mut wall_tiles = WallTiles::new();
        wall_tiles.place_tile(DeepBlue, 0);
        wall_tiles.place_tile(IceBlue, 1);

        let score = wall_tiles.calculate_score_from_placed_tile(Black, 2);
        assert_eq!(score, 3);
    }

    #[test]
    fn test_calculate_score_from_contiguous_tiles_in_row() {
        let mut wall_tiles = WallTiles::new();
        wall_tiles.place_tile(DeepBlue, 0);
        wall_tiles.place_tile(Yellow, 0);

        let score = wall_tiles.calculate_score_from_placed_tile(Red, 0);
        assert_eq!(score, 3);
    }

    #[test]
    fn test_calculate_score_when_next_to_column_and_row() {
        let mut wall_tiles = WallTiles::new();
        wall_tiles.place_tile(Yellow, 0);
        wall_tiles.place_tile(IceBlue, 1);

        let score = wall_tiles.calculate_score_from_placed_tile(DeepBlue, 1);
        assert_eq!(score, 4);
    }

    #[test]
    fn test_calculate_score_when_in_the_centre_of_many_tiles() {
        let mut wall_tiles = WallTiles::new();
        wall_tiles.place_tile(Black, 2);
        wall_tiles.place_tile(IceBlue, 2);
        wall_tiles.place_tile(Yellow, 2);
        wall_tiles.place_tile(Red, 2);

        wall_tiles.place_tile(Red, 0);
        wall_tiles.place_tile(Yellow, 1);
        wall_tiles.place_tile(IceBlue, 3);
        wall_tiles.place_tile(Black, 4);


        let score = wall_tiles.calculate_score_from_placed_tile(DeepBlue, 2);
        assert_eq!(score, 10);
    }


    #[test]
    fn test_calculate_score_when_next_to_nothing() {
        let mut wall_tiles = WallTiles::new();
        wall_tiles.place_tile(Yellow, 0);
        wall_tiles.place_tile(IceBlue, 1);

        let score = wall_tiles.calculate_score_from_placed_tile(Yellow, 1);
        assert_eq!(score, 1);
    }



    #[test]
    fn test_get_unplaced_tiles_for_row() {
        let wall_tiles = WallTiles::new();
        let unplaced_tiles = wall_tiles.get_unplaced_tiles_for_row(0);
        assert_eq!(unplaced_tiles, HashSet::from([DeepBlue, Yellow, Red, Black, IceBlue]));
    }
}
