pub fn calc_beta(
    // p0 (eq0 (2s+1) - s) + β p1 (eq1 (2s+1) - s) + (1 - p0 - βp1) = 0
    (p0, eq0): (f64, f64),
    (p1, eq1): (f64, f64),
    s: f64,
) -> f64 {
    // a x + b = 0
    let a = p1 * (eq1 * (2.0 * s + 1.0) - s) - p1;
    let b = p0 * (eq0 * (2.0 * s + 1.0) - s) + 1.0 - p0;

    -b / a
}

pub fn join_calc_s_and_beta(
    // p0 (eq0 (2s+1) - s) + β p1 (eq1 (2s+1) - s) + (1 - p0 - βp1) = 0
    // p2 (eq2 (2s+1) - s) + β p3 (eq3 (2s+1) - s) + (1 - p2 - βp3) = 0
    ((p0, eq0), (p1, eq1)): ((f64, f64), (f64, f64)),
    ((p2, eq2), (p3, eq3)): ((f64, f64), (f64, f64)),
) -> (f64, f64) {
    // a βs + b β + c s + d = 0
    let a = p1 * (eq1 * 2.0 - 1.0);
    let b = p1 * eq1 - p1 * 1.0;
    let c = p0 * (eq0 * 2.0 - 1.0);
    let d = p0 * eq0 + (1.0 - p0) * 1.0;

    // e βs + f β + g s + h = 0
    let e = p3 * (eq3 * 2.0 - 1.0);
    let f = p3 * eq3 - p3 * 1.0;
    let g = p2 * (eq2 * 2.0 - 1.0);
    let h = p2 * eq2 + (1.0 - p2) * 1.0;

    // A β^2 + B β + C = 0
    #[allow(non_snake_case)]
    let A = b * e - a * f;
    #[allow(non_snake_case)]
    let B = b * g + d * e - a * h - c * f;
    #[allow(non_snake_case)]
    let C = d * g - c * h;

    #[allow(non_snake_case)]
    let Delta = B * B - 4.0 * A * C;
    #[allow(non_snake_case)]
    let sqrt_Delta = Delta.sqrt();

    let beta1 = (-B + sqrt_Delta) / (2.0 * A);
    let beta2 = (-B - sqrt_Delta) / (2.0 * A);

    let beta = match (0.0 <= beta1 && beta1 <= 1.0, 0.0 <= beta2 && beta2 <= 1.0) {
        (true, false) => beta1,
        (false, true) => beta2,
        _ => panic!("no unique beta"),
    };

    let s1 = -(b * beta + d) / (a * beta + c);
    let s2 = -(f * beta + h) / (e * beta + g);
    if (s1 - s2).abs() > 1e-9 {
        panic!("s != s'");
    }

    let s = (s1 + s2) / 2.0;

    (s, beta)
}
