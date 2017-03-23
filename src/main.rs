use std::env;
use std::str;

fn parse_input(args: Vec<String>) -> [[u32; 9]; 9]
{
    let puzzle: [[u32; 9]; 9] = [[0; 9]; 9];
    let mut x: u32 = 0;
    for y in 1..10
    {
        x = 0;
        for c in args[y].chars()
        {
            if (c != '.')
            {
                //puzzle[y - 1][x] = c.from_digit();
            }
            x += 1;
        }
    }
    puzzle
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    if (args.len() - 1 == 0 || args.len() - 1 != 9)
    {
        println!("USAGE:: Enter each row of a sudoku puzzle seperated by spaces, use a '.' for an empty value");
    }
    let mut puzzle = parse_input(args);
}
