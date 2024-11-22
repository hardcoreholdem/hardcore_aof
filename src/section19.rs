use hardcore_equitizer::Equitizer;

use crate::aux;
use crate::format::pretty_percent;
use crate::format::pretty_s;
use crate::research_attacker::research_attacker_2d;
use crate::research_defender::research_defender_2d;
use crate::search::binary_search;
use crate::section18::calc_alpha18;
use crate::section18::calc_beta18;
use crate::types::BetaAKoQQ;
use std::fmt;

pub fn section19(equitizer: &mut Equitizer) {
    let s = 170.0;
    let alpha = calc_alpha18(equitizer, s);
    let beta = calc_beta18(equitizer, s);

    println!("alpha18 = {}", alpha);
    println!("beta18 = {}", beta);

    println!("");

    research_attacker_2d(
        equitizer, "KK+,AKs", "AKo", beta.ako_1, "QQ", beta.qq_2, s, 15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AQs+,ATs,A5s,A4s,AKo",
        "TT",
        alpha.tt_1,
        "A3s",
        alpha.a3s_2,
        s,
        10,
    );

    for s in [170.0, 160.0, 150.0, 140.0, 130.0, 120.0] {
        let alpha = calc_alpha18(equitizer, s);
        let beta = calc_beta18(equitizer, s);
        println!("s = {}, alpha = {}, beta = {}", pretty_s(s), alpha, beta);
    }
    println!("");

    let s = search_s_for_beta18_qq_equals_1(equitizer);
    let alpha = calc_alpha18(equitizer, s);
    let beta = calc_beta18(equitizer, s);

    println!("s = {}", pretty_s(s));
    println!("alpha(s) = {}", alpha);
    println!("beta(s) = {}", beta);
    println!("");

    research_attacker_2d(
        equitizer, "KK+,AKs", "AKo", beta.ako_1, "QQ", beta.qq_2, s, 20,
    );

    // research_defender_2d(
    //     equitizer,
    //     "QQ+,AK,ATs,A5s-A3s",
    //     "TT",
    //     alpha.tt,
    //     "AQs",
    //     alpha.aqs,
    //     s18,
    //     10,
    // );

    let s19 = search_s19_for_attacker_ev_of_ats_equals_0(equitizer);
    let alpha = calc_alpha18(equitizer, s19);
    let beta = calc_beta18(equitizer, s19);

    println!("s19 = {}", pretty_s(s19));
    println!("alpha19(s19) = {}", alpha);
    println!("beta19(s19) = {}", beta);
    println!("");

    research_attacker_2d(
        equitizer, "KK+,AKs", "AKo", beta.ako_1, "QQ", beta.qq_2, s19, 15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AQs+,ATs,A5s,A4s,AKo",
        "TT",
        alpha.tt_1,
        "A3s",
        alpha.a3s_2,
        s19,
        10,
    );

    let alpha19 = calc_alpha19(equitizer, s19);
    let beta19 = calc_beta19(equitizer, s19);
    println!("alpha19(s19) = {}", alpha19);
    println!("beta19(s19) = {}", beta19);
}

fn search_s_for_beta18_qq_equals_1(equitizer: &mut Equitizer) -> f64 {
    let f = |s: f64| calc_beta18(equitizer, s).qq_2 - 1.0;
    binary_search(120.0, 130.0, f)
}

fn search_s19_for_attacker_ev_of_ats_equals_0(equitizer: &mut Equitizer) -> f64 {
    let f = |s: f64| -> f64 {
        let beta = calc_beta18(equitizer, s);
        aux::calc_attacker_ev_2d(
            equitizer.query_prob_and_eq("ATs", "KK+,AKs"),
            beta.ako_1,
            equitizer.query_prob_and_eq("ATs", "AKo"),
            beta.qq_2,
            equitizer.query_prob_and_eq("ATs", "QQ"),
            s,
        )
    };

    binary_search(120.0, 170.0, f)
}

pub struct Alpha19 {
    pub tt_1: f64,
    pub ats_2: f64,
}

impl fmt::Display for Alpha19 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TT:{},ATs:{}",
            pretty_percent(self.tt_1),
            pretty_percent(self.ats_2)
        )
    }
}

pub fn calc_alpha19(equitizer: &mut Equitizer, s: f64) -> Alpha19 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("AKo", "QQ+,AQs+,A5s-A3s,AKo");
    let p_and_eq_1 = equitizer.query_prob_and_eq("AKo", "TT");
    let p_and_eq_2 = equitizer.query_prob_and_eq("AKo", "ATs");
    let p_and_eq_3 = equitizer.query_prob_and_eq("QQ", "QQ+,AQs+,A5s-A3s,AKo");
    let p_and_eq_4 = equitizer.query_prob_and_eq("QQ", "TT");
    let p_and_eq_5 = equitizer.query_prob_and_eq("QQ", "ATs");

    let (tt_1, ats_2) = aux::calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha19 { tt_1, ats_2 }
}

pub fn calc_beta19(equitizer: &mut Equitizer, s: f64) -> BetaAKoQQ {
    let p_and_eq_0 = equitizer.query_prob_and_eq("TT", "KK+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("TT", "QQ");
    let p_and_eq_3 = equitizer.query_prob_and_eq("ATs", "KK+,AKs");
    let p_and_eq_4 = equitizer.query_prob_and_eq("ATs", "AKo");
    let p_and_eq_5 = equitizer.query_prob_and_eq("ATs", "QQ");

    let (ako_1, qq_2) = aux::calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    BetaAKoQQ { ako_1, qq_2 }
}
