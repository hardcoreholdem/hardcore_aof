use super::calc_alpha::calc_alpha;
use super::calc_beta::calc_beta;
use super::calc_s::calc_s;
use super::format::pretty_s;
use crate::format::pretty_percent;
use hardcore_equitizer::Equitizer;

pub fn section12(equitizer: &mut Equitizer) {
    let s12 = calc_s12(equitizer);
    println!("s12: {}", pretty_s(s12));

    let alpha12 = alpha12(equitizer, s12);
    println!("alpha12: {}", pretty_percent(alpha12));

    let beta12 = beta12(equitizer, s12);
    println!("beta12: {}", pretty_percent(beta12));
}

pub fn calc_s12(equitizer: &mut Equitizer) -> f64 {
    let p_and_e = equitizer.query_prob_and_eq("A3s", "KK+,AKs");

    calc_s(p_and_e)
}

pub fn alpha12(equitizer: &mut Equitizer, s: f64) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq("AKo", "KK+,AK,ATs,A5s,A4s");
    let (p1, eq1) = equitizer.query_prob_and_eq("AKo", "A3s");

    calc_alpha((p0, eq0), (p1, eq1), s)
}

pub fn beta12(equitizer: &mut Equitizer, s: f64) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq("A3s", "KK+,AKs");
    let (p1, eq1) = equitizer.query_prob_and_eq("A3s", "AKo");

    calc_beta((p0, eq0), (p1, eq1), s)
}
