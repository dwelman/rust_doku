use std::env;
use std::str;
use std::process;

fn print_usage_error(code: u32)
{
    if (code == 1)
    {
        println!("ERROR::USAGE:: Enter each row of a sudoku puzzle seperated by spaces, use a '.' for an empty value");
    }
    if (code == 2)
    {
        println!("ERROR::USAGE:: Each row must be exactly 9 characters");
    }
    if (code == 3)
    {
        println!("ERROR::USAGE:: Invalid character in sudoku row");
    }
    process::exit(-1);
}

fn parse_input(args: Vec<String>) -> [[u32; 9]; 9]
{
    let mut puzzle: [[u32; 9]; 9] = [[0; 9]; 9];
    let mut x: usize;

    for y in 1..10
    {
        x = 0;
        if (args[y].len() != 9)
        {
            print_usage_error(2);
        }
        for c in args[y].chars()
        {
            if (c == '.')
            {
                puzzle[(y - 1) as usize][x] = 0;
            }
            else
            {
                //Also handles safe error checking, no problem converting normal chars to int
                let temp = (c as u32) - ('0' as u32);
                if (temp > 0 && temp < 10)
                {
                    puzzle[(y - 1) as usize][x] = temp;
                }
                else
                {
                    println!("ERROR::USAGE:: Invalid character in row {}", y);
                    print_usage_error(3);
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

    if (args.len() - 1 == 0 || args.len() - 1 != 9)
    {
        print_usage_error(1);
    }
    let mut puzzle = parse_input(args);
}
