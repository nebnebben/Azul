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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_tiles() {
        let tiles = vec![Tiles::Red, Tiles::Red, Tiles::Yellow];
        let mut factory_tile = FactoryTile::new(tiles);
        let new_tiles = vec![Tiles::DeepBlue, Tiles::DeepBlue, Tiles::DeepBlue];
        factory_tile.update_tiles(new_tiles.clone());
        assert_eq!(factory_tile.tiles, new_tiles);
    }

    #[test]
    fn test_get_current_tiles() {
        let tiles = vec![Tiles::Red, Tiles::Red, Tiles::Yellow];
        let mut factory_tile = FactoryTile::new(tiles.clone());
        assert_eq!(factory_tile.get_current_tiles(), tiles);
    }

    #[test]
    fn test_choose_tiles() {
        let tiles = vec![Tiles::Red, Tiles::Red, Tiles::Yellow, Tiles::DeepBlue];
        let mut factory_tile = FactoryTile::new(tiles);
        let chosen_tile = Tiles::Red;
        let chosen_tiles = factory_tile.choose_tiles(chosen_tile);
        assert_eq!(chosen_tiles, vec![Tiles::Red, Tiles::Red]);
        assert_eq!(factory_tile.get_current_tiles(), vec![Tiles::DeepBlue, Tiles::Yellow]);
    }
}
