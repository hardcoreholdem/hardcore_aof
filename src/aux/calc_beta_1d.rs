use super::solve_linear_eq_1d::solve_linear_eq_1d;
use crate::types::S;

pub fn calc_beta_1d(
    // p0 (eq0 (2s+1) - s) + β p1 (eq1 (2s+1) - s) + (1 - p0 - βp1) = 0
    (p0, eq0): (f64, f64),
    (p1, eq1): (f64, f64),
    s: S,
) -> f64 {
    let s: f64 = s.into();
    // a x + b = 0
    let a = p1 * (eq1 * (2.0 * s + 1.0) - s) - p1;
    let b = p0 * (eq0 * (2.0 * s + 1.0) - s) + 1.0 - p0;

    solve_linear_eq_1d(a, b)
}
