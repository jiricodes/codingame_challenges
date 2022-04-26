use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;
use std::io;
use std::ops::{Add, Mul, Sub};
use std::time::Instant;

#[cfg(test)]
mod tests;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const MAX: Self = Self {
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

    pub fn perp(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn in_bounds(&self, bounds: &Vec2) -> bool {
        self.x >= 0.0 && self.x <= bounds.x && self.y >= 0.0 && self.y <= bounds.y
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x as i32, self.y as i32)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
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

impl Eq for Vec2 {}

#[derive(Debug, Clone)]
pub struct Patrol {
    center: Vec2,
    start_angle: f32,
    end_angle: f32,
    radius: f32,
    n_segments: f32,
    step: f32,
    points: Vec<Vec2>,
    i: i32,
    d: i32,
}

impl Patrol {
    pub fn new(center: Vec2, radius: f32, n: f32) -> Self {
        let (start_angle, end_angle) = if center == Vec2::ZERO {
            ((0.0_f32).to_radians(), (90.0_f32).to_radians())
        } else {
            ((-180.0_f32).to_radians(), (-90.0_f32).to_radians())
        };
        let step = (end_angle - start_angle) / n;
        let mut new = Self {
            center,
            start_angle,
            end_angle,
            radius,
            n_segments: n,
            step,
            points: Vec::new(),
            i: 0,
            d: 1,
        };
        new.calculate_points();
        new
    }

    fn calculate_points(&mut self) {
        self.points.clear();
        for i in 0..(self.n_segments as usize) + 1 {
            let x = self.center.x + self.radius * (self.start_angle + self.step * i as f32).cos();
            let y = self.center.y + self.radius * (self.start_angle + self.step * i as f32).sin();
            self.points.push(Vec2 { x: x, y: y });
        }
    }

    pub fn get(&self) -> Vec2 {
        self.points[self.i as usize].clone()
    }

    pub fn get_next(&mut self) -> Vec2 {
        let t = self.points[self.i as usize].clone();
        let l = (self.points.len() - 1) as i32;
        if self.i == l {
            self.d = -1;
        } else if self.i == 0 {
            self.d = 1;
        }
        self.i = self.i + self.d;
        t
    }

    pub fn get_next_offensively(&mut self) -> Vec2 {
        let l = (self.points.len() - 1) as i32;
        self.i = (self.i) % l;
        self.get_next()
    }
}

#[derive(Debug, Clone)]
struct Hero {
    id: i32,
    pos: Vec2,
    shield: i32,
    charmed: bool,
    patrol: Patrol,
}

impl Hero {
    const VIEW_RANGE: f32 = 2200.0;
    const SPEED: f32 = 800.0;
    const DMG: i32 = 2;
    const ATTACK_RANGE: f32 = 800.0;
    const SHIELD_RANGE: f32 = 2200.0;
    const WIND_RANGE: f32 = 1280.0;
    const CONTROL_RANGE: f32 = 2200.0;

    pub fn new(base: &Vec2, attack: bool) -> Self {
        if attack {
            Self {
                patrol: Patrol::new(base.opposite_corner(), 7000.0, 12.0),
                ..Default::default()
            }
        } else {
            Self {
                patrol: Patrol::new(*base, 8200.0, 10.0),
                ..Default::default()
            }
        }
    }

    pub fn update(&mut self, id: i32, pos: Vec2, shield: i32, charmed: bool) {
        self.id = id;
        self.pos = pos;
        self.shield = shield;
        self.charmed = charmed;
    }

    pub fn move_to(&self, pos: &Vec2, yell: Option<String>) {
        println!(
            "MOVE {} {}:{}",
            pos,
            self.id,
            yell.unwrap_or("n/a".to_string())
        );
    }

    pub fn wait(&self, yell: Option<String>) {
        println!("WAIT {}:{}", self.id, yell.unwrap_or("n/a".to_string()));
    }

    pub fn wind(&self, t: &Vec2, yell: Option<String>) {
        println!(
            "SPELL WIND {} {}:{}",
            t,
            self.id,
            yell.unwrap_or("n/a".to_string())
        );
    }

    pub fn shield(&self, tid: i32, yell: Option<String>) {
        println!(
            "SPELL SHIELD {} {}:{}",
            tid,
            self.id,
            yell.unwrap_or("n/a".to_string())
        );
    }

    pub fn control(&self, tid: i32, t: &Vec2, yell: Option<String>) {
        println!(
            "SPELL CONTROL {} {} {}:{}",
            tid,
            t,
            self.id,
            yell.unwrap_or("n/a".to_string())
        );
    }

    pub fn patrol(&mut self, monsters: &mut Vec<Monster>) {
        if !monsters.is_empty() {
            monsters.sort_by(|a, b| {
                let (_, ai) = self.find_intercept(a);
                let (_, bi) = self.find_intercept(b);
                ai.partial_cmp(&bi).unwrap()
            });
            let mut pruned = monsters
                .iter()
                .filter(|x| x.pos.distance(&self.patrol.center) < 9000.0);
            if let Some(m) = pruned.next() {
                let (t, _) = self.find_intercept(m);
                self.move_to(&t, Some(format!("N{}", m.id)));
                return;
            }
        }
        let mut t = self.patrol.get();
        while self.pos.distance(&t) < Self::VIEW_RANGE {
            t = self.patrol.get_next();
        }
        self.move_to(&t, Some(format!("patrol")));
    }

    pub fn time_to_kill(&self, monster: &Monster) -> (Vec2, i32) {
        let mut ttk = 0;
        let (t, i) = self.find_intercept(monster);
        ttk += i;
        ttk += if monster.hp % Self::DMG == 0 { 0 } else { 1 };
        ttk += monster.hp / Self::DMG;
        (t, ttk)
    }

    /// Attempt to find an ideal target for intercepting the monster
    pub fn find_intercept(&self, monster: &Monster) -> (Vec2, i32) {
        let mut m = monster.clone();
        let mut i: f32 = 0.0;
        while self.pos.distance(&m.pos) > Self::SPEED * i + Self::ATTACK_RANGE {
            i += 1.0;
            m.simulate_move();
        }
        m.simulate_move();
        (m.pos, i as i32)
    }

    pub fn defend(
        &mut self,
        monsters_me: &mut BinaryHeap<Monster>,
        monsters_none: &mut Vec<Monster>,
        mana: u32,
    ) {
        let m_opt = monsters_me.peek();
        if m_opt.is_some() {
            let m = m_opt.unwrap();
            let mid = m.id;
            let (t, ttk) = self.time_to_kill(m);
            if ttk < m.eta {
                let _ = monsters_me.pop();
            } else if mana > 10 && self.pos.distance(&m.pos) < Self::WIND_RANGE {
                self.wind(
                    &self.patrol.center.opposite_corner(),
                    Some(format!("diff {}", m.eta - ttk)),
                );
                return;
            }
            self.move_to(&t, Some(format!("M{}", mid)));
            return;
        }
        self.patrol(monsters_none);
    }

    pub fn attack(
        &mut self,
        monsters_enemy: &mut Vec<Monster>,
        monsters_none: &mut Vec<Monster>,
        mana: u32,
    ) {
        if mana > 120 {
            // attempt to wind
            for m in monsters_enemy.iter() {
                if m.shield == 0 && m.pos.distance(&self.pos) < Self::WIND_RANGE {
                    self.wind(&self.patrol.center, Some("E".to_string()));
                    return;
                }
            }
            for m in monsters_none.iter() {
                if m.shield == 0 && m.pos.distance(&self.pos) < Self::WIND_RANGE {
                    self.wind(&self.patrol.center, Some("N".to_string()));
                    return;
                }
            }
            // attempt to shield
            for m in monsters_enemy.iter() {
                if m.shield == 0 && m.eta < 15 && m.pos.distance(&self.pos) < Self::SHIELD_RANGE {
                    self.shield(m.id, Some("E".to_string()));
                    return;
                }
            }
        }
        self.patrol(monsters_none);
    }
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            id: 0,
            pos: Vec2::ZERO,
            shield: 0,
            charmed: false,
            patrol: Patrol::new(Vec2::ZERO, 8200.0, 12.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Monster {
    id: i32,
    pos: Vec2,
    shield: i32,
    charmed: bool,
    hp: i32,
    velocity: Vec2,
    target: Option<Vec2>,
    reaching: bool,
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
        reaching: bool,
    ) -> Self {
        let mut new = Self {
            id,
            pos,
            shield,
            charmed,
            hp,
            velocity,
            target,
            reaching,
            eta: i32::MAX,
        };
        new.eta();
        new
    }

    // pub fn simulate()

    fn eta(&mut self) {
        self.eta = if self.target.is_some() {
            let t = self.target.unwrap();
            let mut approach: f32 = 0.0;
            let mut p: Vec2 = self.pos.clone();
            let mut v: Vec2 = self.velocity.normalize() * Self::SPEED;
            // find round at which the monster is < 5000
            while (p + (v * approach)).distance(&t) > Self::AGRRO_RANGE {
                approach += 1.0;
            }
            p = p + (v * approach);
            // get new vector towards target
            v = t - p;
            // get eta to target == reach
            let reach = (v.magnitude() - 300.0).max(0.0) / Self::SPEED;
            // return approach + reach
            if reach == 0.0 {
                0
            } else {
                (reach + approach).trunc() as i32 + 1
            }
        } else {
            i32::MAX
        };
    }

    fn simulate_move(&mut self) {
        if self.target.is_some() {
            let t = self.target.unwrap();
            if self.pos.distance(&t) <= Self::AGRRO_RANGE {
                self.velocity = (t - self.pos).normalize() * Self::SPEED;
            }
        }
        self.pos = self.pos + self.velocity;
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

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Monster {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .eta
            .cmp(&self.eta)
            .then_with(|| self.hp.cmp(&other.hp))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Monster {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    monsters_me: BinaryHeap<Monster>,
    monsters_enemy: Vec<Monster>,
    monsters_none: Vec<Monster>,
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
            my_heroes: [
                Hero::new(&base, false),
                Hero::new(&base, false),
                Hero::new(&base, true),
            ],
            monsters_me: BinaryHeap::new(),
            monsters_enemy: Vec::new(),
            monsters_none: Vec::new(),
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
        threat_for: i32,
        reaching: bool,
    ) {
        match threat_for {
            1 => self.monsters_me.push(Monster::new(
                id,
                pos,
                shield,
                charmed,
                hp,
                velocity,
                Some(self.me.base),
                reaching,
            )),
            2 => self.monsters_enemy.push(Monster::new(
                id,
                pos,
                shield,
                charmed,
                hp,
                velocity,
                Some(self.enemy.base),
                reaching,
            )),
            _ => self.monsters_none.push(Monster::new(
                id, pos, shield, charmed, hp, velocity, None, false,
            )),
        }
    }

    pub fn update(&mut self) {
        // We should somehow track monsters that we've seen, but are not visible now
        // for now lets clear the monsters before each update
        self.monsters_me.clear();
        self.monsters_enemy.clear();
        self.monsters_none.clear();

        // Players hp and mana
        self.me.update();
        self.enemy.update();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, usize); // Amount of heros and monsters you can see

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
                let hid = match id {
                    0 | 3 => 0,
                    1 | 4 => 1,
                    2 | 5 => 2,
                    _ => {
                        panic!("hero id {}", id)
                    }
                };
                self.my_heroes[hid].update(
                    id,
                    Vec2 { x: x, y: y },
                    shield_life,
                    is_controlled == 1,
                );
            }

            if tp == 0 {
                self.update_monster(
                    id,
                    Vec2 { x: x, y: y },
                    shield_life,
                    is_controlled == 1,
                    health,
                    Vec2 { x: vx, y: vy },
                    threat_for,
                    near_base == 1,
                );
            }
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
        let now = Instant::now();
        game.update();
        eprintln!("{:.3}ms", now.elapsed().as_millis());
        eprintln!(
            "me {}, enemy {}, none {}",
            game.monsters_me.len(),
            game.monsters_enemy.len(),
            game.monsters_none.len()
        );
        // defender
        let now = Instant::now();
        game.my_heroes[0].defend(&mut game.monsters_me, &mut game.monsters_none, game.me.mana);
        // defender
        game.my_heroes[1].defend(&mut game.monsters_me, &mut game.monsters_none, game.me.mana);
        // attacker
        game.my_heroes[2].attack(
            &mut game.monsters_enemy,
            &mut game.monsters_none,
            game.me.mana,
        );
        let another = now.elapsed().as_millis();
        eprintln!("{:.3}ms", another);
        // check for critical targets - the ones heading to base
        // check for nearby targets
        // go patrol
    }
}
