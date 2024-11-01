pub fn solve_linear_eq_3d(
    (a0, b0, c0, d0): (f64, f64, f64, f64), // ax + by + c z + d = 0
    (a1, b1, c1, d1): (f64, f64, f64, f64), // ax + by + c z + d = 0
    (a2, b2, c2, d2): (f64, f64, f64, f64), // ax + by + c z + d = 0
) -> (f64, f64, f64) {
    let det = det3(
        (a0, b0, c0), //
        (a1, b1, c1), //
        (a2, b2, c2), //
    );

    let det_x = det3(
        (-d0, b0, c0), //
        (-d1, b1, c1), //
        (-d2, b2, c2), //
    );

    let det_y = det3(
        (a0, -d0, c0), //
        (a1, -d1, c1), //
        (a2, -d2, c2), //
    );

    let det_z = det3(
        (a0, b0, -d0), //
        (a1, b1, -d1), //
        (a2, b2, -d2), //
    );

    let x = det_x / det;
    let y = det_y / det;
    let z = det_z / det;

    (x, y, z)
}

pub fn det3(
    (a, b, c): (f64, f64, f64), // first row
    (d, e, f): (f64, f64, f64), // second row
    (g, h, i): (f64, f64, f64), // thrid row
) -> f64 {
    let pos = a * e * i + b * f * g + c * d * h;
    let neg = c * e * g + a * f * h + b * d * i;

    pos - neg
}
