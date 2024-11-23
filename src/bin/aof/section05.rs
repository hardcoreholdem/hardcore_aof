use crate::section01::calc_alpha_new;
use crate::section04::calc_alpha4;
use crate::section04::calc_beta4;
use crate::section04::calc_s4_and_beta;
use hardcore_aof::aux::calc_beta_1d;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;

pub fn section05(equitizer: &mut Equitizer) {
    let (s4, _) = calc_s4_and_beta(equitizer);

    for s in [
        s4,
        465.into(),
        460.into(),
        450.into(),
        440.into(),
        430.into(),
        420.into(),
        410.into(),
        400.into(),
    ] {
        let alpha4 = calc_alpha4(equitizer, s);
        println!(
            "s: {:.2}, alpha4_ATs: {}, alpha4_AKo: {}",
            s,
            pretty_percent(alpha4.ats),
            pretty_percent(alpha4.ako)
        );

        let beta4 = calc_beta4(equitizer, s);
        println!("beta4_AKs: {}", pretty_percent(beta4.aks));
        println!("beta4_KK: {}", pretty_percent(beta4.kk));
    }

    println!("s4: {}", s4);

    let s5 = search_s_for_beta4_equals_0(equitizer, 0.into(), s4);
    println!("s5: {}", s5);

    println!("alpha5(s5): {}", pretty_percent(calc_alpha5(equitizer, s5)));

    println!("beta5(s5): {}", pretty_percent(calc_beta5(equitizer, s5)));
}

fn search_s_for_beta4_equals_0(equitizer: &mut Equitizer, mut low: S, mut high: S) -> S {
    // TODO: use the universal search fn
    if calc_beta4(equitizer, low).aks >= 0.0 {
        panic!("!(beta4(equitizer, low).0 < 0)");
    }
    if calc_beta4(equitizer, high).aks <= 0.0 {
        panic!("!(beta4(equitizer, high).0 > 0.0)");
    }

    for _ in 0..100 {
        let mid = high.midpoint(low);

        let beta4 = calc_beta4(equitizer, mid);

        match beta4.aks.signum() {
            0.0 => return mid,
            1.0 => high = mid,
            -1.0 => low = mid,
            _ => panic!("beta4_AKs.signum() is not 0, -1, or 1"),
        }
    }

    low.midpoint(high)
}

fn calc_alpha5(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("AKo"));
    let (p2, eq2) =
        equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("AA,AKs,A5s"));

    calc_alpha_new((p1, eq1), (p2, eq2), s)
}

fn calc_beta5(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("AA"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("KK"));

    calc_beta_1d((p0, eq0), (p1, eq1), s)
}
