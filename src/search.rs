pub fn binary_search(low: f64, high: f64, mut f: impl FnMut(f64) -> f64) -> f64 {
    let f_low = f(low);
    let f_high = f(high);

    match (f_low.signum(), f_high.signum()) {
        (1.0, 1.0) => panic!("f(low) and f(high) have the same sign"),
        (1.0, -1.0) => binary_search_decreasing(low, high, f),
        (-1.0, 1.0) => binary_search_increasing(low, high, f),
        (-1.0, -1.0) => panic!("f(low) and f(high) have the same sign"),
        _ => panic!("sign of f(low) or f(high) is not 1 or -1"),
    }
}

fn binary_search_increasing(mut low: f64, mut high: f64, mut f: impl FnMut(f64) -> f64) -> f64 {
    for _ in 0..100 {
        let mid = (high + low) / 2.0;
        let f_mid = f(mid);

        match f_mid.signum() {
            0.0 => return mid,
            1.0 => {
                high = mid;
            }
            -1.0 => {
                low = mid;
            }
            _ => panic!("f_mid.signum() is not 0, -1, or 1"),
        }
    }

    (low + high) / 2.0
}

fn binary_search_decreasing(mut low: f64, mut high: f64, mut f: impl FnMut(f64) -> f64) -> f64 {
    for _ in 0..100 {
        let mid = (high + low) / 2.0;
        let f_mid = f(mid);

        match f_mid.signum() {
            0.0 => return mid,
            1.0 => {
                low = mid;
            }
            -1.0 => {
                high = mid;
            }
            _ => panic!("f_mid.signum() is not 0, -1, or 1"),
        }
    }

    (low + high) / 2.0
}
