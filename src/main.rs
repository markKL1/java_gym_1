use ::std::time::Instant;

use crate::naive::solve_naive;
use crate::util::read_data;
use crate::xsort::xsort_par;

mod util;
mod naive;
mod xsort;
mod boxing;


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
        let (p1, p2) = xsort_par(&mut points);
        let duration = now.elapsed().as_nanos() as f64 / 1_000_000.0;
        total_ms += duration;
        let actual_dist1 = p1.dist2(&p2).sqrt();
        println!("rep {0:}: distance {1:.3} in {2:.3} ms",
                 rep, actual_dist1, duration);
        assert!(actual_dist1 < expected_dist2 * 1.000001, "not the shortest distance");
    }
    println!("took {0:.3} ms for {1:} points", total_ms / (REPS as f64), read_data().len())
}

