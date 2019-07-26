use ::smallvec::SmallVec;
use ::std::fmt;

use crate::util::{CoordType, Minimum, Point};

#[derive(Debug)]
struct BoundingBox {
    min_x: CoordType,
    length_x: CoordType,
    min_y: CoordType,
    length_y: CoordType,
    min_z: CoordType,
    length_z: CoordType,
}

#[derive(Debug, Clone)]
struct PointCubeXAssignment {
    point: Point,
    x_cube_nr: usize,
}

//TODO @mverleg: which fields are used?
struct Cubes {
    bbox: BoundingBox,
    rib_length: CoordType,
    x_cnt: usize,
    y_cnt: usize,
    z_cnt: usize,
    yz_cnt: usize,
    total_cnt: usize,
    data: Vec<Vec<PointCubeXAssignment>>
}

impl Cubes {
    fn yz_to_index(&self, y: usize, z: usize) -> usize {
        debug_assert!(y < self.y_cnt);
        debug_assert!(z < self.z_cnt);
        y + self.y_cnt * z
    }

    fn xpos_to_xindex(&self, pos: CoordType) -> usize {
        ((pos - self.bbox.min_x) / self.rib_length).floor() as usize
    }

    fn add_to_xrow(&mut self, y: usize, z: usize, point: Point) {
        let yz_index = yz_to_index(y, z);
        let x_cube_nr = 0;
        let point_assignment = PointCubeXAssignment {
            point,
            x_cube_nr,
        };
        data[index].push(point_assignment);
    }

    fn sort(&mut self) {

    }
}

impl fmt::Debug for Cubes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Cubes[")?;
        f.write_str(&format!("{}", self.x_cnt));
        f.write_str(" x ");
        f.write_str(&format!("{}", self.y_cnt));
        f.write_str(" x ");
        f.write_str(&format!("{}", self.z_cnt));
        f.write_str("]")
    }
}

impl BoundingBox {
    fn volume(&self) -> CoordType {
        (self.max_x - self.min_x) *
        (self.max_y - self.min_y) *
        (self.max_z - self.min_z)
    }

    fn calc_cubes(self, rib_len: CoordType, point_cnt: usize) -> Cubes {
        let x_cnt = (self.length_x / rib_len).ceil() as usize;
        let y_cnt = (self.length_y / rib_len).ceil() as usize;
        let z_cnt = (self.length_z / rib_len).ceil() as usize;
        let yz_cnt = y_cnt * z_cnt;
        let total_cnt = x_cnt * yz_cnt;
        //TODO @mverleg: tune capacity
        let yz_bin_expected_cnt = (1 + point_cnt / yz_cnt);
        let data = vec![Vec::with_capacity(yz_bin_expected_cnt); yz_cnt];
        Cubes {
            bbox: self,
            rib_length: rib_len,
            x_cnt,
            y_cnt,
            z_cnt,
            yz_cnt,
            total_cnt,
            data,
        }
    }
}

fn find_extrema(points: &[Point]) -> BoundingBox {
    let mut min_x: CoordType = points[0].x;
    let mut max_x: CoordType = points[0].x;
    let mut min_y: CoordType = points[0].y;
    let mut max_y: CoordType = points[0].y;
    let mut min_z: CoordType = points[0].z;
    let mut max_z: CoordType = points[0].z;
    for point in points {
        if point.x < bbox.min_x {
            min_x = point.x;
        }
        if point.x > bbox.max_x {
            max_x = point.x;
        }
        if point.y < bbox.min_y {
            min_y = point.y;
        }
        if point.y > bbox.max_y {
            max_y = point.y;
        }
        if point.z < bbox.min_z {
            min_z = point.z;
        }
        if point.z > bbox.max_z {
            max_z = point.z;
        }
    }
    BoundingBox {
        min_x,
        length_x: max_x - min_x,
        min_y,
        length_y: max_y - min_y,
        min_z,
        length_z: max_z - min_z,
    }
}

// Minimum cube rib length to still find the nearest pair even if totally homogeneous
fn min_cube_size(bounding_box: &BoundingBox, point_cnt: usize) -> CoordType {
    (bounding_box.volume() / point_cnt as f64).cbrt()
}

fn assign_points_to_cubes(points: &[Point], bbox: &BoundingBox, grid: &mut Cubes) {
    for point in points {

    }
}

#[allow(dead_code)]
pub fn boxing_ser(points: &mut [Point]) -> (Point, Point) {
    let bbox = find_extrema(points);
    let min_len = min_cube_size(&bbox, points.len());
    //TODO @mverleg: tune box_size
    let box_size = 3 * min_len;
    let cubes = bbox.calc_cubes(box_size, points.len());
    println!("cubes: {:?}", cubes);



//    let mut minimum = Minimum::new(
//        points[0].clone(),
//        points[1].clone(),
//    );
//    for i in 0..points.len() {
//        let p1 = points[i];
//        for j in (i + 1)..points.len() {
//            let p2 = points[j];
//            if p1.dist2(&p2) < minimum.dist2 {
//                minimum = Minimum::new(
//                    p1.clone(),
//                    p2.clone(),
//                );
//            }
//        }
//    }
//    (minimum.point1, minimum.point2)
    panic!();
}
