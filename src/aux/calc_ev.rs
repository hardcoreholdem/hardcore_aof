pub fn calc_attacker_ev_2d(
    (p0, eq0): (f64, f64),
    (p1, eq1): (f64, f64),
    (p2, eq2): (f64, f64),
    beta1: f64,
    beta2: f64,
    s: f64,
) -> f64 {
    let v0 = p0 * (eq0 * (2.0 * s + 1.0) - s);
    let v1 = beta1 * p1 * (eq1 * (2.0 * s + 1.0) - s);
    let v2 = beta2 * p2 * (eq2 * (2.0 * s + 1.0) - s);
    let v3 = 1.0 - p0 - beta1 * p1 - beta2 * p2;

    v0 + v1 + v2 + v3
}
