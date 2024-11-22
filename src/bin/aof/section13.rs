use crate::section12::calc_beta12;
use hardcore_aof::aux;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;

pub fn section13(equitizer: &mut Equitizer) {
    let s = search_s_for_beta_12_equals_1(equitizer);
    println!("s: {}", s);
    println!("");

    let (s13, beta) = calc_s13_and_beta(equitizer);
    println!("s13: {}", s13);
    println!("beta: {}", pretty_percent(beta));

    let alpha13 = calc_alpha13(equitizer, s13);
    println!("alpha_A3s: {}", pretty_percent(alpha13.a3s));
    println!("alpha_TT: {}", pretty_percent(alpha13.tt));

    let beta13 = calc_beta13(equitizer, s13);
    println!("beta_AKo: {}", pretty_percent(beta13.ako_1));
    println!("beta_JJ: {}", pretty_percent(beta13.jj_2));
}

fn search_s_for_beta_12_equals_1(equitizer: &mut Equitizer) -> S {
    let f = |s| calc_beta12(equitizer, s) - 1.0;
    binary_search(0.into(), 300.into(), f)
}

pub fn calc_s13_and_beta(equitizer: &mut Equitizer) -> (S, f64) {
    let p_and_eq_0 = equitizer.query_prob_and_eq("TT", "KK+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("A3s", "KK+,AKs");
    let p_and_eq_3 = equitizer.query_prob_and_eq("A3s", "AKo");

    aux::join_calc_s_and_beta((p_and_eq_0, p_and_eq_1), (p_and_eq_2, p_and_eq_3))
}

pub struct Alpha13 {
    pub a3s: f64,
    pub tt: f64,
}

pub fn calc_alpha13(equitizer: &mut Equitizer, s: S) -> Alpha13 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("AKo", "KK+,AK,ATs,A5s,A4s");
    let p_and_eq_1 = equitizer.query_prob_and_eq("AKo", "A3s");
    let p_and_eq_2 = equitizer.query_prob_and_eq("AKo", "TT");
    let p_and_eq_3 = equitizer.query_prob_and_eq("JJ", "KK+,AK,ATs,A5s,A4s");
    let p_and_eq_4 = equitizer.query_prob_and_eq("JJ", "A3s");
    let p_and_eq_5 = equitizer.query_prob_and_eq("JJ", "TT");

    let (a3s, tt) = aux::calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha13 { a3s, tt }
}

pub struct Beta13 {
    pub ako_1: f64,
    pub jj_2: f64,
}

pub fn calc_beta13(equitizer: &mut Equitizer, s: S) -> Beta13 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("A3s", "KK+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("A3s", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("A3s", "JJ");
    let p_and_eq_3 = equitizer.query_prob_and_eq("TT", "KK+,AKs");
    let p_and_eq_4 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_5 = equitizer.query_prob_and_eq("TT", "JJ");

    let (ako, jj) = aux::calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Beta13 {
        ako_1: ako,
        jj_2: jj,
    }
}
