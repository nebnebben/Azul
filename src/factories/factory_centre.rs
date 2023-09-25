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

    pub fn reset(&mut self) {
        self.tiles = Vec::new();
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

        self.tiles.retain(|&tile | tile != chosen_tile);
        chosen_tiles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_factory_centre() {
        let factory_centre = FactoryCentre::new();
        assert!(factory_centre.tiles.is_empty());
    }

    #[test]
    fn test_add_tiles() {
        let mut factory_centre = FactoryCentre::new();
        let tiles_to_add = vec![Tiles::Red, Tiles::Red, Tiles::Yellow];
        factory_centre.add_tiles(tiles_to_add.clone());
        assert_eq!(factory_centre.tiles, tiles_to_add);
    }

    #[test]
    fn test_get_current_tiles() {
        let tiles_to_add = vec![Tiles::Red, Tiles::Red, Tiles::Yellow];
        let mut factory_centre = FactoryCentre::new();
        factory_centre.add_tiles(tiles_to_add.clone());
        assert_eq!(factory_centre.get_current_tiles(), tiles_to_add);
    }

    #[test]
    fn test_choose_tiles() {
        let mut factory_centre = FactoryCentre::new();
        let tiles_to_add = vec![Tiles::Red, Tiles::Red, Tiles::DeepBlue, Tiles::Yellow];
        factory_centre.add_tiles(tiles_to_add.clone());

        let chosen_tile = Tiles::Red;
        let chosen_tiles = factory_centre.choose_tiles(chosen_tile);
        assert_eq!(chosen_tiles, vec![Tiles::Red, Tiles::Red]);
        assert_eq!(factory_centre.get_current_tiles(), vec![Tiles::DeepBlue, Tiles::Yellow]);
    }
}
