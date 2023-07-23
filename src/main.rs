mod bag;
mod tiles;
mod factory_centre;
mod factory;

use rand::{thread_rng, Rng};
use crate::bag::Bag;

fn random_enum_variant() -> tiles::Tiles {
    let variants = [tiles::Tiles::Red, tiles::Tiles::Black, tiles::Tiles::DeepBlue];
    let mut rng = thread_rng();
    let index = rng.gen_range(0..variants.len());
    variants[index]
}

fn main() {
    println!("Hello, world!");
    let my_enum_array: [tiles::Tiles; 5] = [random_enum_variant(); 5];
    println!("{:?}", my_enum_array);

}
