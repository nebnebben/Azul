use crate::player::floor::Floor;
use crate::player::tile_buffer::TileBuffer;
use crate::player::wall_tiles::WallTiles;
use crate::tiles::Tiles;
use crate::tiles::Tiles::{Black, DeepBlue, IceBlue, Red, Yellow};

const PATTERN_TILES: [[Tiles; 5]; 5] = [
    [DeepBlue, Yellow, Red, Black, IceBlue],
    [IceBlue, DeepBlue, Yellow, Red, Black],
    [Black, IceBlue, DeepBlue, Yellow, Red],
    [Red, Black, IceBlue, DeepBlue, Yellow],
    [Yellow, Red, Black, IceBlue, DeepBlue]
];

const BUFFER_LIMITS: [u32; 5] = [1, 2, 3, 4, 5];
const FLOOR_POINTS_LOSS: [i32; 7] = [1, 1, 2, 2, 2, 3, 3];

pub struct Player {
    tile_buffer: TileBuffer,
    floor: Floor,
    wall_tiles: WallTiles,
    score: i32
}

impl Player {
    pub fn new() -> Player {
        let tile_buffer = TileBuffer::new();
        let floor = Floor::new();
        let wall_tiles = WallTiles::new();
        let score = 0;
        Player {tile_buffer, floor, wall_tiles, score}
    }

    pub fn add_tiles_to_buffer(&mut self, buffer_num: usize, tiles: Vec<Tiles>) {
        let leftover_tiles = self.tile_buffer.add_tiles_to_buffer_and_return_leftover(buffer_num, tiles);
        self.floor.add_tiles(leftover_tiles); //clone?
    }

    // pub fn get_score_for_round(&mut self) -> i32 {
    //     let buffer = self.tile_buffer.get_tile_buffer();
    //     if buffer.len() != 5 {
    //         panic!("Buffer should be length 5");
    //     }
    //     let mut score = 0;
    //     for i in 0..5 {
    //         let cur_tiles = &buffer[i];
    //         if cur_tiles.len() != (i+1){
    //             continue;
    //         }
    //         score += self.wall_tiles.calculate_score_from_placed_tile(cur_tiles[0], i);
    //     }
    //
    //     score
    //
    // }

    pub fn start_new_round(&mut self) {
        self.tile_buffer.reset_buffer();
        self.floor.reset();
    }

}