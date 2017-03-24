pub fn solve_puzzle(puzzle: &mut [[u32; 9]; 9])
{
	print_puzzle(puzzle);
}

pub fn print_puzzle(puzzle: &[[u32; 9]; 9])
{
	for y in 0..9
	{
		println!("{}{}{} | {}{}{} | {}{}{}", puzzle[y][0], puzzle[y][1], puzzle[y][2], puzzle[y][3], puzzle[y][4], puzzle[y][5], puzzle[y][6], puzzle[y][7], puzzle[y][8]);
		if ((y + 1) % 3 == 0 && y != 8)
		{
			println!("----+-----+----");
		}
	}
}

pub fn validate_puzzle(puzzle: &[[u32; 9]; 9]) -> bool
{
	
}