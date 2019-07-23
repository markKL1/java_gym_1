use ::std::fs;
use ::std::time::Instant;

type CoordType = f64;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: CoordType,
    y: CoordType,
    z: CoordType,
}

impl Point {
    fn dist2(&self, other: &Point) -> CoordType {
        (self.x - other.x).powf(2.) +
            (self.y - other.y).powf(2.) +
            (self.z - other.z).powf(2.)
    }
}

struct Minimum {
    point1: Point,
    point2: Point,
    dist1: CoordType,
    dist2: CoordType,
}

impl Minimum {
    fn new(point1: Point, point2: Point) -> Self {
        let dist2 = point1.dist2(&point2);
        Minimum {
            point1,
            point2,
            dist1: dist2.sqrt(),
            dist2,
        }
    }
}

fn read_data() -> Vec<Point> {
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

fn solve(points: &mut [Point]) -> (Point, Point) {
    points.sort_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap());
    let mut minimum = Minimum::new(
        points[0].clone(),
        points[1].clone(),
    );
    for i in 0..points.len() {
        let p1 = points[i];
        for j in (i + 1)..points.len() {
            let p2 = points[j];
            if p2.x - p1.x > minimum.dist1 {
                break;
            }
            if p1.dist2(&p2) < minimum.dist2 {
                minimum = Minimum::new(
                    p1.clone(),
                    p2.clone(),
                );
            }
        }
    }
    (minimum.point1, minimum.point2)
}

fn main() {
    println!("Welcome to Rust gym 1");
    let original_points = read_data();
    let mut points = original_points.clone();
    let now = Instant::now();
    let (p1, p2) = solve(&mut points);
    let duration = now.elapsed().as_millis();
    let expected_dist2 = original_points[55419].dist2(&original_points[152023]);
    println!("found distance {0:.3}", expected_dist2.sqrt());
    let actual_dist2 = p1.dist2(&p2);
    assert!(actual_dist2 < expected_dist2 * 1.001, "not the shortest distance");
    println!("took {0:.3} ms for {1:} points", duration, points.len())
}

