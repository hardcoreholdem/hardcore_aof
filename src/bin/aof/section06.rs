use crate::calc_alpha::calc_alpha_1d;
use hardcore_aof::aux::calc_beta_1d;
use hardcore_aof::aux::join_calc_s_and_beta;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
pub fn section06(equitizer: &mut Equitizer) {
    println!("# section 7");

    let (s6, beta) = calc_s6_and_beta(equitizer);

    println!("s: {:.2}", s6);
    println!("beta: {:.2}%", beta * 100.0);

    println!("alpha6(s6): {:.2}%", calc_alpha6(equitizer, s6) * 100.0);
}

fn calc_s6_and_beta(equitizer: &mut Equitizer) -> (S, f64) {
    let (p0, eq0) = equitizer.query_prob_and_eq(&PureRange::from("A5s"), &PureRange::from("AA"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("A5s"), &PureRange::from("KK"));
    let (p2, eq2) = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("AA"));
    let (p3, eq3) = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("KK"));

    join_calc_s_and_beta(((p0, eq0), (p1, eq1)), ((p2, eq2), (p3, eq3)))
}

pub fn calc_beta6(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq(&PureRange::from("A5s"), &PureRange::from("AA"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("A5s"), &PureRange::from("KK"));

    calc_beta_1d((p0, eq0), (p1, eq1), s)
}

pub fn calc_alpha6(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("AA,AK"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("A5s"));

    calc_alpha_1d((p0, eq0), (p1, eq1), s)
}
