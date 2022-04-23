use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::{Add, Mul};

// mod tests;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    const ZERO: Self = Self { x: 0.0, y: 0.0 };
    const MAX: Self = Self {
        x: 17630.0,
        y: 9000.0,
    };

    pub fn opposite_corner(&self) -> Self {
        if *self == Self::ZERO {
            Self::MAX
        } else {
            Self::ZERO
        }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let dx = (other.x - self.x) as f32;
        let dy = (other.y - self.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn magnitude(&self) -> f32 {
        self.distance(&Self::ZERO)
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self {
            x: self.x as f32 / m,
            y: self.y as f32 / m,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x as i32, self.y as i32)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Hero {
    id: i32,
    pos: Vec2,
    shield: i32,
    charmed: bool,
}

impl Hero {
    pub fn update(&mut self, id: i32, pos: Vec2, shield: i32, charmed: bool) {
        self.id = id;
        self.pos = pos;
        self.shield = shield;
        self.charmed = charmed;
    }

    pub fn move_to(&self, pos: &Vec2) {
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
            pos: Vec2::ZERO,
            shield: 0,
            charmed: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Monster {
    id: i32,
    pos: Vec2,
    shield: i32,
    charmed: bool,
    hp: i32,
    velocity: Vec2,
    target: Option<Vec2>,
    eta: i32,
}

impl Monster {
    const SPEED: f32 = 400.0;
    const AGRRO_RANGE: f32 = 5000.0;

    pub fn new(
        id: i32,
        pos: Vec2,
        shield: i32,
        charmed: bool,
        hp: i32,
        velocity: Vec2,
        target: Option<Vec2>,
    ) -> Self {
        let eta = if target.is_some() {
            let t = target.unwrap();
            let mut approach: f32 = 0.0;
            let mut p: Vec2 = pos.clone();
            let mut v: Vec2 = velocity.normalize();
            while (p + (v * approach)).distance(&t) > Self::AGRRO_RANGE {
                approach += 1.0;
            }
            p = p + (v * approach);
            // get new vector towards target
            // get eta to target == reach
            // return approach + reach
            0
        } else {
            -1
        };
        Self {
            id,
            pos,
            shield,
            charmed,
            hp,
            velocity,
            target,
            eta,
        }
    }

    pub fn closer(self, other: Self, pos: &Vec2) -> Self {
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
    base: Vec2,
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
            base: Vec2::ZERO,
        }
    }
}

struct Game {
    me: Player,
    enemy: Player,
    my_heroes: [Hero; 3],
    monsters: HashMap<i32, Monster>,
}

impl Game {
    pub fn new() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let base_x = parse_input!(inputs[0], f32); // The corner of the map representing your base
        let base_y = parse_input!(inputs[1], f32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let _heroes_per_player = parse_input!(input_line, i32); // Always 3
        let base = Vec2 {
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
            monsters: HashMap::new(),
        }
    }

    fn update_monster(
        &mut self,
        id: i32,
        pos: Vec2,
        shield: i32,
        charmed: bool,
        hp: i32,
        velocity: Vec2,
        target: Option<Vec2>,
    ) {
        // Lets decide later if the ld information is somewhat useful
        let _ = self.monsters.insert(
            id,
            Monster::new(id, pos, shield, charmed, hp, velocity, target),
        );
    }

    pub fn update(&mut self) {
        // We should somehow track monsters that we've seen, but are not visible now
        // for now lets clear the monsters before each update
        self.monsters.clear();

        // Players hp and mana
        self.me.update();
        self.enemy.update();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, usize); // Amount of heros and monsters you can see
        let mut my_hero_index: usize = 0;

        let mut closest_monster: Option<Monster> = None;

        for _ in 0..entity_count {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let id = parse_input!(inputs[0], i32); // Unique identifier
            let tp = parse_input!(inputs[1], i32); // 0=monster, 1=your hero, 2=opponent hero
            let x = parse_input!(inputs[2], f32); // Position of this entity
            let y = parse_input!(inputs[3], f32);
            let shield_life = parse_input!(inputs[4], i32); // Ignore for this league; Count down until shield spell fades
            let is_controlled = parse_input!(inputs[5], i32); // Ignore for this league; Equals 1 when this entity is under a control spell
            let health = parse_input!(inputs[6], i32); // Remaining health of this monster
            let vx = parse_input!(inputs[7], f32); // Trajectory of this monster
            let vy = parse_input!(inputs[8], f32);
            let near_base = parse_input!(inputs[9], i32); // 0=monster with no target yet, 1=monster targeting a base
            let threat_for = parse_input!(inputs[10], i32); // Given this monster's trajectory, is it a threat to 1=your base, 2=your opponent's base, 0=neither

            if tp == 1 {
                self.my_heroes[my_hero_index].update(
                    id,
                    Vec2 { x: x, y: y },
                    shield_life,
                    is_controlled == 1,
                );
                my_hero_index += 1;
            }

            if tp == 0 {
                let target = match threat_for {
                    1 => Some(self.me.base),
                    2 => Some(self.enemy.base),
                    _ => None,
                };
                self.update_monster(
                    id,
                    Vec2 { x: x, y: y },
                    shield_life,
                    is_controlled == 1,
                    health,
                    Vec2 { x: vx, y: vy },
                    target,
                );

                let new_monster = Monster::new(
                    id,
                    Vec2 { x: x, y: y },
                    shield_life,
                    is_controlled == 1,
                    health,
                    Vec2 { x: vx, y: vy },
                    target,
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

    pub fn move_all_to(&self, pos: &Vec2) {
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
        // check for critical targets - the ones heading to base
        // check for nearby targets
        // go patrol
    }
}
