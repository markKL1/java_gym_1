use ::std::fs;

type CoordType = f64;

#[derive(Debug)]
struct Point {
    x: CoordType,
    y: CoordType,
    z: CoordType,
}

fn read_data() -> Vec<Point> {
    let mut points = Vec::with_capacity(100_000);
    let csv = fs::read_to_string("test_data.csv").unwrap();
//    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for line in csv.lines() {
        let mut coords = line.split(";");
        println!("float: {:?}", coords.next().unwrap().parse::<CoordType>());
        let point = Point {
            x: coords.next().unwrap().parse::<CoordType>().unwrap(),
            y: coords.next().unwrap().parse::<CoordType>().unwrap(),
            z: coords.next().unwrap().parse::<CoordType>().unwrap(),
        };
        println!("points: {:?}", point);
        points.push(point);
    }
    points
}

fn main() {
    println!("Welcome to Rust gym 1");
    let points = read_data();
}

