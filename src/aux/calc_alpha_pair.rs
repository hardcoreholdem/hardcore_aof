use super::solve_linear_eq::solve_linear_eq;

pub fn calc_alpha_pair(
    (p0, eq0, p1, eq1, p2, eq2): (f64, f64, f64, f64, f64, f64),
    (p3, eq3, p4, eq4, p5, eq5): (f64, f64, f64, f64, f64, f64),
    s: f64,
) -> (f64, f64) {
    // ax + by + c = 0
    let a = p1 * eq1 * (2.0 * s + 1.0) - p1 * s;
    let b = p2 * eq2 * (2.0 * s + 1.0) - p2 * s;
    let c = p0 * eq0 * (2.0 * s + 1.0) - p0 * s;

    // dx + ey + f = 0
    let d = p4 * eq4 * (2.0 * s + 1.0) - p4 * s;
    let e = p5 * eq5 * (2.0 * s + 1.0) - p5 * s;
    let f = p3 * eq3 * (2.0 * s + 1.0) - p3 * s;

    solve_linear_eq((a, b, c), (d, e, f))
}
