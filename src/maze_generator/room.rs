pub const DIRECTION_UP : u8 = 1;
pub const DIRECTION_DOWN : u8 = 2;
pub const DIRECTION_LEFT : u8 = 4;
pub const DIRECTION_RIGHT : u8 = 8;

use super::coords::Coords;

pub fn apply_direction(coords : &Coords, direction : u8) -> Coords
{
    match direction
    {
        DIRECTION_UP => Coords::new(coords.x, coords.y - 1),
        DIRECTION_DOWN => Coords::new(coords.x, coords.y + 1),
        DIRECTION_LEFT => Coords::new(coords.x - 1, coords.y),
        DIRECTION_RIGHT => Coords::new(coords.x + 1, coords.y),
        _ => unreachable!()
    }
}

pub fn opposite_direction(direction : u8) -> u8
{
    match direction
    {
        DIRECTION_UP => DIRECTION_DOWN,
        DIRECTION_DOWN => DIRECTION_UP,
        DIRECTION_LEFT => DIRECTION_RIGHT,
        DIRECTION_RIGHT => DIRECTION_LEFT,
        _ => unreachable!()
    }
}

pub struct Room
{
    pub visited : bool,
    directions : u8
}

impl Room
{
    pub fn new() -> Room
    {
        Room { visited : false, directions : 0 }
    }

    pub fn with_directions(directions : u8) -> Room
    {
        let mut room = Room::new();

        room.dig_direction(directions & DIRECTION_UP);
        room.dig_direction(directions & DIRECTION_DOWN);
        room.dig_direction(directions & DIRECTION_LEFT);
        room.dig_direction(directions & DIRECTION_RIGHT);

        room
    }

    pub fn dig_direction(&mut self, direction : u8)
    {
        self.directions |= direction;
    }

    pub fn is_dig_direction(&self, direction : u8) -> bool
    {
        self.directions & direction != 0
    }
}
