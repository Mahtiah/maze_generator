#[derive(Clone, Copy)]
pub struct Coords
{
    pub x : usize,
    pub y : usize
}

impl Coords
{
    pub fn new(x : usize, y : usize) -> Coords
    {
        Coords { x : x, y : y }
    }

    pub fn to_index(&self, width : usize) -> usize
    {
        self.y * width + self.x
    }
}
