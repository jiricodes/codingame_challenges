use std::fmt;
use std::io;
use std::ops::Add;

mod tests;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const ZERO: Self = Self { x: 0, y: 0 };
    const MAX: Self = Self { x: 17630, y: 9000 };

    pub fn opposite_corner(&self) -> Self {
        if *self == Self::ZERO {
            Self::MAX
        } else {
            Self::ZERO
        }
    }

    pub fn distance(&self, other: &Self) -> u32 {
        let dx = (other.x - self.x) as f32;
        let dy = (other.y - self.y) as f32;
        (dx * dx + dy * dy).sqrt() as u32
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Hero {
    id: i32,
    pos: Point,
    shield: i32,
    charmed: bool,
}

impl Hero {
    pub fn update(&mut self, id: i32, pos: Point, shield: i32, charmed: bool) {
        self.id = id;
        self.pos = pos;
        self.shield = shield;
        self.charmed = charmed;
    }

    pub fn move_to(&self, pos: &Point) {
        println!("MOVE {}", pos)
    }

    pub fn wait(&self) {
        println!("WAIT");
    }
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            id: 0,
            pos: Point::ZERO,
            shield: 0,
            charmed: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Monster {
    id: i32,
    pos: Point,
    shield: i32,
    charmed: bool,
    hp: i32,
    velocity: Point,
}

impl Monster {
    pub fn new(id: i32, pos: Point, shield: i32, charmed: bool, hp: i32, velocity: Point) -> Self {
        Self {
            id,
            pos,
            shield,
            charmed,
            hp,
            velocity,
        }
    }

    pub fn closer(self, other: Self, pos: &Point) -> Self {
        let d0 = self.pos.distance(pos);
        let d1 = other.pos.distance(pos);
        if d0 < d1 {
            self
        } else {
            other
        }
    }
}

struct Player {
    hp: u32,
    mana: u32,
    base: Point,
}

impl Player {
    pub fn update(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        self.hp = parse_input!(inputs[0], u32); // Your base health
        self.mana = parse_input!(inputs[1], u32); // Ignore in the first league; Spend ten mana to cast a spell
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hp: 3,
            mana: 0,
            base: Point::ZERO,
        }
    }
}

struct Game {
    me: Player,
    enemy: Player,
    my_heroes: [Hero; 3],
}

impl Game {
    pub fn new() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let base_x = parse_input!(inputs[0], i32); // The corner of the map representing your base
        let base_y = parse_input!(inputs[1], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let _heroes_per_player = parse_input!(input_line, i32); // Always 3
        let base = Point {
            x: base_x,
            y: base_y,
        };
        let enemy_base = base.opposite_corner();
        Self {
            me: Player {
                base,
                ..Default::default()
            },
            enemy: Player {
                base: enemy_base,
                ..Default::default()
            },
            my_heroes: [Hero::default(); 3],
        }
    }

    pub fn update(&mut self) {
        // Players hp and mana
        self.me.update();
        self.enemy.update();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, usize); // Amount of heros and monsters you can see
        let mut my_hero_index: usize = 0;

        let mut closest_monster: Option<Monster> = None;

        for i in 0..entity_count {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let id = parse_input!(inputs[0], i32); // Unique identifier
            let tp = parse_input!(inputs[1], i32); // 0=monster, 1=your hero, 2=opponent hero
            let x = parse_input!(inputs[2], i32); // Position of this entity
            let y = parse_input!(inputs[3], i32);
            let shield_life = parse_input!(inputs[4], i32); // Ignore for this league; Count down until shield spell fades
            let is_controlled = parse_input!(inputs[5], i32); // Ignore for this league; Equals 1 when this entity is under a control spell
            let health = parse_input!(inputs[6], i32); // Remaining health of this monster
            let vx = parse_input!(inputs[7], i32); // Trajectory of this monster
            let vy = parse_input!(inputs[8], i32);
            let near_base = parse_input!(inputs[9], i32); // 0=monster with no target yet, 1=monster targeting a base
            let threat_for = parse_input!(inputs[10], i32); // Given this monster's trajectory, is it a threat to 1=your base, 2=your opponent's base, 0=neither

            if tp == 1 {
                self.my_heroes[my_hero_index].update(
                    id,
                    Point { x: x, y: y },
                    shield_life,
                    is_controlled == 1,
                );
                my_hero_index += 1;
            }

            if tp == 0 && threat_for == 1 {
                let new_monster = Monster::new(
                    id,
                    Point { x: x, y: y },
                    shield_life,
                    is_controlled == 1,
                    health,
                    Point { x: vx, y: vy },
                );
                if closest_monster.is_none() {
                    closest_monster = Some(new_monster)
                } else {
                    let mut monster = closest_monster.unwrap();
                    monster = monster.closer(new_monster, &self.me.base);
                    closest_monster = Some(monster);
                }
            }
        }

        if closest_monster.is_some() {
            let monster = closest_monster.unwrap();
            let pos = monster.pos + monster.velocity;
            self.move_all_to(&pos);
        } else {
            self.wait_all();
        }
    }

    pub fn move_all_to(&self, pos: &Point) {
        for hero in self.my_heroes.iter() {
            hero.move_to(pos);
        }
    }

    pub fn wait_all(&self) {
        for hero in self.my_heroes.iter() {
            hero.wait();
        }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut game = Game::new();

    // game loop
    loop {
        game.update();
    }
}
