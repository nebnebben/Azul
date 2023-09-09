use crate::factories::bag::Bag;
use crate::factories::factory_centre::FactoryCentre;
use crate::factories::factory_tile::FactoryTile;
use crate::tiles::Tiles;

const NUM_FACTORIES: usize = 5;

pub struct FactoryController {
    factories: [FactoryTile; NUM_FACTORIES],
    factory_centre: FactoryCentre,
    bag: Bag
}

impl FactoryController {
    pub fn new() -> FactoryController {
        let mut bag = Bag::new();
        let factories: [FactoryTile; NUM_FACTORIES] =
            core::array::from_fn(|_i| FactoryTile::new(bag.get_factory_tiles()));
        let factory_centre = FactoryCentre::new();

        FactoryController{factories, factory_centre, bag }
    }

    pub fn refill_tiles(&mut self) {
        if self.bag.is_empty() {
            self.bag.reset_bag();
        }

        for factory in &mut self.factories {
            factory.update_tiles(self.bag.get_factory_tiles());
        }
    }

    pub fn display_tiles(&self) {
        self.factories.iter().for_each(|factory| factory.display_options());
        self.factory_centre.display_options()
    }

    pub fn get_tiles_from_factory(&mut self, factory_number: u16, chosen_tile: Tiles) -> Vec<Tiles> {
        let factory_chosen = &mut self.factories[factory_number as usize];
        let player_tiles = factory_chosen.choose_tiles(chosen_tile);
        let factory_centre_tiles = factory_chosen.get_current_tiles();
        // Remove factory tiles
        factory_chosen.update_tiles(Vec::new());
        // Add tiles to centre
        self.factory_centre.add_tiles(factory_centre_tiles);

        player_tiles
    }

    pub fn get_tiles_from_centre(&mut self, chosen_tile: Tiles) -> Vec<Tiles> {
        return self.factory_centre.choose_tiles(chosen_tile);
    }

}