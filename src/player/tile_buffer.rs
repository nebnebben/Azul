use crate::player::player_constants::BUFFER_LIMITS;
use crate::tiles::Tiles;

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
        let amount_to_drain = buffer_spaces_remaining.min(unused_tiles.len() as u32);
        let buffer_tiles: Vec<_> = unused_tiles.drain(..amount_to_drain as usize).collect();

        self.tile_buffer[buffer_num].extend(buffer_tiles);
        return unused_tiles
    }

    pub fn reset_buffer(&mut self) {
        self.tile_buffer = vec![Vec::new(); 5];
    }

    pub fn get_tile_buffer(&self) -> &Vec<Vec<Tiles>> {
        return &self.tile_buffer;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tile_buffer() {
        let tile_buffer = TileBuffer::new();
        assert_eq!(tile_buffer.tile_buffer.len(), 5);
        for buffer in tile_buffer.tile_buffer.iter() {
            assert!(buffer.is_empty());
        }
    }

    #[test]
    fn test_add_tiles_to_buffer_and_return_leftover() {
        let mut tile_buffer = TileBuffer::new();

        // Add tiles to buffer 1
        let tiles1 = vec![Tiles::Red, Tiles::Red];
        let leftover1 = tile_buffer.add_tiles_to_buffer_and_return_leftover(0, tiles1.clone());

        // Add tiles to buffer 5
        let tiles5 = vec![Tiles::Red, Tiles::Red, Tiles::Red, Tiles::Red];
        let leftover5 = tile_buffer.add_tiles_to_buffer_and_return_leftover(4, tiles5.clone());

        let buffer = tile_buffer.get_tile_buffer();

        assert_eq!(buffer[0], vec![Tiles::Red]);
        assert_eq!(buffer[4], vec![Tiles::Red, Tiles::Red, Tiles::Red, Tiles::Red]);
        assert_eq!(leftover1, vec![Tiles::Red]);
        assert_eq!(leftover5, vec![]);

    }

    #[test]
    fn test_reset_buffer() {
        let mut tile_buffer = TileBuffer::new();

        // Add tiles to buffer 0
        let tiles = vec![Tiles::Red, Tiles::Red];
        tile_buffer.add_tiles_to_buffer_and_return_leftover(0, tiles);

        // Reset the buffer
        tile_buffer.reset_buffer();

        for buffer in tile_buffer.tile_buffer.iter() {
            assert!(buffer.is_empty());
        }
    }
}
