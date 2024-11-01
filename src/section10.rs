use crate::format::pretty_percent;

use crate::aux::calc_alpha_1d;
use crate::aux::calc_beta_1d;
use crate::aux::calc_s;
use crate::format::pretty_s;
use hardcore_equitizer::Equitizer;

pub fn section10(equitizer: &mut Equitizer) {
    let s10 = calc_s10(equitizer);
    println!("s10: {}", pretty_s(s10));

    println!(
        "alpha10(s10)={}",
        pretty_percent(calc_alpha10(equitizer, s10))
    );
    println!(
        "beta10(s10)={}",
        pretty_percent(calc_beta10(equitizer, s10))
    );
}

fn calc_s10(equitizer: &mut Equitizer) -> f64 {
    let p_and_eq = equitizer.query_prob_and_eq("ATs", "KK+");
    calc_s(p_and_eq)
}

fn calc_alpha10(equitizer: &mut Equitizer, s: f64) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq("AKs", "KK+,AK,A5s,A4s");
    let (p1, eq1) = equitizer.query_prob_and_eq("AKs", "ATs");
    calc_alpha_1d((p0, eq0), (p1, eq1), s)
}

pub fn calc_beta10(equitizer: &mut Equitizer, s: f64) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq("ATs", "KK+");
    let (p1, eq1) = equitizer.query_prob_and_eq("ATs", "AKs");
    calc_beta_1d((p0, eq0), (p1, eq1), s)
}
