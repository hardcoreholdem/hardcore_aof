use crate::aux::calc_alpha_pair;
use crate::aux::calc_beta_pair;
use crate::aux::join_calc_s_and_beta;
use crate::format::pretty_percent;
use crate::format::pretty_s;
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

    #[allow(non_snake_case)]
    let (alpha_AKo, alpha_JJ) = alpha13(equitizer, s13);
    println!("alpha_AKo: {}", pretty_percent(alpha_AKo));
    println!("alpha_JJ: {}", pretty_percent(alpha_JJ));

    #[allow(non_snake_case)]
    let (beta_AKo, beta_TT) = beta13(equitizer, s13);
    println!("beta_AKo: {}", pretty_percent(beta_AKo));
    println!("beta_TT: {}", pretty_percent(beta_TT));
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

fn alpha13(equitizer: &mut Equitizer, s: f64) -> (f64, f64) {
    let p_and_eq_0 = equitizer.query_prob_and_eq("AKo", "KK+,AK,ATs,A5s,A4s");
    let p_and_eq_1 = equitizer.query_prob_and_eq("AKo", "A3s");
    let p_and_eq_2 = equitizer.query_prob_and_eq("AKo", "TT");
    let p_and_eq_3 = equitizer.query_prob_and_eq("JJ", "KK+,AK,ATs,A5s,A4s");
    let p_and_eq_4 = equitizer.query_prob_and_eq("JJ", "A3s");
    let p_and_eq_5 = equitizer.query_prob_and_eq("JJ", "TT");

    #[allow(non_snake_case)]
    let (alpha_AKo, alpha_JJ) = calc_alpha_pair(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    (alpha_AKo, alpha_JJ)
}

fn beta13(equitizer: &mut Equitizer, s: f64) -> (f64, f64) {
    let p_and_eq_0 = equitizer.query_prob_and_eq("A3s", "KK+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("A3s", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("A3s", "JJ");
    let p_and_eq_3 = equitizer.query_prob_and_eq("TT", "KK+,AKs");
    let p_and_eq_4 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_5 = equitizer.query_prob_and_eq("TT", "JJ");

    #[allow(non_snake_case)]
    let (beta_AKo, beta_TT) = calc_beta_pair(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    (beta_AKo, beta_TT)
}
