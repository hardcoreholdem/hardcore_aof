use crate::aux::calc_beta;
use crate::section01::calc_alpha_new;
use crate::section04::alpha4;
use crate::section04::beta4;
use crate::section04::calc_s4_and_beta;
use hardcore_equitizer::Equitizer;

fn search_s_for_beta4_equals_0(equitizer: &mut Equitizer, mut low: f64, mut high: f64) -> f64 {
    if beta4(equitizer, low).0 >= 0.0 {
        panic!("!(beta4(equitizer, low).0 < 0)");
    }
    if beta4(equitizer, high).0 <= 0.0 {
        panic!("!(beta4(equitizer, high).0 > 0.0)");
    }

    for _ in 0..100 {
        let mid = (high + low) / 2.0;

        #[allow(non_snake_case)]
        let (beta4_AKs, _) = beta4(equitizer, mid);

        match beta4_AKs.signum() {
            0.0 => return mid,
            1.0 => high = mid,
            -1.0 => low = mid,
            _ => panic!("beta4_AKs.signum() is not 0, -1, or 1"),
        }
    }

    (low + high) / 2.0
}

fn alpha5(equitizer: &mut Equitizer, s: f64) -> f64 {
    let (p1, eq1) = equitizer.query_prob_and_eq("KK", "AKo");
    let (p2, eq2) = equitizer.query_prob_and_eq("KK", "AA,AKs,A5s");

    calc_alpha_new((p1, eq1), (p2, eq2), s)
}

fn beta5(equitizer: &mut Equitizer, s: f64) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq("AKo", "AA");
    let (p1, eq1) = equitizer.query_prob_and_eq("AKo", "KK");

    calc_beta((p0, eq0), (p1, eq1), s)
}

pub fn section05(equitizer: &mut Equitizer) {
    println!("# section 6");

    let (s4, _) = calc_s4_and_beta(equitizer);

    for s in [s4, 465.0, 460.0, 450.0, 440.0, 430.0, 420.0, 410.0, 400.0] {
        #[allow(non_snake_case)]
        let (alpha4_ATs, alpha4_AKo) = alpha4(equitizer, s);
        println!(
            "s: {:.2}, alpha4_ATs: {:.2}%, alpha4_AKo: {:.2}%",
            s,
            alpha4_ATs * 100.0,
            alpha4_AKo * 100.0
        );

        #[allow(non_snake_case)]
        let (beta4_AKs, beta4_KK) = beta4(equitizer, s);
        println!("beta4_AKs: {:.2}%", beta4_AKs * 100.0);
        println!("beta4_KK: {:.2}%", beta4_KK * 100.0);
    }

    println!("s4: {:.2}", s4);

    let s5 = search_s_for_beta4_equals_0(equitizer, 0.0, s4);
    println!("s5: {:.2}", s5);

    println!("alpha5(s5): {:.2}%", alpha5(equitizer, s5) * 100.0);

    println!("beta5(s5): {:.2}%", beta5(equitizer, s5) * 100.0);
}
