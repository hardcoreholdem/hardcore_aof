use hardcore_aof::math;
use hardcore_aof::types::S;

pub fn calc_alpha_1d(
    // (p0 eq0 + α p1 eq1) / (p0 + α p1) = (s-1) / (2s)
    (p0, eq0): (f64, f64),
    (p1, eq1): (f64, f64),
    s: S,
) -> f64 {
    let s: f64 = s.into();

    // a x + b = 0
    let a = p1 * eq1 * (2.0 * s) - p1 * (s - 1.0);
    let b = p0 * eq0 * (2.0 * s) - p0 * (s - 1.0);

    math::solve_linear_eq_1d(a, b)
}
