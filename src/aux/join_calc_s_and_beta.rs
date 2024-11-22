use crate::types::S;
pub fn join_calc_s_and_beta(
    // p0 (eq0 (2s+1) - s) + β p1 (eq1 (2s+1) - s) + (1 - p0 - βp1) = 0
    // p2 (eq2 (2s+1) - s) + β p3 (eq3 (2s+1) - s) + (1 - p2 - βp3) = 0
    ((p0, eq0), (p1, eq1)): ((f64, f64), (f64, f64)),
    ((p2, eq2), (p3, eq3)): ((f64, f64), (f64, f64)),
) -> (S, f64) {
    // e βs + f β + g s + h = 0
    let e: f64 = p1 * (eq1 * 2.0 - 1.0);
    let f = p1 * eq1 - p1 * 1.0;
    let g = p0 * (eq0 * 2.0 - 1.0);
    let h: f64 = p0 * eq0 + (1.0 - p0) * 1.0;

    // i βs + j β + k s + w = 0
    let i: f64 = p3 * (eq3 * 2.0 - 1.0);
    let j = p3 * eq3 - p3 * 1.0;
    let k = p2 * (eq2 * 2.0 - 1.0);
    let w = p2 * eq2 + (1.0 - p2) * 1.0;

    // a β^2 + b β + c = 0
    let a = f * i - e * j;
    let b = f * k + h * i - e * w - g * j;
    let c: f64 = h * k - g * w;

    let delta = b * b - 4.0 * a * c;
    let sqrt_delta = delta.sqrt();

    let beta1 = (-b + sqrt_delta) / (2.0 * a);
    let beta2 = (-b - sqrt_delta) / (2.0 * a);

    let beta = match (0.0 <= beta1 && beta1 <= 1.0, 0.0 <= beta2 && beta2 <= 1.0) {
        (true, false) => beta1,
        (false, true) => beta2,
        _ => panic!("no unique beta"),
    };

    let s1 = -(f * beta + h) / (e * beta + g);
    let s2 = -(j * beta + w) / (i * beta + k);
    if (s1 - s2).abs() > 1e-9 {
        panic!("s != s'");
    }

    let s = (s1 + s2) / 2.0;

    (s.into(), beta)
}
