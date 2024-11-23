use crate::calc_alpha::calc_alpha_1d;
use crate::calc_beta::calc_beta_1d;
use hardcore_aof::aux;
use hardcore_aof::format;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
pub fn section10(equitizer: &mut Equitizer) {
    let s10 = calc_s10(equitizer);
    println!("s10: {}", s10);

    println!(
        "alpha10(s10)={}",
        format::pretty_percent(calc_alpha10(equitizer, s10))
    );
    println!(
        "beta10(s10)={}",
        format::pretty_percent(calc_beta10(equitizer, s10))
    );
}

fn calc_s10(equitizer: &mut Equitizer) -> S {
    let p_and_eq = equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("KK+"));
    aux::calc_s(p_and_eq)
}

fn calc_alpha10(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p0, eq0) =
        equitizer.query_prob_and_eq(&PureRange::from("AKs"), &PureRange::from("KK+AKA5sA4s"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("AKs"), &PureRange::from("ATs"));
    calc_alpha_1d((p0, eq0), (p1, eq1), s)
}

pub fn calc_beta10(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("KK+"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("AKs"));
    calc_beta_1d((p0, eq0), (p1, eq1), s)
}
