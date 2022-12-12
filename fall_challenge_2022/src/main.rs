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
}

// enum Cell {
//     Grass,
//     Me(u32),
//     Enemy(u32),
//     MyRecycler,
//     EnemyRecycler,
// }

// impl Cell {
//     fn is_mine(&self) -> bool {
//         match *self {
//             Cell::Grass => false,
//             Cell::Me(_) => true,
//             Cell::Enemy(_) => false,
//             Cell::MyRecycler => true,
//             Cell::EnemyRecycler => false,
//         }
//     }

//     fn is_enemys(&self) -> bool {
//         match *self {
//             Cell::Grass => false,
//             Cell::Me(_) => false,
//             Cell::Enemy(_) => true,
//             Cell::MyRecycler => false,
//             Cell::EnemyRecycler => true,
//         }
//     }
// }

struct Grid {
    dim: (u32, u32),
    grid: Vec<Cell>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Self {
        Self {
            dim: (width, height),
            grid: vec![Cell::default(); (width * height) as usize],
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
        for mut cell in self.grid.iter_mut() {
            cell.update_from_stdin();
        }
    }
}

enum Action {
    Wait,
    Move(u32, u32, u32, u32, u32),
    Build(u32, u32),
    Spawn(u32, u32, u32),
}

fn main() {
    let mut state = GameState::default();
    let mut grid = Grid::from_stdin();
    // game loop
    loop {
        state.update_from_stdin();
        grid.update_from_stdin();

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("WAIT");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {}
}
