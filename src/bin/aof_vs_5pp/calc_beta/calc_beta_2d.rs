use hardcore_aof::aux::solve_linear_eq_2d;
use hardcore_aof::types::S;

pub fn calc_beta_2d(
    ((p0, eq0), (p1, eq1), (p2, eq2)): ((f64, f64), (f64, f64), (f64, f64)),
    ((p3, eq3), (p4, eq4), (p5, eq5)): ((f64, f64), (f64, f64), (f64, f64)),
    s: S,
) -> (f64, f64) {
    let s: f64 = s.into();

    // a x + b y + c = 0
    let a = p1 * (2.0 * s * eq1 - s) - p1;
    let b = p2 * (2.0 * s * eq2 - s) - p2;
    let c = p0 * (2.0 * s * eq0 - s) + (1.0 - p0) * 1.0;

    // c x + d y + e = 0
    let d = p4 * (2.0 * s * eq4 - s) - p4;
    let e = p5 * (2.0 * s * eq5 - s) - p5;
    let f = p3 * (2.0 * s * eq3 - s) + (1.0 - p3) * 1.0;

    solve_linear_eq_2d((a, b, c), (d, e, f))
}
