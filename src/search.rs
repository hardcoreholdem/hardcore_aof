use crate::types::S;

pub fn binary_search(low: S, high: S, mut f: impl FnMut(S) -> f64) -> S {
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

fn binary_search_increasing(mut low: S, mut high: S, mut f: impl FnMut(S) -> f64) -> S {
    for _ in 0..100 {
        let mid = high.midpoint(low);
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

    low.midpoint(high)
}

fn binary_search_decreasing(mut low: S, mut high: S, mut f: impl FnMut(S) -> f64) -> S {
    for _ in 0..100 {
        let mid = high.midpoint(low);
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

    high.midpoint(low)
}
