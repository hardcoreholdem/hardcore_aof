use crate::aux::calc_alpha_3d;
use crate::aux::calc_beta_3d;
use crate::aux::calc_ev_two_betas;
use crate::format::pretty_percent;
use crate::format::pretty_s;
use crate::search::binary_search;
use crate::section13::calc_alpha13;
use crate::section13::calc_beta13;
use hardcore_equitizer::Equitizer;

// TODO: remove all #[allow(non_snake_case)]

pub fn section14(equitizer: &mut Equitizer) {
    let s14 = search_s14_for_ev_qq_equals_0(equitizer);
    println!("s14={}", pretty_s(s14));

    #[allow(non_snake_case)]
    let (alpha_A3s, alpha_TT) = calc_alpha13(equitizer, s14);
    println!("alpha_A3s={}", pretty_percent(alpha_A3s));
    println!("alpha_TT={}", pretty_percent(alpha_TT));
    println!("");

    #[allow(non_snake_case)]
    let (beta_AKo, beta_JJ) = calc_beta13(equitizer, s14);
    println!("beta_AKo={}", pretty_percent(beta_AKo));
    println!("beta_JJ={}", pretty_percent(beta_JJ));
    println!("");

    #[allow(non_snake_case)]
    let (alpha_A3s, alpha_TT, alpha_QQ) = calc_alpha14(equitizer, s14);
    println!("alpha_A3s={}", pretty_percent(alpha_A3s));
    println!("alpha_TT={}", pretty_percent(alpha_TT));
    println!("alpha_QQ={}", pretty_percent(alpha_QQ));
    println!("");

    #[allow(non_snake_case)]
    let beta14 = calc_beta14(equitizer, s14);
    println!("beta_AKo={}", pretty_percent(beta14.ako));
    println!("beta_JJ={}", pretty_percent(beta14.jj));
    println!("beta_QQ={}", pretty_percent(beta14.qq));
    println!("");
}

fn search_s14_for_ev_qq_equals_0(equitizer: &mut Equitizer) -> f64 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("QQ", "KK+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("QQ", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("QQ", "JJ");

    let f = |s| {
        let (beta_1, beta_2) = calc_beta13(equitizer, s);
        calc_ev_two_betas(p_and_eq_0, p_and_eq_1, p_and_eq_2, beta_1, beta_2, s)
    };

    binary_search(0.0, 300.0, f)
}

pub fn calc_alpha14(equitizer: &mut Equitizer, s: f64) -> (f64, f64, f64) {
    let p_and_eq_00 = equitizer.query_prob_and_eq("AKo", "KK+,AK,ATs,A5s,A4s");
    let p_and_eq_01 = equitizer.query_prob_and_eq("AKo", "A3s");
    let p_and_eq_02 = equitizer.query_prob_and_eq("AKo", "TT");
    let p_and_eq_03 = equitizer.query_prob_and_eq("AKo", "QQ");
    let p_and_eq_10 = equitizer.query_prob_and_eq("JJ", "KK+,AK,ATs,A5s,A4s");
    let p_and_eq_11 = equitizer.query_prob_and_eq("JJ", "A3s");
    let p_and_eq_12 = equitizer.query_prob_and_eq("JJ", "TT");
    let p_and_eq_13 = equitizer.query_prob_and_eq("JJ", "QQ");
    let p_and_eq_20 = equitizer.query_prob_and_eq("QQ", "KK+,AK,ATs,A5s,A4s");
    let p_and_eq_21 = equitizer.query_prob_and_eq("QQ", "A3s");
    let p_and_eq_22 = equitizer.query_prob_and_eq("QQ", "TT");
    let p_and_eq_23 = equitizer.query_prob_and_eq("QQ", "QQ");

    #[allow(non_snake_case)]
    let (alpha_A3s, alpha_TT, alpha_QQ) = calc_alpha_3d(
        (p_and_eq_00, p_and_eq_01, p_and_eq_02, p_and_eq_03),
        (p_and_eq_10, p_and_eq_11, p_and_eq_12, p_and_eq_13),
        (p_and_eq_20, p_and_eq_21, p_and_eq_22, p_and_eq_23),
        s,
    );

    (alpha_A3s, alpha_TT, alpha_QQ)
}

pub struct Beta14 {
    pub ako: f64,
    pub jj: f64,
    pub qq: f64,
}

pub fn calc_beta14(equitizer: &mut Equitizer, s: f64) -> Beta14 {
    let p_and_eq_00 = equitizer.query_prob_and_eq("A3s", "KK+,AKs");
    let p_and_eq_01 = equitizer.query_prob_and_eq("A3s", "AKo");
    let p_and_eq_02 = equitizer.query_prob_and_eq("A3s", "JJ");
    let p_and_eq_03 = equitizer.query_prob_and_eq("A3s", "QQ");
    let p_and_eq_10 = equitizer.query_prob_and_eq("TT", "KK+,AKs");
    let p_and_eq_11 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_12 = equitizer.query_prob_and_eq("TT", "JJ");
    let p_and_eq_13 = equitizer.query_prob_and_eq("TT", "QQ");
    let p_and_eq_20 = equitizer.query_prob_and_eq("QQ", "KK+,AKs");
    let p_and_eq_21 = equitizer.query_prob_and_eq("QQ", "AKo");
    let p_and_eq_22 = equitizer.query_prob_and_eq("QQ", "JJ");
    let p_and_eq_23 = equitizer.query_prob_and_eq("QQ", "QQ");

    #[allow(non_snake_case)]
    let (beta_AKo, beta_JJ, beta_QQ) = calc_beta_3d(
        (p_and_eq_00, p_and_eq_01, p_and_eq_02, p_and_eq_03),
        (p_and_eq_10, p_and_eq_11, p_and_eq_12, p_and_eq_13),
        (p_and_eq_20, p_and_eq_21, p_and_eq_22, p_and_eq_23),
        s,
    );

    Beta14 {
        ako: beta_AKo,
        jj: beta_JJ,
        qq: beta_QQ,
    }
}
