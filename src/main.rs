use std::env;
use std::str;
mod puzzle_logic;
mod error;

fn parse_input(args: Vec<String>) -> [[u32; 9]; 9]
{
    let mut puzzle: [[u32; 9]; 9] = [[0; 9]; 9];
    let mut x: usize;

    for y in 1..10
    {
        x = 0;
        if args[y].len() != 9
        {
            error::print_usage_error(2);
        }
        for c in args[y].chars()
        {
            if c == '.'
            {
                puzzle[(y - 1) as usize][x] = 0;
            }
            else
            {
                //Also handles safe error checking, no problem converting normal chars to int
                let temp = (c as u32) - ('0' as u32);
                if temp > 0 && temp < 10
                {
                    puzzle[(y - 1) as usize][x] = temp;
                }
                else
                {
                    println!("ERROR::USAGE:: Invalid character in row {}", y);
                    error::print_usage_error(3);
                }
            }
            x += 1;
        }
    }
    puzzle
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() - 1 == 0 || args.len() - 1 != 9
    {
        error::print_usage_error(1);
    }
    let mut puzzle = parse_input(args);
    puzzle_logic::solve_puzzle(&mut puzzle);
}
