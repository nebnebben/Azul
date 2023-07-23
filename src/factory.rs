use crate::tiles::Tiles;

pub struct  Factory {
    tiles: Vec<Tiles>
}

impl Factory {
    pub fn new(mut tiles: Vec<Tiles>) -> Factory {
        tiles.sort();
        Factory{tiles}
    }

    pub fn update_tiles(&mut self, tiles: Vec<Tiles>) {
        self.tiles = tiles;
        self.tiles.sort();
    }

    pub fn display_options(&self) {
        println!("{:?}", self.tiles);
    }

    pub fn get_options(&self) -> Vec<Tiles>{
        self.tiles.clone()
    }

    // Also take away from current tiles and add to centre
    pub fn choose_tile(&mut self, chosen_tile: Tiles) -> Vec<Tiles> {
        self.tiles.iter()
            .filter(|&tile| *tile == chosen_tile)
            .cloned()
            .collect()
    }
}