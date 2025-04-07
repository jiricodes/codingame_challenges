use std::fmt;
use std::fmt::Display;
use std::io;
use std::ops::Add;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone, Copy)]
struct StateHash {
    hash: i32,
}

impl From<State> for StateHash {
    fn from(value: State) -> Self {
        let mut hash: i32 = 0;
        for i in 0..9 {
            hash = hash * 10 + value.tiles[i] as i32;
        }
        Self { hash }
    }
}

impl From<&State> for StateHash {
    fn from(value: &State) -> Self {
        let mut hash: i32 = 0;
        for i in 0..9 {
            hash = hash * 10 + value.tiles[i] as i32;
        }
        Self { hash }
    }
}

impl Default for StateHash {
    fn default() -> Self {
        Self { hash: 0 }
    }
}

const HASHMOD: i32 = 1 << 30;
const BITWISE_HASHMOD: i32 = 0b111111111111111111111111111111;

impl Add<StateHash> for StateHash {
    type Output = Self;
    fn add(self, rhs: StateHash) -> Self::Output {
        Self {
            hash: (self.hash + rhs.hash) & BITWISE_HASHMOD,
        }
    }
}

impl Display for StateHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}

struct Tile {
    val: u8,
}

#[derive(Debug, Clone, Copy)]
struct State {
    tiles: [u8; 9],
}

impl State {
    const NGBS_0: [u8; 2] = [1, 3];
    const NGBS_1: [u8; 3] = [0, 2, 4];
    const NGBS_2: [u8; 2] = [1, 5];
    const NGBS_3: [u8; 3] = [0, 4, 6];
    const NGBS_4: [u8; 4] = [1, 3, 5, 7];
    const NGBS_5: [u8; 3] = [2, 4, 8];
    const NGBS_6: [u8; 2] = [3, 7];
    const NGBS_7: [u8; 3] = [4, 6, 8];
    const NGBS_8: [u8; 2] = [5, 7];
    const NGBS_0_PERMS: [&[usize]; 1] = [&[1, 3]];
    const NGBS_1_PERMS: [&[usize]; 4] = [&[0, 2], &[0, 4], &[2, 4], &[0, 2, 4]];
    const NGBS_2_PERMS: [&[usize]; 1] = [&[1, 5]];
    const NGBS_3_PERMS: [&[usize]; 4] = [&[0, 4], &[0, 6], &[4, 6], &[0, 4, 6]];
    const NGBS_4_PERMS: [&[usize]; 11] = [
        &[1, 3],
        &[1, 5],
        &[1, 7],
        &[3, 5],
        &[3, 7],
        &[5, 7],
        &[1, 3, 5],
        &[1, 3, 7],
        &[1, 5, 7],
        &[3, 5, 7],
        &[1, 3, 5, 7],
    ];
    const NGBS_5_PERMS: [&[usize]; 4] = [&[2, 4], &[2, 8], &[4, 8], &[2, 4, 8]];
    const NGBS_6_PERMS: [&[usize]; 1] = [&[3, 7]];
    const NGBS_7_PERMS: [&[usize]; 4] = [&[4, 6], &[4, 8], &[6, 8], &[4, 6, 8]];
    const NGBS_8_PERMS: [&[usize]; 1] = [&[5, 7]];
    const HARDCODE_NGBS: [&'static [&'static [usize]]; 9] = [
        &Self::NGBS_0_PERMS,
        &Self::NGBS_1_PERMS,
        &Self::NGBS_2_PERMS,
        &Self::NGBS_3_PERMS,
        &Self::NGBS_4_PERMS,
        &Self::NGBS_5_PERMS,
        &Self::NGBS_6_PERMS,
        &Self::NGBS_7_PERMS,
        &Self::NGBS_8_PERMS,
    ];
    fn from_io() -> Self {
        let mut tiles: [u8; 9] = [0; 9];
        for i in 0..3 as usize {
            let mut inputs = String::new();
            io::stdin().read_line(&mut inputs).unwrap();
            for (j, txt) in inputs.split_whitespace().enumerate() {
                let value = parse_input!(txt, u8);
                tiles[i * 3 + j] = value;
            }
        }
        Self { tiles }
    }

    fn try_capture(&self, placement: usize, ngbs: &[usize]) -> Option<Self> {
        let mut cnt = 0;
        let mut ttl = 0;
        for ngb in ngbs {
            if self.tiles[*ngb] == 0 {
                return None;
            }
            cnt += 1;
            ttl += self.tiles[*ngb];
        }
        // let (cnt, ttl) = ngbs.iter().fold((0, 0), |acc, n| {
        //     (acc.0 + (self.tiles[*n] != 0) as u8, acc.1 + self.tiles[*n])
        // });
        if cnt > 1 && ttl <= 6 {
            let mut new = *self;
            new.tiles[placement] = ttl;
            for n in ngbs {
                new.tiles[*n] = 0;
            }
            Some(new)
        } else {
            None
        }
    }

    fn next_states(&self, placement: usize, results: &mut Vec<Self>) {
        results.clear();
        for ngbs in Self::HARDCODE_NGBS[placement] {
            if let Some(new) = self.try_capture(placement, ngbs) {
                results.push(new);
            }
        }
        if results.len() == 0 {
            let mut new = *self;
            new.tiles[placement] = 1;
            results.push(new);
        }
    }

    fn solve2(&self, depth: i32) -> StateHash {
        if depth == 0 {
            return StateHash::from(self);
        }
        StateHash::default()
    }

    fn solve(&self, depth: i32) -> i32 {
        let mut current: Vec<State> = vec![self.clone()];
        let mut res_count = 0;
        let mut result = StateHash::default();
        let mut next_states: Vec<Self> = Vec::with_capacity(12);
        for d in 0..depth {
            // if we don't have a state in the queue, we ran out of the options
            if current.is_empty() {
                eprintln!("stopped at d={}", d);
                break;
            }
            //dbg!(&current);
            // create a new queue, that will serve as next iteration with increased depth
            // we can perhaps cull here?
            // iterate over all states at this depth
            let mut new: Vec<State> = Vec::with_capacity(1000000);
            for state in current.iter() {
                // eprintln!("-----------{}----------", d);
                // state.eprint();
                // eprintln!(">>>>");
                let mut is_finished = true;
                // for each state check all possible placements
                for i in 0..9 {
                    if state.tiles[i] != 0 {
                        continue;
                    }
                    is_finished = false;
                    state.next_states(i, &mut next_states);
                    // dbg!(next_states.len());
                    // dbg!(&next_states);
                    // for state in next_states.iter() {
                    //     state.eprint();
                    //     eprintln!();
                    // }
                    for next_state in next_states.iter() {
                        new.push(*next_state);
                    }
                    // for each capture, queue up next possible state
                    // or place one
                }
                if is_finished {
                    // results.push(StateHash::from(state));
                    result = result + StateHash::from(state);
                    res_count += 1;
                }
            }
            current = new;
        }
        // append the state att current depth, but not finished games
        for state in current {
            // results.push(StateHash::from(state));
            result = result + StateHash::from(state);
            res_count += 1;
            res_count += 1;
        }
        // iter over results and sum hashes
        dbg!(res_count);
        // let ret = results.iter().fold(StateHash::default(), |acc, x| acc + *x);
        // ret.hash
        result.hash
    }

    //change toggle
    //for each tile
    //if not empty then
    //continue
    //else
    //check for neighbors and capture
    //else add 1
    //que the new state
    //toggle = true
    //continue
    //if toggle false, record state (hash)

    fn eprint(&self) {
        eprintln!("{} | {} | {}", self.tiles[0], self.tiles[1], self.tiles[2]);
        eprintln!("{} | {} | {}", self.tiles[3], self.tiles[4], self.tiles[5]);
        eprintln!("{} | {} | {}", self.tiles[6], self.tiles[7], self.tiles[8]);
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} | {} | {} | {} | {} | {} | {} | {} | {}",
            self.tiles[0],
            self.tiles[1],
            self.tiles[2],
            self.tiles[3],
            self.tiles[4],
            self.tiles[5],
            self.tiles[6],
            self.tiles[7],
            self.tiles[8],
        )
    }
}
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // let mut input_line = String::new();
    // io::stdin().read_line(&mut input_line).unwrap();
    // let depth = parse_input!(input_line, i32);
    // dbg!(depth);
    // let state = State::from_io();
    // dbg!(state.tiles);

    let depth = 24;
    let state = State {
        tiles: [3, 0, 0, 3, 6, 2, 1, 0, 2],
    };

    // state.eprint();
    // for i in 0..3 as usize {
    //     let mut inputs = String::new();
    //     io::stdin().read_line(&mut inputs).unwrap();
    //     for j in inputs.split_whitespace() {
    //         let value = parse_input!(j, i32);
    //     }
    // }

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", state.solve(depth));
}

#[cfg(test)]
mod tests {
    use std::i32;

    use super::*;
    #[test]
    fn hash() {
        let state = State {
            tiles: [1, 2, 3, 4, 5, 6, 0, 1, 2],
        };
        let hash = StateHash::from(state);
        assert_eq!(hash.hash, 123456012);
    }

    #[test]
    fn correct_mod() {
        assert_eq!(HASHMOD, 0b1000000000000000000000000000000);

        let n = 123456012;
        assert_eq!(n % HASHMOD, n & BITWISE_HASHMOD);
        let n = i32::MAX - 2;
        assert_eq!(n % HASHMOD, n & BITWISE_HASHMOD);
    }

    #[test]
    fn two_states() {
        let depth = 20;
        let state = State {
            tiles: [0, 6, 0, 2, 2, 2, 1, 6, 1],
        };
        let res = state.solve(depth);
        let expected = 322444322;
        assert_eq!(res, expected);
    }

    #[test]
    fn two_unique_states() {
        let depth = 1;
        let state = State {
            tiles: [5, 5, 5, 0, 0, 5, 5, 5, 5],
        };
        let res = state.solve(depth);
        let expected = 36379286;
        assert_eq!(res, expected);
    }

    #[test]
    fn unique_states_20() {
        let depth = 8;
        let state = State {
            tiles: [6, 0, 6, 0, 0, 0, 6, 1, 5],
        };
        let res = state.solve(depth);
        let expected = 76092874;
        assert_eq!(res, expected);
    }

    fn unique_states_241() {
        let depth = 24;
        let state = State {
            tiles: [3, 0, 0, 3, 6, 2, 1, 0, 2],
        };
        let res = state.solve(depth);
        let expected = 661168294; // 418440394 end states
        assert_eq!(res, expected);
    }
}
