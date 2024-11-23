use crate::types::S;
pub fn calc_attacker_ev_2d(
    (p_0, eq_0): (f64, f64),
    (beta_1, p_1, eq_1): (f64, f64, f64),
    (beta_2, p_2, eq_2): (f64, f64, f64),
    s: S,
) -> f64 {
    let s: f64 = s.into();

    let v0 = p_0 * (eq_0 * (2.0 * s + 1.0) - s);
    let v1 = beta_1 * p_1 * (eq_1 * (2.0 * s + 1.0) - s);
    let v2 = beta_2 * p_2 * (eq_2 * (2.0 * s + 1.0) - s);
    let v_fold = 1.0 - p_0 - beta_1 * p_1 - beta_2 * p_2;

    v0 + v1 + v2 + v_fold
}

pub fn calc_attacker_ev_1d(
    (p_0, eq_0): (f64, f64),
    beta_1: f64,
    (p_1, eq_1): (f64, f64),
    s: S,
) -> f64 {
    let s: f64 = s.into();

    let v0 = p_0 * (eq_0 * (2.0 * s + 1.0) - s);
    let v1 = beta_1 * p_1 * (eq_1 * (2.0 * s + 1.0) - s);
    let v_fold = (1.0 - p_0 - beta_1 * p_1) * 1.0;

    v0 + v1 + v_fold
}

pub fn calc_attacker_ev_0d((p_0, eq_0): (f64, f64), s: S) -> f64 {
    let s: f64 = s.into();

    let v0 = p_0 * (eq_0 * (2.0 * s + 1.0) - s);
    let v_fold = (1.0 - p_0) * 1.0;

    v0 + v_fold
}
