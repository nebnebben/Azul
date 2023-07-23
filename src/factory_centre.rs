use crate::tiles::Tiles;

pub struct FactoryCentre {
    tiles: Vec<Tiles>
}

impl FactoryCentre {
    pub fn new() -> FactoryCentre {
        FactoryCentre{tiles: Vec::new()}
    }
}
