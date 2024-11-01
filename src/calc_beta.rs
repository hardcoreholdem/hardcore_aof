pub fn calc_beta(p0: f64, eq0: f64, p1: f64, eq1: f64, s: f64) -> f64 {
    // a x + b = 0
    let a = p1 * (eq1 * (2.0 * s + 1.0) - s) - p1;
    let b = p0 * (eq0 * (2.0 * s + 1.0) - s) + 1.0 - p0;

    -b / a
}
