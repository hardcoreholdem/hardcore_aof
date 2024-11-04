use std::cmp::max;

const EPS: f64 = 1e-9;

pub fn pretty_s(v: f64) -> String {
    let rounded = v.round();
    if (v - rounded).abs() < EPS {
        if v.abs() < EPS {
            "0".into()
        } else {
            format!("{:.0}", rounded)
        }
    } else if v.abs() > 1.0 {
        format!("{:.2}", v)
    } else {
        let log10 = v.abs().log10();
        let digits = (-log10.floor() + 2.0) as usize;
        format!("{v:.digits$}")
    }
}

pub fn pretty_percent(v: f64) -> String {
    let v = v * 100.0;

    let rounded = v.round();
    if (v - rounded).abs() < EPS {
        if v.abs() < EPS {
            "0%".into()
        } else {
            format!("{:.0}%", rounded)
        }
    } else {
        let abs_v = v.abs();

        // 三位有效数字
        let digits1 = (-abs_v.log10().floor() + 2.0) as usize;

        let digits = {
            if v > 100.0 {
                digits1
            } else {
                let digits2 = (-(100.0 - abs_v).log10().floor() + 2.0) as usize;
                max(digits1, digits2)
            }
        };

        format!("{v:.digits$}%")
    }
}
