use crate::player::player_constants::BUFFER_LIMITS;
use crate::tiles::Tiles;

// #[derive(Copy, Clone)]
pub struct TileBuffer {
    tile_buffer: Vec<Vec<Tiles>>
}

impl TileBuffer {
    pub fn new() -> TileBuffer {
        let tile_buffer = vec![Vec::new(); 5];
        TileBuffer{tile_buffer}
    }

    pub fn add_tiles_to_buffer_and_return_leftover(&mut self, buffer_num: usize, tiles: Vec<Tiles>) -> Vec<Tiles> {
        let mut unused_tiles = tiles.clone();

        let buffer_spaces_remaining = BUFFER_LIMITS[buffer_num] - self.tile_buffer[buffer_num].len() as u32;
        let buffer_tiles: Vec<_> = unused_tiles.drain(..buffer_spaces_remaining as usize).collect();

        self.tile_buffer[buffer_num].extend(buffer_tiles);
        return unused_tiles
    }

    pub fn reset_buffer(&mut self) {
        self.tile_buffer = vec![Vec::new(); 5];
    }

    pub fn get_tile_buffer(self) -> Vec<Vec<Tiles>>{
        return self.tile_buffer;
    }
}