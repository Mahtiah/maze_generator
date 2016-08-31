mod maze_generator;

use maze_generator::maze::Maze;
use maze_generator::coords::Coords;
use maze_generator::room::*;


fn maze_print(maze : &Maze)
{
    for x in 0..maze.width()
    {
        print!(" _");
    }

    println!("");

    for y in 0..maze.height()
    {
        for x in 0..maze.width()
        {
            let room = maze.at(&Coords::new(x, y));

            if !room.is_dig_direction(DIRECTION_LEFT) || x == 0
            {
                print!("|");
            }
            else
            {
                print!(" ");
            }

            if !room.is_dig_direction(DIRECTION_DOWN) || y == maze.height() - 1
            {
                print!("_");
            }
            else
            {
                print!(" ");
            }

            if x == maze.width() - 1
            {
                print!("|");
            }
        }

        println!("")
    }
}

fn main()
{
    let maze = Maze::new(50, 50);
    maze_print(&maze);
}
