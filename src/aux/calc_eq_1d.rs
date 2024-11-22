pub fn calc_eq_1d((p0, eq0): (f64, f64), scale_1: f64, (p1, eq1): (f64, f64)) -> f64 {
    let num = p0 * eq0 + scale_1 * p1 * eq1;
    let den = p0 + scale_1 * p1;

    num / den
}
