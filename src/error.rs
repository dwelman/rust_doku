use std::process;

pub fn print_usage_error(code: u32)
{
    if code == 1
    {
        println!("ERROR::USAGE:: Enter each row of a sudoku puzzle seperated by spaces, use a '.' for an empty value");
    }
    if code == 2
    {
        println!("ERROR::USAGE:: Each row must be exactly 9 characters");
    }
    if code == 3
    {
        println!("ERROR::USAGE:: Invalid character in sudoku row");
    }
    if code == 4
    {
        println!("ERROR::USAGE:: Invalid sudoku puzzle");
    }
    process::exit(-1);
}