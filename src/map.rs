use crate::constants::SCREEN_SIZE;

pub const MAP_HEIGHT: usize = 8;
pub const MAP_WIDTH: usize = 21;
pub const TILE_SIZE: i32 = SCREEN_SIZE as i32 / MAP_WIDTH as i32;

const MAP: [u8; MAP_HEIGHT * MAP_WIDTH] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1,
    1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1,
    1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1,
    1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 2,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Terrain {
    Open,
    Wall,
    Doorway,
    Mirage,
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Sjekk ka som finst eit punkt på kartet
pub fn read_map(x: f32, y: f32) -> Terrain {
    match MAP.get((y as i32 * MAP_WIDTH as i32 + x as i32) as usize) {
        Some(&square) if square == 0 => Terrain::Open,
        Some(&square) if square == 1 => Terrain::Wall,
        Some(&square) if square == 2 => Terrain::Doorway,
        Some(&square) if square == 4 => Terrain::Mirage,
        _ => Terrain::Wall,
    }
}