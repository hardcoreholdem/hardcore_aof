use super::calc_alpha::calc_alpha;
use super::calc_beta::calc_beta;
use super::calc_s::calc_s;
use super::format::pretty_s;
use crate::format::pretty_percent;
use crate::join_calc_s_and_beta::join_calc_s_and_beta;
use crate::search::binary_search;
use crate::section12::beta12;
use hardcore_equitizer::Equitizer;

pub fn section13(equitizer: &mut Equitizer) {
    let s = search_s_for_beta_12_equals_1(equitizer);
    println!("s: {}", pretty_s(s));
    println!("");

    let (s13, beta) = calc_s13_and_beta(equitizer);
    println!("s13: {}", pretty_s(s13));
    println!("beta: {}", pretty_percent(beta));
}

fn search_s_for_beta_12_equals_1(equitizer: &mut Equitizer) -> f64 {
    let f = |s| beta12(equitizer, s) - 1.0;
    binary_search(0.0, 300.0, f)
}

pub fn calc_s13_and_beta(equitizer: &mut Equitizer) -> (f64, f64) {
    let p_and_eq_0 = equitizer.query_prob_and_eq("TT", "KK+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("A3s", "KK+,AKs");
    let p_and_eq_3 = equitizer.query_prob_and_eq("A3s", "AKo");

    join_calc_s_and_beta((p_and_eq_0, p_and_eq_1), (p_and_eq_2, p_and_eq_3))
}
