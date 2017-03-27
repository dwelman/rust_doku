use error;

pub fn solve_puzzle(puzzle: &mut [[u32; 9]; 9])
{
	puzzle_loop(puzzle, 0, 0);
	if validate_puzzle(puzzle) == true
	{
		print_puzzle(puzzle);
		println!("Puzzle complete!");
	}
	else
	{
		println!("Puzzle cannot be solved");
	}
}

//The recursive loop that solves the puzzle
fn puzzle_loop(puzzle: &mut [[u32; 9]; 9], y: usize, x: usize) -> bool
{
	if y == 9
	{
		return true
	}
	if puzzle[y][x] == 0
	{
		for n in 1..10
		{
			puzzle[y][x] = n;
			if validate_specific_val(puzzle, y, x) == true
			{
				if x == 8
				{
					if puzzle_loop(puzzle, y + 1, 0) == true
					{
						return true
					}
				}
				else
				{
					if puzzle_loop(puzzle, y, x + 1) == true
					{
						return true
					}
				}
			}
		}
		puzzle[y][x] = 0;
		return false
	}
	else
	{
		if x == 8
		{
			if puzzle_loop(puzzle, y + 1, 0) == true
			{
				return true
			}
		}
		else
		{
			if puzzle_loop(puzzle, y, x + 1) == true
			{
				return true
			}
		}
	}
	return false
}

pub fn print_puzzle(puzzle: &[[u32; 9]; 9])
{
	for y in 0..9
	{
		println!("{}{}{} | {}{}{} | {}{}{}", puzzle[y][0], puzzle[y][1], puzzle[y][2], puzzle[y][3], puzzle[y][4], puzzle[y][5], puzzle[y][6], puzzle[y][7], puzzle[y][8]);
		if (y + 1) % 3 == 0 && y != 8
		{
			println!("----+-----+----");
		}
	}
}

//Validates the entire puzzle
pub fn validate_puzzle(puzzle: &[[u32; 9]; 9]) -> bool
{
	for y in 0..9
	{
		for x in 0..9
		{
			if validate_specific_val(puzzle, y, x) == false
			{
				return false
			}
		}
	}
	true
}

//Validates a specific piece of the puzzle
fn validate_specific_val(puzzle: &[[u32; 9]; 9], y: usize, x: usize) -> bool
{
	//For the current column
	for c in 0..9
	{
		//Ignore the piece we are validating
		if y != c
		{
			if puzzle[y][x] == puzzle[c][x]
			{
				return false
			}
		}
	}
	//For the current row
	for r in 0..9
	{
		//Ignore the piece we are validating
		if x != r
		{
			if puzzle[y][x] == puzzle[y][r]
			{
				return false
			}
		}
	}
	//For the current block
	let b_y: usize = (y / 3) + 1;
	let b_x: usize = (x / 3) + 1;
	for c_y in ((b_y - 1) * 3)..((b_y * 3) - 1)
	{
		for c_x in ((b_x - 1) * 3)..((b_x * 3) - 1)
		{
			//Ignore the piece we are validating
			if c_y != y && c_x != x
			{
				if puzzle[y][x] == puzzle[c_y][c_x]
				{
					return false
				}
			}
		}
	}
	true
}