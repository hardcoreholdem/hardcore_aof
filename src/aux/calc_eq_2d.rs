pub fn calc_eq_2d(
    (p_0, eq_0): (f64, f64),
    (scale_1, p_1, eq_1): (f64, f64, f64),
    (scale_2, p_2, eq_2): (f64, f64, f64),
) -> f64 {
    let num = p_0 * eq_0 + scale_1 * p_1 * eq_1 + scale_2 * p_2 * eq_2;
    let den = p_0 + scale_1 * p_1 + scale_2 * p_2;

    num / den
}
