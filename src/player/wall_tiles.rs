use crate::tiles::Tiles;
use crate::player::player_constants::PATTERN_TILES;

pub struct WallTiles {
    tile_pattern: [[bool; 5]; 5],
    unfilled_tiles: Vec<Vec<Tiles>>
}

impl WallTiles {
    pub fn new() -> WallTiles {
        let tile_pattern = core::array::from_fn(|_| [false; 5]);
        let unfilled_tiles = PATTERN_TILES.iter()
            .map(|&row| row.to_vec()).collect();
        WallTiles{tile_pattern, unfilled_tiles}
    }

    pub fn place_tile(&mut self, placed_tile: Tiles, row: usize) {
        if !self.unfilled_tiles[row].contains(&placed_tile) {
            panic!("There was already a tile of that type on the row!");
        }

        let column_index = PATTERN_TILES[row].iter()
            .position(|&tile| tile == placed_tile).unwrap();

        self.tile_pattern[row][column_index] = true;
        self.unfilled_tiles[row].retain(|tile| *tile != placed_tile);
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

    pub fn get_unplaced_tiles_for_row(&self, row: usize) -> Vec<Tiles> {
        return self.unfilled_tiles[row].clone();
    }

}