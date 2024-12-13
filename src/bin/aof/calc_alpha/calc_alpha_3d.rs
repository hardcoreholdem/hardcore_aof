use hardcore_aof::math;
use hardcore_aof::types::S;

pub fn calc_alpha_3d(
    ((p00, eq00), (p01, eq01), (p02, eq02), (p03, eq03)): (
        (f64, f64),
        (f64, f64),
        (f64, f64),
        (f64, f64),
    ),
    ((p10, eq10), (p11, eq11), (p12, eq12), (p13, eq13)): (
        (f64, f64),
        (f64, f64),
        (f64, f64),
        (f64, f64),
    ),
    ((p20, eq20), (p21, eq21), (p22, eq22), (p23, eq23)): (
        (f64, f64),
        (f64, f64),
        (f64, f64),
        (f64, f64),
    ),
    s: S,
) -> (f64, f64, f64) {
    let s: f64 = s.into();

    // ax + by + cz + d = 0
    let a0 = p01 * eq01 * (2.0 * s + 1.0) - p01 * s;
    let b0 = p02 * eq02 * (2.0 * s + 1.0) - p02 * s;
    let c0 = p03 * eq03 * (2.0 * s + 1.0) - p03 * s;
    let d0 = p00 * eq00 * (2.0 * s + 1.0) - p00 * s;

    let a1 = p11 * eq11 * (2.0 * s + 1.0) - p11 * s;
    let b1 = p12 * eq12 * (2.0 * s + 1.0) - p12 * s;
    let c1 = p13 * eq13 * (2.0 * s + 1.0) - p13 * s;
    let d1 = p10 * eq10 * (2.0 * s + 1.0) - p10 * s;

    let a2 = p21 * eq21 * (2.0 * s + 1.0) - p21 * s;
    let b2 = p22 * eq22 * (2.0 * s + 1.0) - p22 * s;
    let c2 = p23 * eq23 * (2.0 * s + 1.0) - p23 * s;
    let d2 = p20 * eq20 * (2.0 * s + 1.0) - p20 * s;

    math::solve_linear_eq_3d((a0, b0, c0, d0), (a1, b1, c1, d1), (a2, b2, c2, d2))
}
