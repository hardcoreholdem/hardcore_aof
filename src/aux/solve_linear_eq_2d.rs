pub fn solve_linear_eq_2d(
    (a, b, c): (f64, f64, f64), // ax + by + c = 0
    (d, e, f): (f64, f64, f64), // dx + ey + f = 0
) -> (f64, f64) {
    let det = det2(
        (a, b), //
        (d, e), //
    );

    let det_x = det2(
        (c, b), //
        (f, e), //
    );

    let det_y = det2(
        (a, c), //
        (d, f), //
    );

    let x = -det_x / det;
    let y = -det_y / det;

    (x, y)
}

pub fn det2(
    (a, b): (f64, f64), // first row
    (c, d): (f64, f64), // second row
) -> f64 {
    a * d - b * c
}
