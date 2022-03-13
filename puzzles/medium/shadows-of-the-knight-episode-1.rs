use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}


fn new_mid(max_val: i32, min_val: i32) -> i32 {
    ((max_val - min_val) / 2) + min_val
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let mut x: i32 = parse_input!(inputs[0], i32);
    let mut y: i32 = parse_input!(inputs[1], i32);
    let mut minx: i32 = 0;
    let mut maxx: i32 = w - 1;
    let mut miny: i32 = 0;
    let mut maxy: i32 = h - 1;
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        eprintln!("DIR {} | N {}", bomb_dir, n);
        match bomb_dir.as_str() {
            "U" => {
                maxy = y - 1;
                y = new_mid(maxy, miny);
            },
            "UR" => {
                maxy = y - 1;
                y = new_mid(maxy, miny);
                minx = x + 1;
                x = new_mid(maxx, minx);
            },
            "R" => {
                minx = x + 1;
                x = new_mid(maxx, minx);
            },
            "DR" => {
                minx = x + 1;
                x = new_mid(maxx, minx);
                miny = y + 1;
                y = new_mid(maxy, miny);
            },
            "D" => {
                miny = y + 1;
                y = new_mid(maxy, miny);
            },
            "DL" => {
                miny = y + 1;
                y = new_mid(maxy, miny);
                maxx = x - 1;
                x = new_mid(maxx, minx);
            },
            "L" => {
                maxx = x - 1;
                x = new_mid(maxx, minx);
            },
            "UL" => {
                maxx = x - 1;
                x = new_mid(maxx, minx);
                maxy = y - 1;
                y = new_mid(maxy, miny);
            },
            _ => println!("Error direction!"),
        }

        // the location of the next window Batman should jump to.
        println!("{} {}", x, y);
        n -= 1;

    }
}
