extern crate rand;

use self::rand::Rng;

use super::room::*;
use super::coords::Coords;

pub struct Maze
{
    width : usize,
    height : usize,
    rooms : Vec<Room>
}

impl Maze
{
    pub fn new(width : usize, height : usize) -> Maze
    {
        let mut maze = Maze { width : width, height : height, rooms : Vec::with_capacity(width * height) };
        maze.setup_bounds();
        maze.backtracking();
        maze
    }

    fn setup_bounds(&mut self)
    {
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                let mut directions : u8 = 0;

                if x == 0
                {
                    directions |= DIRECTION_LEFT;
                }
                else if x == self.width - 1
                {
                    directions |= DIRECTION_RIGHT;
                }

                if y == 0
                {
                    directions |= DIRECTION_UP;
                }
                else if y == self.width - 1
                {
                    directions |= DIRECTION_DOWN;
                }

                self.rooms.push(Room::with_directions(directions));
            }
        }
    }

    fn random_direction_from(&self, coords : &Coords) -> Option<u8>
    {
        let directions = [DIRECTION_UP, DIRECTION_DOWN, DIRECTION_LEFT, DIRECTION_RIGHT];
        let directions : Vec<_> = directions.into_iter()
                                            .filter(|direction| !self.at(&coords).is_dig_direction(**direction))
                                            .filter(|direction| !self.at(&apply_direction(coords, **direction)).visited)
                                            .collect();

        match directions.len()
        {
            0 => None,
            len => Some(*directions[rand::thread_rng().gen_range(0, len)])
        }
    }

    fn backtracking(&mut self)
    {
        let mut stack = Vec::new();

        let mut current_coords = Coords::new(0, 0);
        self.at_mut(&current_coords).visited = true;

        while let Some(_) = self.rooms.iter().find(|room| !room.visited)
        {
            if let Some(direction) = self.random_direction_from(&current_coords)
            {
                self.at_mut(&current_coords).dig_direction(direction);
                current_coords = apply_direction(&current_coords, direction);
                stack.push(current_coords);
                self.at_mut(&current_coords).visited = true;
                self.at_mut(&current_coords).dig_direction(opposite_direction(direction));
            }
            else
            {
                current_coords = stack.pop().unwrap();
            }
        }
    }

    pub fn at(&self, coords : &Coords) -> &Room
    {
        &self.rooms[coords.to_index(self.width)]
    }

    pub fn at_mut(&mut self, coords : &Coords) -> &mut Room
    {
        &mut self.rooms[coords.to_index(self.width)]
    }

    pub fn width(&self) -> usize
    {
        self.width
    }

    pub fn height(&self) -> usize
    {
        self.height
    }
}
