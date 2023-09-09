use crate::tiles::Tiles;
use crate::tiles::Tiles::{Black, DeepBlue, IceBlue, Red, Yellow};

pub(crate) const PATTERN_TILES: [[Tiles; 5]; 5] = [
    [DeepBlue, Yellow, Red, Black, IceBlue],
    [IceBlue, DeepBlue, Yellow, Red, Black],
    [Black, IceBlue, DeepBlue, Yellow, Red],
    [Red, Black, IceBlue, DeepBlue, Yellow],
    [Yellow, Red, Black, IceBlue, DeepBlue]
];

pub(crate) const BUFFER_LIMITS: [u32; 5] = [1, 2, 3, 4, 5];
pub(crate) const FLOOR_POINTS_LOSS: [i32; 7] = [1, 1, 2, 2, 2, 3, 3];
