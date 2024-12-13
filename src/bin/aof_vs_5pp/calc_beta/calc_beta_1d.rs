use hardcore_aof::aux::solve_linear_eq_1d;
use hardcore_aof::types::S;

pub fn calc_beta_1d(
    // p0 (2s eq0 - s) + β p1 (2s eq1 - s) + (1 - p0 - β p1) = 0
    (p0, eq0): (f64, f64),
    (p1, eq1): (f64, f64),
    s: S,
) -> f64 {
    let s: f64 = s.into();

    // a x + b = 0
    let a = p1 * (eq1 * (2.0 * s) - s) - p1;
    let b = p0 * (eq0 * (2.0 * s) - s) + 1.0 - p0;

    solve_linear_eq_1d(a, b)
}
