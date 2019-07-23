use ::rayon::prelude::*;
use ::rayon::slice::ParallelSliceMut;
use ::std::fs;
use ::std::sync::Mutex;
use ::std::time::Instant;

type CoordType = f64;

//TODO @mverleg: try with and without copy
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

#[derive(Debug, Clone)]
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

#[allow(dead_code)]
fn solve_naive(points: &[Point]) -> (Point, Point) {
    let mut minimum = Minimum::new(
        points[0].clone(),
        points[1].clone(),
    );
    for i in 0..points.len() {
        let p1 = points[i];
        for j in (i + 1)..points.len() {
            let p2 = points[j];
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn solve_par(points: &mut [Point]) -> (Point, Point) {

    let initial_search_preference = 1;
    let batch_size = 32;

    // Sort by X-coordinate
    points.par_sort_unstable_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap());
    let length = points.len();

    // Find how much to do serially
    let mut initial_minimum = Minimum::new(
        points[0].clone(),
        points[1].clone(),
    );
    let mut initial_search = length % batch_size;
    while initial_search < initial_search_preference {
        initial_search += batch_size;
    }
    if initial_search > length {
        initial_search = length;
    }
    let batch_count = (length - initial_search) / batch_size;

    // Do some serial searching at the end, to set the initial minimum lower
    let initial_search = if length < initial_search_preference { length } else { initial_search_preference };
    for i in (length - initial_search)..length {
        let p1 = points[i];
        for j in (i + 1)..length {
            let p2 = points[j];
            if p2.x - p1.x > initial_minimum.dist1 {
                break;
            }
            if p1.dist2(&p2) < initial_minimum.dist2 {
                initial_minimum = Minimum::new(
                    p1.clone(),
                    p2.clone(),
                );
            }
        }
    }
    let global_minimum = Mutex::new(initial_minimum);

    // Use parallel search for the rest of it
    (0..batch_count).into_par_iter()
        .for_each(|batch_nr| {
            let mut local_minimum: Minimum = {
                (global_minimum.lock().unwrap()).clone()
            };
            let offset = batch_nr * batch_size;
            for i in offset..(offset + batch_size) {
                let p1 = points[i];
                for j in (i + 1)..length {
                    let p2 = points[j];
                    if p2.x - p1.x > local_minimum.dist1 {
                        break;
                    }
                    if p1.dist2(&p2) < local_minimum.dist2 {
//                    println!("Potentially updating minimum to {}", p1.dist2(&p2).sqrt());
                        let mut global_minimum_ref = global_minimum.lock().unwrap();
                        if p1.dist2(&p2) < global_minimum_ref.dist2 {
//                        println!("  Definitely updating");
                            *global_minimum_ref = Minimum::new(
                                p1.clone(),
                                p2.clone(),
                            );
                        }
                        local_minimum = global_minimum_ref.clone();
                    }
                }
            }
        });

    // Step 3: profit
    {
        let global_minimum_ref = global_minimum.lock().unwrap();
        (global_minimum_ref.point1, global_minimum_ref.point2)
    }
}

fn main() {
    println!("Welcome to Rust gym 1");
    const REPS: u128 = 20;
    let mut total_ms = 0.0;
    let mut now = Instant::now();
    let calculate_reference = false;
    let expected_dist2 = if calculate_reference {
        println!("solving brute-force to find reference distance");
        let points = read_data();
        let (refp1, refp2) = solve_naive(&points);
        let expected_dist1 = refp1.dist2(&refp2).sqrt();
        // expected minimum distance 430.809 found in 40833 ms
        println!("expected minimum distance {0:.3} found in {1:} ms",
                 expected_dist1.sqrt(), now.elapsed().as_nanos() as f64 / 1_000_000.0);
        expected_dist1
    } else {
        430.80863789034777
    };
    for rep in 0..REPS {
        let mut points = read_data();
        now = Instant::now();
        let (p1, p2) = solve_par(&mut points);
        let duration = now.elapsed().as_nanos() as f64 / 1_000_000.0;
        total_ms += duration;
        let actual_dist1 = p1.dist2(&p2).sqrt();
        println!("rep {0:}: distance {1:.3} in {2:.3} ms",
                 rep, actual_dist1, duration);
        assert!(actual_dist1 < expected_dist2 * 1.000001, "not the shortest distance");
    }
    println!("took {0:.3} ms for {1:} points", total_ms / (REPS as f64), read_data().len())
}

