use std::fmt;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone, Copy, Default)]
struct GameState {
    my_matter: u32,
    enemy_matter: u32,
}

impl GameState {
    fn update_from_stdin(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        self.my_matter = parse_input!(inputs[0], u32);
        self.enemy_matter = parse_input!(inputs[1], u32);
    }

    fn update() {
        // resolve BUILD
        // MOVE & SPAWN
        // Remove colliding robots

        // Mark tiles

        // Recyclers reduce scraps of tiles
        // Tiles with 0 scraps -> Grass -> Remove units and structures
        // Currency update
        // Check game end
    }

    /// Checks if the game should end
    /// Reasons to end
    /// - a player no longer controls single tile
    /// - 20 turns have passed without any tile changing scraps or owner
    /// - 200 have concluded
    fn check_game_end() {
        // check end
        // check winner?
    }

    /// Player that controls the most tiles
    fn check_winning_player() {}
}

#[derive(Debug, Clone, Copy, Default)]
struct Cell {
    scrap_amount: u32,
    owner: i32,
    units: u32,
    recycler: bool,
    can_build: bool,
    can_spawn: bool,
    in_recycler_range: bool,
}

impl Cell {
    fn update_from_stdin(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        self.scrap_amount = parse_input!(inputs[0], u32);
        self.owner = parse_input!(inputs[1], i32); // 1 = me, 0 = foe, -1 = neutral
        self.units = parse_input!(inputs[2], u32);
        self.recycler = parse_input!(inputs[3], i32) == 1;
        self.can_build = parse_input!(inputs[4], i32) == 1;
        self.can_spawn = parse_input!(inputs[5], i32) == 1;
        self.in_recycler_range = parse_input!(inputs[6], i32) == 1;
    }

    fn is_mine(&self) -> bool {
        self.owner == 1
    }
}

#[derive(Debug, Clone)]
struct Grid {
    my_matter: u32,
    enemy_matter: u32,
    dim: (u32, u32),
    grid: Vec<Cell>,
    mine: Vec<usize>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Self {
        Self {
            my_matter: 10,
            enemy_matter: 10,
            dim: (width, height),
            grid: vec![Cell::default(); (width * height) as usize],
            mine: Vec::with_capacity((width * height) as usize),
        }
    }

    fn from_stdin() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let width = parse_input!(inputs[0], u32);
        let height = parse_input!(inputs[1], u32);
        Self::new(width, height)
    }

    fn update_from_stdin(&mut self) {
        // update currency
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        self.my_matter = parse_input!(inputs[0], u32);
        self.enemy_matter = parse_input!(inputs[1], u32);
        // clear list of owned cells
        self.mine.clear();
        // update cells
        for (i, cell) in self.grid.iter_mut().enumerate() {
            cell.update_from_stdin();
            if cell.is_mine() {
                self.mine.push(i);
            }
        }
    }

    fn get_neighbours(&self, index: usize) -> [Option<usize>; 4] {
        let mut ret = [None; 4];
        let x = index as u32 % self.dim.0;
        let y = index as u32 / self.dim.0;
        //Left
        if x > 0 {
            ret[0] = Some(index - 1);
        }
        //Right
        if x < self.dim.0 - 1 {
            ret[1] = Some(index + 1);
        }
        // Top
        if y > 0 {
            ret[2] = Some(index - self.dim.0 as usize);
        }
        // Bottom
        if y < self.dim.1 - 1 {
            ret[3] = Some(index + self.dim.0 as usize);
        }
        ret
    }

    fn get_xy(&self, index: usize) -> (u32, u32) {
        let x = index as u32 % self.dim.0;
        let y = index as u32 / self.dim.0;
        (x, y)
    }
}

fn check_build(grid: &Grid) -> Option<Action> {
    if grid.my_matter >= 10 {
        for &i in grid.mine.iter() {
            if grid.grid[i].can_build {
                let can = grid.get_neighbours(i).iter().all(|x| {
                    if let Some(idx) = x {
                        grid.grid[*idx].scrap_amount <= grid.grid[i].scrap_amount
                    } else {
                        false
                    }
                });
                if can {
                    let xy = grid.get_xy(i);
                    return Some(Action::Build(xy.0, xy.1));
                }
            }
        }
    }
    None
}

#[derive(Debug)]
enum Action {
    Wait,
    Move(u32, u32, u32, u32, u32),
    Build(u32, u32),
    Spawn(u32, u32, u32),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Action::Wait => write!(f, "WAIT"),
            Action::Move(amount, from_x, from_y, to_x, to_y) => {
                write!(f, "MOVE {} {} {} {} {}", amount, from_x, from_y, to_x, to_y)
            }
            Action::Build(x, y) => write!(f, "BUILD {} {}", x, y),
            Action::Spawn(amount, x, y) => write!(f, "SPAWN {} {} {}", amount, x, y),
        }
    }
}

fn main() {
    let mut grid = Grid::from_stdin();
    let mut action_set: Vec<Action> = Vec::new();
    let mut action_string: String = String::new();
    // game loop
    loop {
        grid.update_from_stdin();

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        action_set.clear();
        if let Some(action) = check_build(&grid) {
            action_set.push(action);
        }
        if action_set.is_empty() {
            println!("{}", Action::Wait);
            continue;
        }
        action_string.clear();
        for action in action_set.iter() {
            print!("{};", action);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn basics() {}

    #[test]
    fn get_neighbours_of_index() {
        let grid = Grid::new(3, 4);
        let i = 0;
        let expected: [Option<usize>; 4] = [None, Some(1), None, Some(3)];
        let res = grid.get_neighbours(i);
        assert_eq!(res, expected, "failed at index={}", i);
        let i = 1;
        let expected: [Option<usize>; 4] = [Some(0), Some(2), None, Some(4)];
        let res = grid.get_neighbours(i);
        assert_eq!(res, expected, "failed at index={}", i);
        let i = 2;
        let expected: [Option<usize>; 4] = [Some(1), None, None, Some(5)];
        let res = grid.get_neighbours(i);
        assert_eq!(res, expected, "failed at index={}", i);
        let i = 3;
        let expected: [Option<usize>; 4] = [None, Some(4), Some(0), Some(6)];
        let res = grid.get_neighbours(i);
        assert_eq!(res, expected, "failed at index={}", i);
        let i = 4;
        let expected: [Option<usize>; 4] = [Some(3), Some(5), Some(1), Some(7)];
        let res = grid.get_neighbours(i);
        assert_eq!(res, expected, "failed at index={}", i);
        let i = 5;
        let expected: [Option<usize>; 4] = [Some(4), None, Some(2), Some(8)];
        let res = grid.get_neighbours(i);
        assert_eq!(res, expected, "failed at index={}", i);
        let i = 9;
        let expected: [Option<usize>; 4] = [None, Some(10), Some(6), None];
        let res = grid.get_neighbours(i);
        assert_eq!(res, expected, "failed at index={}", i);
        let i = 10;
        let expected: [Option<usize>; 4] = [Some(9), Some(11), Some(7), None];
        let res = grid.get_neighbours(i);
        assert_eq!(res, expected, "failed at index={}", i);
        let i = 11;
        let expected: [Option<usize>; 4] = [Some(10), None, Some(8), None];
        let res = grid.get_neighbours(i);
        assert_eq!(res, expected, "failed at index={}", i);
    }
}
