use ::std::fs;


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
