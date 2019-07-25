use ::std::fs;
use std::time::Instant;


pub type CoordType = f64;

//TODO @mverleg: try with and without copy
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: CoordType,
    pub y: CoordType,
    pub z: CoordType,
}

impl Point {
    pub fn dist2(&self, other: &Point) -> CoordType {
        (self.x - other.x).powf(2.) +
            (self.y - other.y).powf(2.) +
            (self.z - other.z).powf(2.)
    }
}

#[derive(Debug, Clone)]
pub struct Minimum {
    pub point1: Point,
    pub point2: Point,
    pub dist1: CoordType,
    pub dist2: CoordType,
}

impl Minimum {
    pub fn new(point1: Point, point2: Point) -> Self {
        let dist2 = point1.dist2(&point2);
        Minimum {
            point1,
            point2,
            dist1: dist2.sqrt(),
            dist2,
        }
    }
}

pub type SolveFun = fn(&mut [Point]) -> (Point, Point);

pub fn read_data() -> Vec<Point> {
    let mut points = Vec::with_capacity(100_000);
    let csv = fs::read_to_string("test_data.csv").unwrap();
//    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for line in csv.lines() {
        if !line.contains(";") {
            break;
        }
        let mut coords = line.split(";");
        let point = Point {
            x: coords.next().unwrap().replace(",", ".").parse::<CoordType>().unwrap(),
            y: coords.next().unwrap().replace(",", ".").parse::<CoordType>().unwrap(),
            z: coords.next().unwrap().replace(",", ".").parse::<CoordType>().unwrap(),
        };
        points.push(point);
    }
    points
}

pub fn bench(name: &str, solve_fun: SolveFun, reps: usize) {
    let mut total_ms = 0.0;
    let mut now;
    let expected_dist2 = 430.80863789034777;
    for rep in 0..reps {
        let mut points = read_data();
        now = Instant::now();
        let (p1, p2) = solve_fun(&mut points);
        let duration = now.elapsed().as_nanos() as f64 / 1_000_000.0;
        total_ms += duration;
        let actual_dist1 = p1.dist2(&p2).sqrt();
        println!("{3:} rep {0:}: distance {1:.3} in {2:.3} ms",
                 rep, actual_dist1, duration, name);
        assert!(actual_dist1 < expected_dist2 * 1.000001, "not the shortest distance");
    }
    println!("{2:} took {0:.3} ms for {1:} points",
             total_ms / (reps as f64), read_data().len(), name)
}