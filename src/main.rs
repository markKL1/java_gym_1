#[allow(unused_imports)]
use crate::naive::solve_naive;
#[allow(unused_imports)]
use crate::util::{read_data, bench};
#[allow(unused_imports)]
use crate::xsort::xsort_par;
#[allow(unused_imports)]
use crate::boxing::boxing_ser;

mod util;
mod naive;
mod xsort;
mod boxing;


fn main() {
    println!("Welcome to Rust gym 1");
//    let calculate_reference = false;
//    let expected_dist2 = if calculate_reference {
//        println!("solving brute-force to find reference distance");
//        let points = read_data();
//        let (refp1, refp2) = solve_naive(&points);
//        refp1.dist2(&refp2)
//        // expected minimum distance 430.809 found in 40833 ms
////        println!("expected minimum distance {0:.3} found in {1:} ms",
////                 expected_dist1.sqrt(), now.elapsed().as_nanos() as f64 / 1_000_000.0);
//    } else {
//        430.80863789034777
//    };
//    bench("xsort_par", xsort_par, 20);
    bench("boxing_ser", boxing_ser, 20);
}
