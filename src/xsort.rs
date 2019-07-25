use ::rayon::prelude::*;
use ::rayon::slice::ParallelSliceMut;
use ::std::sync::Mutex;

use crate::util::Point;
use crate::util::Minimum;


#[allow(dead_code)]
fn xsort_ser(points: &mut [Point]) -> (Point, Point) {
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
pub fn xsort_par(points: &mut [Point]) -> (Point, Point) {

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
