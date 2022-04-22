use std::collections::VecDeque;
use std::fmt;
use std::io;

macro_rules! parse_input {
	($x:expr, $t:ident) => {
		$x.trim().parse::<$t>().unwrap()
	};
}

fn read_simple_input<T>() -> T
where
	T: std::str::FromStr,
	<T as std::str::FromStr>::Err: std::fmt::Debug,
{
	let mut input_line = String::new();
	io::stdin().read_line(&mut input_line).unwrap();
	let ret = parse_input!(input_line, T);
	ret
}

#[derive(Debug, Clone, Copy)]
struct Cell {
	water: bool,
	lake_size: usize,
	visited: bool,
}

impl Cell {
	fn from_char(c: &char) -> Self {
		if *c == '#' {
			Self {
				water: false,
				lake_size: 0,
				visited: false,
			}
		} else {
			Self {
				water: true,
				lake_size: 0,
				visited: false,
			}
		}
	}

	fn is_land(&self) -> bool {
		!self.water
	}

	fn is_water(&self) -> bool {
		self.water
	}

	fn is_known(&self) -> (bool, usize) {
		match self.is_water() {
			true => (self.visited, self.lake_size),
			false => (true, 0),
		}
	}
}

#[derive(Debug, Clone)]
struct Map {
	width: usize,
	heigth: usize,
	grid: Vec<Vec<Cell>>,
}

impl Map {
	pub fn read_input() -> Self {
		let width = read_simple_input::<usize>();
		let heigth = read_simple_input::<usize>();
		let mut grid: Vec<Vec<Cell>> = Vec::new();
		let mut input_line = String::new();
		for _ in 0..heigth {
			let mut line: Vec<Cell> = Vec::new();
			input_line.clear();
			io::stdin().read_line(&mut input_line).unwrap();
			let line_str = input_line.trim_matches('\n').to_string();
			eprintln!("{}", line_str);
			for c in line_str.chars() {
				line.push(Cell::from_char(&c));
			}
			grid.push(line);
		}
		Self {
			width,
			heigth,
			grid,
		}
	}

	pub fn neighbours(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
		let mut neighbours: Vec<(usize, usize)> = Vec::new();
		// Left
		if coords.0 > 0 {
			neighbours.push((coords.0 - 1, coords.1))
		}
		// Right
		if coords.0 < self.width - 1 {
			neighbours.push((coords.0 + 1, coords.1))
		}
		// Up
		if coords.1 > 0 {
			neighbours.push((coords.0, coords.1 - 1))
		}
		// Down
		if coords.1 < self.heigth - 1 {
			neighbours.push((coords.0, coords.1 + 1))
		}
		neighbours
	}

	pub fn floodfill(&mut self, coords: (usize, usize)) -> usize {
		let status = self.is_known(&coords);
		if status.0 {
			eprintln!("Is known = {}", status.1);
			return status.1;
		}
		let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
		let mut cache: Vec<(usize, usize)> = Vec::new();
		queue.push_back(coords);
		self.visit(&coords);
		let mut lake_size: usize = 1;
		while !queue.is_empty() {
			let current = queue.pop_front().unwrap();
			let ngbs = self.neighbours(current);
			for n in ngbs.iter() {
				if self.is_unvisited_water(n) {
					lake_size += 1;
					queue.push_back(*n);
					self.visit(n);
				}
			}
			cache.push(current);
		}
		for c in cache.iter() {
			self.update_cell(c, lake_size)
		}
		return lake_size;
	}

	fn is_land(&self, coords: &(usize, usize)) -> bool {
		self.grid[coords.1][coords.0].is_land()
	}

	fn is_water(&self, coords: &(usize, usize)) -> bool {
		self.grid[coords.1][coords.0].is_water()
	}

	fn is_known(&self, coords: &(usize, usize)) -> (bool, usize) {
		self.grid[coords.1][coords.0].is_known()
	}

	fn is_unvisited_water(&self, coords: &(usize, usize)) -> bool {
		self.grid[coords.1][coords.0].is_water() && !self.grid[coords.1][coords.0].visited
	}

	fn visit(&mut self, coords: &(usize, usize)) {
		self.grid[coords.1][coords.0].visited = true;
	}

	fn update_cell(&mut self, coords: &(usize, usize), lake_size: usize) {
		self.grid[coords.1][coords.0].lake_size = lake_size;
	}
}

impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})\n{:?}", self.width, self.heigth, self.grid)
	}
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
	let mut map = Map::read_input();
	let n = read_simple_input::<usize>();
	for _ in 0..n {
		let mut input_line = String::new();
		io::stdin().read_line(&mut input_line).unwrap();
		let inputs = input_line.split(" ").collect::<Vec<_>>();
		let x = parse_input!(inputs[0], usize);
		let y = parse_input!(inputs[1], usize);
		eprintln!("Start Point {:?}", (x, y));
		println!("{}", map.floodfill((x, y)));
	}
}
