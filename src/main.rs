type Board = [[Option<u8>; 9]; 9];
fn possible_results(position: (usize, usize), board: &Board) -> Vec<u8> {
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

fn empty_cells(board: Board) -> Vec<(usize, usize)> {
	(0..9)
		.map(|i| (0..9).map(move |j| (i, j)))
		.flatten()
		.filter(|(i, j)| board[*i][*j].is_none())
		.collect()
}

fn solve(board: &mut Board) -> bool {
	use std::time::Instant;
	let start = Instant::now();
	#[derive(Debug, Clone)]
	struct Stage {
		board: Board,
		solutions: Vec<u8>,
		position: (usize, usize),
	}
	let position = empty_cells(*board);
	if position.is_empty() {
		return true;
	}
	let position = *position.first().unwrap();
	let mut backtrack = vec![Stage {
		board: *board,
		solutions: possible_results(position, board),
		position,
	}];
	loop {
		if start.elapsed().as_secs() > 1 {
			return false;
		}
		if backtrack.len() == 0 {
			return false;
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
			return true;
		}
		let new_position = *pos.first().unwrap();
		backtrack.push(Stage {
			board: try_board,
			solutions: possible_results(new_position, &try_board),
			position: new_position,
		});
	}
}

fn print(board: &Board) {
	println!("╔═══╤═══╤═══╗");
	board.iter().enumerate().for_each(|(i, row)| {
		if (i % 3 == 0 && i != 0) {
			println!("╟───┼───┼───╢")
		}
		print!("║");
		row.iter().enumerate().for_each(|(j, cell)| {
			if (j % 3 == 0 && j != 0) {
				print!("│");
			}
			if let Some(val) = *cell {
				print!("{val}")
			} else {
				print!(" ")
			}
		});
		println!("║");
	});
	println!("╚═══╧═══╧═══╝");
}

use std::{
	io::{prelude::*, BufReader},
	net::{TcpListener, TcpStream},
};

fn main() {
	let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
	for stream in listener.incoming() {
		let mut stream = stream.unwrap();
		println!("Connection Established with {stream:?}");
		let buf_reader = BufReader::new(&mut stream);
		let mut input_board = handle_connection(buf_reader).unwrap();
		println!("Received board: ");
		print(&input_board);
		println!();
		let solved = solve(&mut input_board);
		if solved {
			println!("Solved board: ");
			print(&input_board);
			println!();
		} else {
			println!("No solutions found")
		}
		let mut output = String::with_capacity(82);
		output.push(if solved { '1' } else { '0' });
		input_board.iter().for_each(|row| {
			row.iter()
				.for_each(|cell| output.push_str(cell.unwrap_or(0).to_string().as_str()))
		});
		stream.write_all(output.as_bytes()).unwrap();
	}
}

fn handle_connection(buf_reader: BufReader<&mut TcpStream>) -> Option<Board> {
	let http_request: Vec<_> = buf_reader
		.lines()
		.map(|result| result.unwrap())
		.take_while(|line| !line.is_empty())
		.collect();
	let mut board: Board = [[None; 9]; 9];
	for (index, char) in http_request[0].char_indices() {
		if index >= 81 {
			break;
		}
		if !char.is_numeric() {
			return None;
		}
		let cell_value = char.to_digit(10)? as u8;
		board[index / 9][index % 9] = if cell_value == 0 {
			None
		} else {
			Some(cell_value)
		}
	}
	Some(board)
}
