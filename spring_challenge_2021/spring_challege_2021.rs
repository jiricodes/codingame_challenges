use std::io;
use std::cmp;
use std::fmt;

// Plan
// - add enum for richness values to consts
// - parse actions
// - implement board - distance between cells (if needed since given list of actions)

// Strat notes
// - prio seed (correct slot) > grow > harvest > wait
// - some kind of simulation is needed. if I don't gain enough to make difference by having a tree, then it should be completed, otherwise the enemy removes nutritions
// - spawning seems to be on the outer ring, with 2 trees of level 1.
// - grow trees in positions so they dont cast shadows on each other - one step in direction and then one more in dir +1 or -1
// - make sure to harvest some trees early
// - never harvest all trees unless end of game - check if harvesting gains more points than keeping sun points (harvest all athe end?)


// TODO
// - create and action struct
// - action should self evaluate itself somehow and return Option(value), if returned None then "WAIT"
// - add seeding and growing lvl1 to naive
// - think of harvesting strat
// - add initial steps

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
// Constants
const TREE_LIFECYCLE_COST: i32 = 4;
const FOREST_INITIAL_NUTRIENT: i32 = 20;
const POINTS_PER_3SUN: i32 = 1;
const GAME_LENGTH: i32 = 24;
const BOARD_SIZE: i32 = 37;

// Settings
const MIN_TREE3_N: i32 = 3;
const MAX_TREE0_N: i32 = 3;
const MAX_TREES: i32 = 12;

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

    pub fn reset_tree(&mut self) {
        self.tree = None;
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tree.is_some() {
            write!(f, "Cell: {}\nRich: {}\nTree: {}\n", self.index, self.richness, self.tree.as_ref().unwrap())
        } else {
            write!(f, "Cell: {}\nRich: {}\nTree: None\n", self.index, self.richness)
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
        for cell in self.board.iter() {
            eprintln!("{}", cell);
        }
    }

    pub fn get_cell_richness_points(&self, index: usize) -> i32 {
        let value = (self.board[index].richness - 1) * 2;
        cmp::max(0, value)
    }

    pub fn reset_trees(&mut self) {
        for cell in self.board.iter_mut() {
            cell.reset_tree();
        }
    }
}

// Tree
struct Tree {
    cell_index: i32,
    size: i32,
    is_mine: bool,
    is_dormant: bool,
}

impl Tree {
    pub fn new() -> Tree {
        let mut new_tree = Tree {
            cell_index: 0,
            size: 0,
            is_mine: false,
            is_dormant: false,
        };
        let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            new_tree.cell_index = parse_input!(inputs[0], i32); // location of this tree
            new_tree.size = parse_input!(inputs[1], i32); // size of this tree: 0-3
            new_tree.is_mine = parse_input!(inputs[2], i32) != 0; // 1 if this is your tree
            new_tree.is_dormant = parse_input!(inputs[3], i32) != 0; // 1 if this tree is dormant
        return new_tree;
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tree: {}\nSize: {}\nis_mine: {}\nis_dormant: {}", self.cell_index, self.size, self.is_mine, self.is_dormant)
    }
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
struct Action {
    action_string: String,
    command: String, //should be changed to enum later
    cell_index: i32,
    target_index: i32,
}

impl Action {
    pub fn new() -> Action {
        let mut action = Action {
            action_string: String::new(),
            command: String::new(),
            cell_index: -1,
            target_index: -1,
        };
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        action.action_string = input_line.trim_matches('\n').to_string();
        let inputs = action.action_string.split(" ").collect::<Vec<_>>();
        action.command = inputs[0].to_string();
        if inputs.len() > 1 {
            action.cell_index = parse_input!(inputs[1], i32);
            if inputs.len() > 2 {
                action.target_index = parse_input!(inputs[2], i32);
            }
        }
        return action;
    }

    pub fn exec(&self) {
        println!("{}", self.action_string);
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Action: {}\nOrigin: {}\nTarget: {}", self.command, self.cell_index, self.target_index)
    }
}

// Game
struct Game {
    day: i32,
    nutrients: i32,
    board: Board,
    trees: Vec<Tree>,
    actions: Vec<Action>,
    me: Player,
    opponent: Player,
    ntree0: i32,
    ntree1: i32,
    ntree2: i32,
    ntree3: i32,
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
            trees: Vec::new(),
            actions: Vec::new(),
            me: Player::new(),
            opponent: Player::new(),
            ntree0: 0,
            ntree1: 0,
            ntree2: 0,
            ntree3: 0,
        }
    }

    pub fn update(&mut self) {
        // self.reset();
        // Consider using input_line.clear() instead of new all the time

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        self.day = parse_input!(input_line, i32); // the game lasts 24 days: 0-23

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        self.nutrients = parse_input!(input_line, i32); // the base score you gain from the next COMPLETE action

        self.me.update();
        self.opponent.update();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_trees = parse_input!(input_line, i32); // the current amount of trees
        for i in 0..number_of_trees as usize {
            self.trees.push(Tree::new());
            if self.trees[i].is_mine {
                match self.trees[i].size {
                    0 => self.ntree0 += 1,
                    1 => self.ntree1 += 1,
                    2 => self.ntree2 += 1,
                    3 => self.ntree3 += 1,
                    _ => {},
                }
            }
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_possible_moves = parse_input!(input_line, i32);
        for i in 0..number_of_possible_moves as usize {
            self.actions.push(Action::new());
        }
    }

    pub fn reset(&mut self) {
        self.trees.clear();
        self.actions.clear();
        self.ntree0 = 0;
        self.ntree1 = 0;
        self.ntree2 = 0;
        self.ntree3 = 0;
        self.board.reset_trees();
    }

    pub fn naive_move(&mut self) {
        let mut gain: i32 = -2 * self.me.sun;
        let mut action_index: usize = 0;
        for (i, action) in self.actions.iter().enumerate() {
            let mut current_gain = -2 * self.me.sun;
            if action.command == "COMPLETE" && (self.day > 20 || self.ntree3 > MIN_TREE3_N) {
                current_gain = self.nutrients;
                current_gain += self.board.get_cell_richness_points(action.cell_index as usize);
            } else if action.command == "GROW" {
                let size = self.get_tree_size(action.cell_index);
                let cost: i32 = match size {
                    0 => 1 + self.ntree1,
                    1 => 3 + self.ntree2,
                    2 => 7 + self.ntree3,
                    _ => 1000000,
                };
                // eprintln!("{} for {} ({})", action.action_string, cost, self.me.sun);
                current_gain = self.board.get_cell_richness_points(action.cell_index as usize) - cost + size;
            } else if action.command == "WAIT" {
                current_gain = -2 * self.me.sun + 1;
            } else if action.command == "SEED" && self.ntree0 < MAX_TREE0_N && (self.ntree0 + self.ntree1 + self.ntree2 + self.ntree3) < MAX_TREES {
                // check for shadows
                current_gain = self.board.get_cell_richness_points(action.target_index as usize) - self.ntree0;
            }
            // eprintln!("{} gains {} ({})", action.action_string, current_gain, gain);
            if current_gain > gain {
                gain = current_gain;
                action_index = i;
            }
        }
        self.actions[action_index].exec();
    }

    fn get_tree_size(&self, cell: i32) -> i32 {
        for tree in self.trees.iter() {
            if tree.cell_index == cell { return tree.size; }
        }
        return 0;
    }

    fn initial_stage(&mut self) {
        match self.day {
            0 => { println!("WAIT"); },
            1 => { 
            eprintln!("grow one");
            println!("WAIT");
            },
            2 => {
                eprintln!("grow another one");
                println!("WAIT");
            },
            3=> {
                eprintln!("plant two seeds");
                println!("WAIT");
            },
            4 => {
                eprintln!("grow one seed");
                println!("WAIT");
            },
            5 => {
                eprintln!("grow other seed and plant one in center (from lvl1)");
                println!("WAIT");
            },
            _ => {
                eprintln!("other case");
                println!("WAIT");
            }
        }
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
        game.board.print_dbg();

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        // println!("WAIT");
        // if game.day < 6 {
        //     game.initial_stage();
        // } else {
            game.naive_move();
        // }

        /// RESET STATE
        game.reset();

    }
}
