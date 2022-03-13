use std::io;
use std::collections::VecDeque;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

enum Operation {
    Val,
    Add,
    Sub,
    Mult
}

impl Operation {
    fn do_op(&self, arg1: &Arg, arg2: &Arg) -> i32 {
        match *self {
            Self::Val => arg1.get_val().unwrap(),
            Self::Add => arg1.get_val().unwrap() + arg2.get_val().unwrap(),
            Self::Sub => arg1.get_val().unwrap() - arg2.get_val().unwrap(),
            Self::Mult => arg1.get_val().unwrap() * arg2.get_val().unwrap(),
        }
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Operation {
        match s {
            "VALUE" => Operation::Val,
            "ADD" => Operation::Add,
            "SUB" => Operation::Sub,
            "MULT" => Operation::Mult,
            _ => panic!("unknown operation"),
        }
    }
}

enum Arg {
    Val(i32),
    Refs(usize),
    Empty,
}

impl Arg {
    fn get_ref(&self) -> Option<usize> {
        match *self {
            Arg::Refs(i) => Some(i),
            _ => None,
        }
    }

    fn get_val(&self) -> Option<i32> {
        match *self {
            Arg::Val(i) => Some(i),
            _ => None,
        }
    }
}
impl From<&str> for Arg {
    fn from(s: &str) -> Self {
        match &s[0..1] {
            "$" => Self::Refs(parse_input!(&s[1..], usize)),
            "_" => Self::Empty,
            _ => Self::Val(parse_input!(s, i32))
        }
    }
}

struct Op {
    index: usize,
    operation: Operation,
    arg1: Arg,
    arg2: Arg,
}

impl Op {
    fn new(i: usize) -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let operation = inputs[0].trim();
        let arg1 = inputs[1].trim();
        let arg2 = inputs[2].trim();
        Self {
            index: i,
            operation: operation.into(),
            arg1: arg1.into(),
            arg2: arg2.into(),
        }
    }

    fn try_solve(&self) -> (usize, i32) {
        (self.index, self.operation.do_op(&self.arg1, &self.arg2))
    }

    fn get_refs(&self) -> (Option<usize>, Option<usize>) {
        (self.arg1.get_ref(), self.arg2.get_ref())
    }

    fn update_arg1(&mut self, arg: Arg) {
        self.arg1 = arg;
    }

    fn update_arg2(&mut self, arg: Arg) {
        self.arg2 = arg;
    }
}

struct Spreadsheet {
    size: usize,
    queue: VecDeque<Op>,
    data: Vec<Option<i32>>,
}

impl Spreadsheet {
    fn new() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let n = parse_input!(input_line, usize);
        Self {
            size: n,
            queue: VecDeque::with_capacity(n),
            data: vec![None; n],
        }
    }

    fn load(&mut self) {
        for i in 0..self.size {
            self.queue.push_back(Op::new(i));
        }
    }

    fn resolve_refs(&mut self, op: &mut Op) -> Result<(), ()> {
        let mut ret1 = Err(());
        let refs = op.get_refs();
        if let Some(i) = refs.0 {
            if let Some(val) = self.data[i] {
                op.update_arg1(Arg::Val(val));
                ret1 = Ok(());
            };
        } else {
            ret1 = Ok(());
        };

        let mut ret2 = Err(());
        if let Some(i) = refs.1 {
            if let Some(val) = self.data[i] {
                op.update_arg2(Arg::Val(val));
                ret2 = Ok(());
            };
        } else {
            ret2 = Ok(());
        };
        if ret1.is_ok() && ret2.is_ok() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn solve(&mut self) {
        while !self.queue.is_empty() {
            let mut current = self.queue.pop_front().unwrap();
            if self.resolve_refs(&mut current).is_ok() {
                let res = current.try_solve();
                self.data[res.0] = Some(res.1);
            } else {
                self.queue.push_back(current);
            }
            
        }
    }
    fn print(&self) {
        for v in self.data.iter() {
            println!("{}",v.unwrap())
        }
    }
}
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut spreadsheet = Spreadsheet::new();
    spreadsheet.load();
    spreadsheet.solve();
    spreadsheet.print();
}
