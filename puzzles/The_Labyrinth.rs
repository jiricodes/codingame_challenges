/// Approach
/// - Move towards closest ? until control room found
/// - once control room found
///     - if path doesn't exist or path control->target longer than timeout
///         - then keep discovering map
///     - else
///         - search shortest path to control
/// - once control room reached find path to target
/// 




use std::io;
use std::convert::TryInto;
use std::fmt;
use std::collections::VecDeque;
use std::ops::Sub;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Eq for Point {}

impl Point {
    pub fn new(x: i32, y:i32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn update(&mut self, x: i32, y:i32) {
        self.x = x;
        self.y = y;
    }

    fn in_bounds(&self, xmax: i32, ymax: i32) -> bool {
        self.x >= 0 && self.x < xmax && self.y >= 0 && self.y < ymax
    }

    fn get_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn get_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn get_up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn get_down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn get_neighbours(&self) -> [Point; 4] {
        [self.get_right(), self.get_left(), self.get_up(), self.get_down()]
    }

    // supports only 4 main directions
    fn get_direction(&self, other: &Point) -> &str {
        let dir = *other - *self;
        if dir.x < 0 {
            return "LEFT";
        }
        else if dir.x > 0 {
            return "RIGHT";
        }
        else if dir.y < 0 {
            return "UP";
        }
        else {
            return "DOWN";
        }
    }
}

struct Kirk {
    location:       Point,
    timeout:        i32,
    control_room:   bool,
    path:           VecDeque<Point>,
}

impl Kirk {
    pub fn new(timeout: i32) -> Self {
        Self {
            location: Point::new(0, 0),
            timeout: timeout,
            control_room: false,
            path: VecDeque::new(),
        }
    }

    pub fn update(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let kr = parse_input!(inputs[0], i32); // row where Kirk is located.
        let kc = parse_input!(inputs[1], i32); // column where Kirk is located.
        self.location.update(kc, kr);
    }

    pub fn make_move(&mut self, map: &mut Map) {
        // Set current visited
        map.mark_visited(&self.location);
        // Check if currently in Control Room
        self.control_room = self.control_room | map.is_value(&self.location, 'C');
        // Update State
        // self.update_state(map);
        let mut kirk_move = "RIGHT";
        if !self.path.is_empty() {
            eprintln!("Should follow my path {:?}", self.path);
            if self.control_room {
                self.timeout -= 1;
            }
            kirk_move = self.move_on_path();
        }
        else if map.control_room.is_none() {
            eprintln!("Should search map efficiently");
            self.path = map.bfs_to_fog(&self.location);
            kirk_move = self.move_on_path();
        }
        else if !self.control_room {
            eprintln!("Should find path to the control room {}", map.control_room.as_ref().unwrap());
            self.path = map.bfs(&self.location, &map.control_room.unwrap());
            // Keep discovering
            let ct_len = map.path_to_target_from_control().len();
            eprintln!("CT {} | TIME {}", ct_len, self.timeout);
            if self.path.is_empty() || ct_len > (self.timeout as usize) {
                eprintln!("Haven't found path to the control room, gonna explore more map!");
                self.path = map.bfs_to_fog(&self.location);
            }
            // Path Move
            kirk_move = self.move_on_path();
        }
        else {
            eprintln!("Should find path to the Target {}", &map.target.as_ref().unwrap());
            self.path = map.bfs(&self.location, &map.target.unwrap());
            eprintln!("Path found. Length {}, timeout {}", self.path.len(), self.timeout);
            self.timeout -= 1;
            kirk_move = self.move_on_path();
        }
        // else search path to Target
        /// Return output value
        println!("{}", kirk_move); // Kirk's next move (UP DOWN LEFT or RIGHT).
    }

    fn check_neighbours(&self, map: &Map) -> Option<&str> {
        //Right
        let mut next_cell: Point = self.location.get_right();
        if map.is_not_visited_path(&next_cell) {
                return Some("RIGHT"); }
        //Left
        let mut next_cell: Point = self.location.get_left();
        if map.is_not_visited_path(&next_cell) {
                return Some("LEFT"); }
        // Up
        let mut next_cell: Point = self.location.get_up();
        if map.is_not_visited_path(&next_cell) {
                return Some("UP"); }
        // Down
        let mut next_cell: Point = self.location.get_down();
        if map.is_not_visited_path(&next_cell) {
                return Some("DOWN"); }
        None
    }

    fn move_on_path(&mut self) -> &str {
        if !self.path.is_empty() {
        let next_tile = self.path.pop_front().unwrap();
        return self.location.get_direction(&next_tile);
        }
        "PATH EMPTY"
    }
}


struct Map {
    r:              i32,
    c:              i32,
    grid:           Vec<String>,
    target:         Option<Point>,
    control_room:   Option<Point>,
    visited:        Vec<Vec<bool>>,
}

impl Map {
    pub fn new(r: i32, c: i32) -> Self {
        let mut grid: Vec<String> = Vec::new();
        for i in 0..r as usize {
            grid.push(String::new());
        }
        let mut new = Self {
            r: r,
            c: c,
            grid: grid,
            target: None,
            control_room: None,
            visited: Vec::new(),
        };
        for _i in 0..r {
            let mut line: Vec<bool> = Vec::new();
            for _j in 0..c {
                line.push(false);
            }
            new.visited.push(line);
        }
        new
    }

    pub fn update(&mut self) {
        for i in 0..self.r as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            self.grid[i] = input_line.trim().to_string(); // C of the characters in '#.TC?' (i.e. one line of the ASCII maze).
            let tmp = self.grid[i].find('T');
            if tmp.is_some() && self.target.is_none() {
                eprintln!("Found T @{},{}", tmp.unwrap() as i32, i);
                self.target = Some(Point::new(tmp.unwrap() as i32, i as i32));
            }
            let tmp = self.grid[i].find('C');
            if tmp.is_some() && self.control_room.is_none() {
                eprintln!("Found C @{},{}", tmp.unwrap() as i32, i);
                self.control_room = Some(Point::new(tmp.unwrap() as i32, i as i32));
            }
        }

    }

    pub fn eprint(&self) {
        eprintln!("{:?}",(self.r, self.c, self.target.as_ref(), self.control_room.as_ref()));
        for i in 0..self.r as usize {
            eprintln!("{}", &self.grid[i]);
        }
    }

    fn is_in_bounds(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.c && p.y >= 0 && p.y < self.r
    }

    pub fn is_path(&self, p: &Point) -> bool {
        p.in_bounds(self.c, self.r) && self.grid[p.y as usize].chars().nth(p.x as usize).unwrap() == '.'
    }

    pub fn is_value(&self, p: &Point, value: char) -> bool {
        p.in_bounds(self.c, self.r) && self.grid[p.y as usize].chars().nth(p.x as usize).unwrap() == value
    }

    fn mark_visited(&mut self, p: &Point) {
        self.visited[p.y as usize][p.x as usize] = true;
    }

    fn is_not_visited_path(&self, p: &Point) -> bool {
        self.is_path(p) && !self.visited[p.y as usize][p.x as usize]
    }

    fn is_not_visited_tile(&self, p: &Point) -> bool {
        (self.is_path(p) || self.is_control_room(p) || self.is_target(p)) && !self.visited[p.y as usize][p.x as usize]
    }

    fn is_control_room(&self, p: &Point) -> bool {
        self.is_value(p, 'C')
    }

    fn is_target(&self, p: &Point) -> bool {
        self.is_value(p, 'T')
    }

    fn is_fog(&self, p: &Point) -> bool {
        self.is_value(p, '?')
    }

    fn reset_visited(&mut self) {
        for line in self.visited.iter_mut() {
            for cell in line.iter_mut() {
                *cell = false;
            }
        }
    }

    fn bfs(&mut self, start: &Point, end: &Point) -> VecDeque<Point> {
        //
        eprint!("Path from {} to {}: ", start, end);
        // clear visited
        self.reset_visited();
        // do bfs
        let mut queue: VecDeque<VecDeque<Point>> = VecDeque::new();
        let mut start_path: VecDeque<Point> = VecDeque::new();
        start_path.push_back(*start);
        queue.push_back(start_path);
        while !queue.is_empty() {
            let mut path = queue.pop_front().unwrap();
            let current = path.back().unwrap();
            if current == end {
                path.pop_front();
                eprintln!("found! {:?}", path);
                return path;
            }
            for n in current.get_neighbours().iter() {
                if self.is_not_visited_tile(n) {
                    let mut new_path = path.clone();
                    self.mark_visited(&n);
                    new_path.push_back(*n);
                    queue.push_back(new_path);
                }
            }
        }
        eprintln!("not found :(");
        VecDeque::new()
    }

    fn path_to_control(&mut self, start: &Point) -> VecDeque<Point> {
        self.bfs(start, &self.control_room.unwrap())
    }

    fn path_to_target_from_control(&mut self) -> VecDeque<Point> {
        self.bfs(&self.control_room.unwrap(), &self.target.unwrap())
    }

    fn bfs_to_fog(&mut self, start: &Point) -> VecDeque<Point> {
        //
        eprint!("Path from {} to ?: ", start);
        // clear visited
        self.reset_visited();
        // do bfs
        let mut queue: VecDeque<VecDeque<Point>> = VecDeque::new();
        let mut start_path: VecDeque<Point> = VecDeque::new();
        start_path.push_back(*start);
        queue.push_back(start_path);
        while !queue.is_empty() {
            let mut path = queue.pop_front().unwrap();
            let current = path.back().unwrap();
            for n in current.get_neighbours().iter() {
                if self.is_fog(n) {
                    path.pop_front();
                    eprintln!("found! {:?}", path);
                    return path;
                }
                if self.is_not_visited_path(n) || self.is_fog(n) || self.is_target(n) {
                    let mut new_path = path.clone();
                    self.mark_visited(&n);
                    new_path.push_back(*n);
                    queue.push_back(new_path);
                }
            }
        }
        eprintln!("not found :(");
        VecDeque::new()
    }
}


/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let r = parse_input!(inputs[0], i32); // number of rows.
    let c = parse_input!(inputs[1], i32); // number of columns.
    let a = parse_input!(inputs[2], i32); // number of rounds between the time the alarm countdown is activated and the time the alarm goes off.
    
    let mut map: Map = Map::new(r, c);
    let mut kirk: Kirk = Kirk::new(a);
    // game loop
    loop {
        // update
        kirk.update();
        map.update();
        // dbg print
        map.eprint();
        kirk.make_move(&mut map);
    }
}
