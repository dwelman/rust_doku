use std::env;

fn parse_input(args: Vec<String>) -> Vec<String>
{
    
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    if (args.len() - 1 == 0 || args.len() - 1 != 9)
    {
        println!("USAGE:: Enter each row of a sudoku puzzle seperated by spaces, use a '.' for an empty value");
    }
}
