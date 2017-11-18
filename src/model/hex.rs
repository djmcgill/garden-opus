use graphics::math::*;

pub const ROW_COUNT: usize = 11;

pub const IX_TO_COORDS: &'static[(usize, usize)] = &[
                    ( 0, 0), ( 1, 0), ( 2, 0), ( 3, 0), ( 4, 0), ( 5, 0),
                ( 0, 1), ( 1, 1), ( 2, 1), ( 3, 1), ( 4, 1), ( 5, 1), ( 6, 1), 
            ( 0, 2), ( 1, 2), ( 2, 2), ( 3, 2), ( 4, 2), ( 5, 2), ( 6, 2), ( 7, 2),
        ( 0, 3), ( 1, 3), ( 2, 3), ( 3, 3), ( 4, 3), ( 5, 3), ( 6, 3), ( 7, 3), ( 8, 3),
    ( 0, 4), ( 1, 4), ( 2, 4), ( 3, 4), ( 4, 4), ( 5, 4), ( 6, 4), ( 7, 4), ( 8, 4), ( 9, 4),
( 0, 5), ( 1, 5), ( 2, 5), ( 3, 5), ( 4, 5), ( 5, 5), ( 6, 5), ( 7, 5), ( 8, 5), ( 9, 5), (10, 5),
    ( 1, 6), ( 2, 6), ( 3, 6), ( 4, 6), ( 5, 6), ( 6, 6), ( 7, 6), ( 8, 6), ( 9, 6), (10, 6),
        ( 2, 7), ( 3, 7), ( 4, 7), ( 5, 7), ( 6, 7), ( 7, 7), ( 8, 7), ( 9, 7), (10, 7),
            ( 3, 8), ( 4, 8), ( 5, 8), ( 6, 8), ( 7, 8), ( 8, 8), ( 9, 8), (10, 8),
                ( 4, 9), ( 5, 9), ( 6, 9), ( 7, 9), ( 8, 9), ( 9, 9), (10, 9),
                    ( 5,10), ( 6,10), ( 7,10), ( 8,10), ( 9,10), (10,10),
];

pub const COORDS_TO_IX: &'static[&'static[usize]] = &[
    &[ 0, 1, 2, 3, 4, 5],
    &[ 6, 7, 8, 9,10,11,12],
    &[13,14,15,16,17,18,19,20],
    &[21,22,23,24,25,26,27,28,29],
    &[30,31,32,33,34,35,36,37,38,39],
    &[40,41,42,43,44,45,46,47,48,49,50],
    &[51,52,53,54,55,56,57,58,59,60],
    &[61,62,63,64,65,66,67,68,69],
    &[70,71,72,73,74,75,76,77],
    &[78,79,80,81,82,83,84],
    &[85,86,87,88,89,90],
];

pub fn hex_coords_offset(x: usize, y: usize) -> [f64;2] {
    let v = [x as f64, y as f64];
    transform_vec(HEX_BASIS_TO_MODEL, v)
}
pub fn model_to_hex(x: f64, y: f64) -> (isize, isize) {
    let xy = transform_vec(MODEL_BASIS_TO_HEX, [x, y]);
    (xy[0].round() as isize, xy[1].round() as isize)
}
pub const HEX_HEIGHT: f64 = 1.0;
pub const HEX_WIDTH: f64 = 0.86602540378 * HEX_HEIGHT; // sqrt(3)/2

/// Convert the unit vector in hex coords to modelspace i.e. cartesian coords
pub const HEX_BASIS_TO_MODEL: [[f64; 3]; 2] = [
    [1.0 * HEX_WIDTH , -0.5  * HEX_WIDTH , 0.0],
    [0.0 * HEX_HEIGHT, -0.75 * HEX_HEIGHT, 0.0],
];

const DET: f64 = HEX_BASIS_TO_MODEL[0][0] * HEX_BASIS_TO_MODEL[1][1]
               - HEX_BASIS_TO_MODEL[0][1] * HEX_BASIS_TO_MODEL[1][0];
/// Inverse of HEX_BASIS_TO_MODEL
pub const MODEL_BASIS_TO_HEX: [[f64; 3]; 2] = [
    [ HEX_BASIS_TO_MODEL[1][1]/DET, -HEX_BASIS_TO_MODEL[0][1]/DET, 0.0],
    [-HEX_BASIS_TO_MODEL[1][0]/DET,  HEX_BASIS_TO_MODEL[0][0]/DET, 0.0]
];

#[cfg(test)]
mod tests {
    use super::*;
    use graphics::math::*;

    #[test]
    fn inverse_is_correct() {
        const EPS:f64 = 0.001;
        let forward_result = multiply(HEX_BASIS_TO_MODEL, MODEL_BASIS_TO_HEX);
        assert!((forward_result[0][0] - 1.0).abs() < EPS);
        assert!((forward_result[0][1] - 0.0).abs() < EPS);
        assert!((forward_result[0][2] - 0.0).abs() < EPS);

        assert!((forward_result[1][0] - 0.0).abs() < EPS);
        assert!((forward_result[1][1] - 1.0).abs() < EPS);
        assert!((forward_result[1][2] - 0.0).abs() < EPS);
    }
}
