fn possible_results(position: (usize, usize), board: &[[Option<u8>; 9]]) -> Vec<u8> {
	let mut result = Vec::new();
	(1..=9).for_each(|test_value| {
		let mut flag = true;
		board[position.0].iter().for_each(|cell_value| {
			if Some(test_value) == *cell_value {
				flag = false;
			}
		});
		board.iter().for_each(|row| {
			if Some(test_value) == row[position.1] {
				flag = false;
			}
		});
		let origin = (position.0 / 3, position.1 / 3);
		(0..3).for_each(|i| {
			(0..3).for_each(|j| {
				if Some(test_value) == board[3 * origin.0 + i][3 * origin.1 + j] {
					flag = false;
				}
			})
		});
		if flag {
			result.push(test_value);
		}
	});
	result
}

fn empty_cells(board: [[Option<u8>; 9]; 9]) -> Vec<(usize, usize)> {
	(0..9)
		.map(|i| (0..9).map(move |j| (i, j)))
		.flatten()
		.filter(|(i, j)| board[*i][*j].is_none())
		.collect()
}

fn solve(board: &mut [[Option<u8>; 9]; 9]) {
	#[derive(Debug, Clone)]
	struct Stage {
		board: [[Option<u8>; 9]; 9],
		solutions: Vec<u8>,
		position: (usize, usize),
	}
	let position = empty_cells(*board);
	if position.is_empty() {
		return;
	}
	let position = *position.first().unwrap();
	let mut backtrack = vec![Stage {
		board: *board,
		solutions: possible_results(position, board),
		position,
	}];
	loop {
		if backtrack.len() == 0 {
			eprintln!("No solution found!");
			break;
		}
		let current_stage = backtrack.last_mut().unwrap();
		if current_stage.solutions.len() == 0 {
			backtrack.pop().unwrap().board;
			continue;
		}
		let try_solution = current_stage.solutions.pop().unwrap();
		let try_position = current_stage.position;
		let mut try_board = current_stage.board.clone();
		try_board[try_position.0][try_position.1] = Some(try_solution);
		let pos = empty_cells(try_board);
		if pos.is_empty() {
			*board = try_board;
			return;
		}
		let new_position = *pos.first().unwrap();
		backtrack.push(Stage {
			board: try_board,
			solutions: possible_results(new_position, &try_board),
			position: new_position,
		});
	}
}

fn print(board: &[[Option<u8>; 9]]) {
	println!("┌─────────┐");
	board.iter().for_each(|row| {
		print!("│");
		row.iter().for_each(|cell| {
			if let Some(val) = *cell {
				print!("{val}")
			} else {
				print!("-")
			}
		});
		println!("│");
	});
	println!("└─────────┘");
}
fn main() {
	let mut board = [[Option::<u8>::None; 9]; 9];
	for _ in 0..25 {
		let pos = || rand::random::<usize>() % 8;
		let p = (pos(), pos());
		let solutions = possible_results(p, &board);
		board[p.0][p.1] = Some(solutions[rand::random::<usize>() % solutions.len()]);
	}
	print(&board);
	solve(&mut board);
	print(&board);
}
