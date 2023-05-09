#[cfg(test)]
mod tests;

use std::fmt;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
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
    others: Vec<(u32, u32)>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Self {
        Self {
            my_matter: 10,
            enemy_matter: 10,
            dim: (width, height),
            grid: vec![Cell::default(); (width * height) as usize],
            mine: Vec::with_capacity((width * height) as usize),
            others: Vec::with_capacity((width * height) as usize),
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
        // clear lists
        self.mine.clear();
        self.others.clear();

        // update cells
        for (i, cell) in self.grid.iter_mut().enumerate() {
            cell.update_from_stdin();
            if cell.is_mine() {
                self.mine.push(i);
            } else {
                let x = i as u32 % self.dim.0;
                let y = i as u32 / self.dim.0;
                self.others.push((x, y));
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

/// This function aims to build at such tile that the recycler doesn't
/// disappear before all neighbouring tiles are without scrap
/// probably not useful from a strategical point of view?
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

fn dist_squared(from: (u32, u32), to: (u32, u32)) -> u32 {
    let dx = to.0 as i32 - from.0 as i32;
    let dy = to.1 as i32 - from.1 as i32;
    (dx * dx + dy * dy) as u32
}
/// First attemt at spawning
/// General idea is to spawn at a tile closest to most empty and enemy tiles
fn check_spawn(grid: &Grid) -> Option<Action> {
    if grid.my_matter >= 10 {
        let mut max_other = ((0, 0), u32::MAX);
        for &i in grid.mine.iter() {
            if grid.grid[i].can_spawn {
                let xy = grid.get_xy(i);
                let td: u32 = grid
                    .others
                    .iter()
                    .fold(0, |acc, &pos| acc + dist_squared(xy, pos));
                if td < max_other.1 {
                    max_other.0 = xy;
                    max_other.1 = td;
                }
            }
        }
        if max_other.1 < u32::MAX {
            return Some(Action::Spawn(1, max_other.0 .0, max_other.0 .1));
        }
    }
    None
}

/// First attemt at moving
/// General idea is to move to a tile closest to most empty and enemy tiles
fn check_move(grid: &Grid, from: (u32, u32), amount: u32) -> Option<Action> {
    if grid.my_matter >= 10 {
        let mut max_other = ((0, 0), u32::MAX);
        for &xy in grid.others.iter() {
            let td: u32 = grid
                .others
                .iter()
                .fold(0, |acc, &pos| acc + dist_squared(xy, pos));
            if td < max_other.1 {
                max_other.0 = xy;
                max_other.1 = td;
            }
        }

        if max_other.1 < u32::MAX {
            return Some(Action::Move(
                amount,
                from.0,
                from.1,
                max_other.0 .0,
                max_other.0 .1,
            ));
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
        // move first
        for &i in grid.mine.iter() {
            if grid.grid[i].units > 0 {
                let from_xy = grid.get_xy(i);
                if let Some(action) = check_move(&grid, from_xy, grid.grid[i].units) {
                    action_set.push(action);
                    // should update grid here to enable build and spawn in newly free locations
                }
            }
        }
        // update grid with moves
        // build to block?
        // spawn if there's enough credits
        if let Some(action) = check_spawn(&grid) {
            action_set.push(action);
        }
        // if let Some(action) = check_build(&grid) {
        //     action_set.push(action);
        //     // update grid with the build action
        //     // check for additional builds?
        // }
        // spawn decision or move decision
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
