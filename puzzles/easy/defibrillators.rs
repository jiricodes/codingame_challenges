use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug)]
struct Defib {
    name: String,
    lon: f64,
    lat: f64,
}

impl Defib {
    fn new() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut name = String::new();
        let mut lon: f64 = 0.0;
        let mut lat: f64 = 0.0;
        for (i, word) in input_line.trim_matches('\n').split(";").enumerate() {
            if i == 1 {
                name = word.to_string();
            } else if i == 4 {
                lon = parse_input!(word.to_string().replace(",", "."), f64).to_radians();
            } else if i == 5 {
                lat = parse_input!(word.to_string().replace(",", "."), f64).to_radians();
            }
        }
        Self {
            name,
            lon,
            lat,
        }
    }

    fn dist(&self, lon: f64, lat: f64) -> f64 {
        let x = (lon - self.lon) * ((lat + self.lat) / 2.0).cos();
        let y = lat - self.lat;
        (x * x + y * y).sqrt() * 6371.0
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

}
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lon = parse_input!(input_line.trim().to_string().replace(",", "."), f64).to_radians();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lat = parse_input!(input_line.trim().to_string().replace(",", "."), f64).to_radians();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut nearest_name = String::from("shit");
    let mut nearest_dist = std::f64::MAX;
    for i in 0..n as usize {
        let defib = Defib::new();
        let d = defib.dist(lon, lat);
        if d < nearest_dist {
            nearest_dist = d;
            nearest_name = defib.get_name();
        }
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", nearest_name);
}
