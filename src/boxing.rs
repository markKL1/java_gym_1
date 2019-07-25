use crate::util::{Point, Minimum};

#[allow(dead_code)]
pub fn boxing_ser(points: &mut [Point]) -> (Point, Point) {
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
