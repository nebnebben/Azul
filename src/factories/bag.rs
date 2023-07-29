use crate::tiles::Tiles;
use rand::seq::SliceRandom;


const NUM_TILES: usize = 100;
pub struct Bag {
    contents: [Tiles; NUM_TILES as usize],
    seen: Vec<Tiles>,
    unseen: Vec<Tiles>
}

impl Bag {
    pub fn new() -> Bag {
        let tile_options = [Tiles::Red, Tiles::DeepBlue, Tiles::IceBlue, Tiles::Black, Tiles::Yellow];
        let mut contents: [Tiles; NUM_TILES] = [Tiles::Black; NUM_TILES];
        for i in 0..NUM_TILES {
            contents[i] = tile_options[i % 5];
        }
        contents.shuffle(&mut rand::thread_rng());
        let unseen = contents.to_vec();
        println!("{:?}", contents);
        Bag{contents, seen: vec![], unseen}
    }

    pub fn get_factory_tiles(&mut self) -> Vec<Tiles> {
        let start_drain_index = (self.unseen.len()-4).max(0);
        let mut factory_tiles: Vec<Tiles> = Vec::new();
        for  tile in self.unseen.drain(start_drain_index..) {
            self.seen.push(tile);
            factory_tiles.push(tile)
        }
        factory_tiles
    }

    pub fn is_empty(&self) -> bool {
        return self.unseen.is_empty()
    }

    pub fn reset_bag(&mut self) {
        self.seen = Vec::new();
        self.unseen = self.contents.to_vec();
    }
}