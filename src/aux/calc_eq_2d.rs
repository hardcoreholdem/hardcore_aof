// TODO: rename file
pub fn calc_eq_2d(
    (p0, eq0): (f64, f64),
    (p1, eq1): (f64, f64),
    (p2, eq2): (f64, f64),
    scale_1: f64,
    scale_2: f64,
) -> f64 {
    let num = p0 * eq0 + scale_1 * p1 * eq1 + scale_2 * p2 * eq2;
    let den = p0 + scale_1 * p1 + scale_2 * p2;

    num / den
}
