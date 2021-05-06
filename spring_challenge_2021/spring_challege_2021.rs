use std::io;


// Structures
// - board
// - tile
// - player?
// Plan
// - add enum for richness values to consts
// - read input
// - parse actions
// - implement board - distance between cells (if needed since given list of actions)

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
// Constants
const TREE_SUN_POINTS: i32 = 3;
const TREE_LIFE_COST: i32 = 4;
const FOREST_INITIAL_NUTRIENT: i32 = 20;
const POINTS_PER_3SUN: i32 = 1;
const GAME_LENGTH: i32 = 24;
const BOARD_SIZE: i32 = 37;


// Struct Cell
struct Cell {
    index: i32,
    richness: i32,
    tree: Option<Tree>,
    neighbours: [i32; 6],
}

impl Cell {
    pub fn new() -> Cell {

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let index = parse_input!(inputs[0], i32); // 0 is the center cell, the next cells spiral outwards
        let richness = parse_input!(inputs[1], i32); // 0 if the cell is unusable, 1-3 for usable cells
        let neigh_0 = parse_input!(inputs[2], i32); // the index of the neighbouring cell for each direction
        let neigh_1 = parse_input!(inputs[3], i32);
        let neigh_2 = parse_input!(inputs[4], i32);
        let neigh_3 = parse_input!(inputs[5], i32);
        let neigh_4 = parse_input!(inputs[6], i32);
        let neigh_5 = parse_input!(inputs[7], i32);
        Cell {
            index: index,
            richness: richness,
            tree: None,
            neighbours: [neigh_0, neigh_1, neigh_2, neigh_3, neigh_4, neigh_5],
        }
    }
}

// Board
struct Board {
    board: Vec<Cell>,
}

impl Board {
    pub fn new() -> Board {
        let mut new_board: Board = Board {
            board: Vec::with_capacity(10),
        };
        for _ in 0..BOARD_SIZE as usize {
            new_board.board.push(Cell::new());
        }
        return new_board;
    }

    pub fn print_dbg(&self) {
        for i in 0..BOARD_SIZE as usize {
            eprintln!("{}: {}", i, self.board[i].index);
        }
    }
}

// Tree
struct Tree {
    owner: Player,
    sun_points: i32,
    cell_index: i32,
    size: i32,
    is_mine: bool,
    is_dormant: bool,
}

// Player - is it needed?
struct Player {
    sun: i32,
    score: i32,
    waiting: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            sun: 0,
            score: 0,
            waiting: false,
        }
    }

    pub fn update(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        self.sun = parse_input!(inputs[0], i32);
        self.score = parse_input!(inputs[1], i32);
        if inputs.len() > 2 {
            self.waiting = parse_input!(inputs[2], i32) != 0;
        } else {
            self.waiting = false;
        }
    }
}

// Actions
struct Actions {
    action_string: String,
}

// Game
struct Game {
    day: i32,
    nutrients: i32,
    board: Board,
    actions: Vec<Actions>,
    me: Player,
    opponent: Player,
}

impl Game {
    pub fn new() -> Game {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_cells = parse_input!(input_line, i32);
        assert!(number_of_cells == BOARD_SIZE, "Board size input is not default.");
        // Read Board
        let board: Board = Board::new();
        // Return Game Struct
        Game {
            day: 0,
            nutrients: FOREST_INITIAL_NUTRIENT,
            board: board,
            actions: Vec::new(),
            me: Player::new(),
            opponent: Player::new(),
        }
    }

    pub fn update(&mut self) {
        // Consider using input_line.clear() instead of new all the time

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        self.day = parse_input!(input_line, i32); // the game lasts 24 days: 0-23

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        self.nutrients = parse_input!(input_line, i32); // the base score you gain from the next COMPLETE action

        self.me.update();
        self.opponent.update();
    } 
}


/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // Initialize game
    let mut game: Game = Game::new();

    // game loop
    loop {
        game.update();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_trees = parse_input!(input_line, i32); // the current amount of trees
        for i in 0..number_of_trees as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let cell_index = parse_input!(inputs[0], i32); // location of this tree
            let size = parse_input!(inputs[1], i32); // size of this tree: 0-3
            let is_mine = parse_input!(inputs[2], i32); // 1 if this is your tree
            let is_dormant = parse_input!(inputs[3], i32); // 1 if this tree is dormant
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_possible_moves = parse_input!(input_line, i32);
        for i in 0..number_of_possible_moves as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let possible_move = input_line.trim_matches('\n').to_string();
            eprintln!("{}", possible_move);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        println!("WAIT");
    }
}
