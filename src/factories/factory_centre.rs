use crate::tiles::Tiles;

pub struct FactoryCentre {
    tiles: Vec<Tiles>
}

impl FactoryCentre {
    pub fn new() -> FactoryCentre {
        FactoryCentre{tiles: Vec::new()}
    }

    pub fn add_tiles(&mut self, new_tiles: Vec<Tiles>) {
        self.tiles.extend(new_tiles);
        self.tiles.sort();
    }

    pub fn get_current_tiles(&self) -> Vec<Tiles>{
        self.tiles.clone()
    }

    pub fn display_options(&self) {
        println!("{:?}", self.tiles);
    }

    pub fn choose_tiles(&mut self, chosen_tile: Tiles) -> Vec<Tiles> {
        let chosen_tiles = self.tiles.iter()
            .filter(|&tile| *tile == chosen_tile)
            .cloned()
            .collect();
        // Does this work? It doesn't derefence tile
        self.tiles.retain(|&tile | tile != chosen_tile);
        chosen_tiles
    }
}
