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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use crate::factories::bag::Bag;
    use crate::tiles::Tiles;

    #[test]
    fn test_new_bag() {
        let bag = Bag::new();
        let contents = &bag.contents;
        // Ensure that the bag has the correct number of tiles
        assert_eq!(bag.contents.len(), NUM_TILES);

        // Ensure that all tiles are from the predefined options
        let tile_options = [Tiles::Red, Tiles::DeepBlue, Tiles::IceBlue, Tiles::Black, Tiles::Yellow];
        for tile in &bag.contents {
            assert!(tile_options.contains(tile));
        }
    }

    #[test]
    fn test_get_factory_tiles() {
        let mut bag = Bag::new();

        // Ensure that calling get_factory_tiles returns 4 tiles and they are removed from unseen
        let factory_tiles = bag.get_factory_tiles();
        assert_eq!(factory_tiles.len(), 4);
        assert_eq!(bag.unseen.len(), NUM_TILES - 4);
    }

    #[test]
    fn test_is_empty() {
        let mut bag = Bag::new();
        assert!(!bag.is_empty()); // Bag should not be empty initially

        // Drain the bag to make it empty
        while !bag.is_empty() {
            bag.get_factory_tiles();
        }

        assert!(bag.is_empty()); // Bag should be empty after draining
    }

    #[test]
    fn test_reset_bag() {
        let mut bag = Bag::new();

        // Drain some tiles to simulate gameplay
        for _ in 0..10 {
            bag.get_factory_tiles();
        }

        // Reset the bag and ensure it's back to its initial state
        bag.reset_bag();
        assert_eq!(bag.seen.len(), 0);
        assert_eq!(bag.unseen.len(), NUM_TILES);
        assert_eq!(bag.contents.len(), NUM_TILES);
    }
}
