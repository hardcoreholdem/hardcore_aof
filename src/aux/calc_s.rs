pub fn calc_s(
    // p * (eq * (2s+1)) + (1-p) * 1 = 0
    (p, eq): (f64, f64),
) -> f64 {
    // a s + b = 0
    let a = p * (eq * 2.0 - 1.0);
    let b = p * eq + 1.0 - p;

    -b / a
}
