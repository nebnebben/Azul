use crate::tiles::Tiles;

pub struct FactoryTile {
    tiles: Vec<Tiles>
}

impl FactoryTile {
    pub fn new(mut tiles: Vec<Tiles>) -> FactoryTile {
        tiles.sort();
        FactoryTile {tiles}
    }

    pub fn update_tiles(&mut self, tiles: Vec<Tiles>) {
        self.tiles = tiles;
        self.tiles.sort();
    }

    pub fn display_options(&self) {
        println!("{:?}", self.tiles);
    }

    pub fn get_current_tiles(&self) -> Vec<Tiles>{
        self.tiles.clone()
    }

    pub fn choose_tiles(&mut self, chosen_tile: Tiles) -> Vec<Tiles> {
        let chosen_tiles = self.tiles.iter()
            .filter(|&tile| *tile == chosen_tile)
            .cloned()
            .collect();
        self.tiles.retain(|&tile | tile != chosen_tile);
        chosen_tiles
    }
}