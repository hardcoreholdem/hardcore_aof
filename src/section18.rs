use hardcore_equitizer::Equitizer;

use crate::aux;
use crate::format::pretty_percent;
use crate::research_attacker::research_attacker_2d;
use crate::research_defender::research_defender_2d;
use crate::search::binary_search;
use crate::section17::calc_alpha17;
use crate::section17::calc_beta17;
use crate::types::BetaAKoQQ;
use crate::types::S;
use std::fmt;

pub fn section18(equitizer: &mut Equitizer) {
    let s = 175.into();
    let alpha = calc_alpha17(equitizer, s);
    let beta = calc_beta17(equitizer, s);

    println!("alpha = {}", alpha);
    println!("beta = {}", beta);

    println!("");

    research_attacker_2d(
        equitizer, "KK+,AKs", "AKo", beta.ako_1, "QQ", beta.qq_2, s, 15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AK,ATs,A5s-A3s",
        "TT",
        alpha.tt,
        "AQs",
        alpha.aqs,
        s,
        10,
    );

    for s in [
        175.into(),
        170.into(),
        160.into(),
        150.into(),
        140.into(),
        130.into(),
    ] {
        let alpha = calc_alpha17(equitizer, s);
        let beta = calc_beta17(equitizer, s);
        println!("s = {}, alpha = {}, beta = {}", s, alpha, beta);
    }
    println!("");

    let s18 = search_s18_for_alpha17_aqs_ev_equals_0(equitizer);
    let alpha = calc_alpha17(equitizer, s18);
    let beta = calc_beta17(equitizer, s18);

    println!("s = {}", s18);
    println!("alpha(s) = {}", alpha);
    println!("beta(s) = {}", beta);
    println!("");

    research_attacker_2d(
        equitizer, "KK+,AKs", "AKo", beta.ako_1, "QQ", beta.qq_2, s18, 20,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AK,ATs,A5s-A3s",
        "TT",
        alpha.tt,
        "AQs",
        alpha.aqs,
        s18,
        10,
    );

    let s18 = search_s_for_attacker_ev_of_a3s_equals_0(equitizer);
    let alpha = calc_alpha17(equitizer, s18);
    let beta = calc_beta17(equitizer, s18);

    println!("s18 = {}", s18);
    println!("alpha17(s18) = {}", alpha);
    println!("beta17(s18) = {}", beta);
    println!("");

    research_attacker_2d(
        equitizer, "KK+,AKs", "AKo", beta.ako_1, "QQ", beta.qq_2, s18, 15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AK,ATs,A5s-A3s",
        "TT",
        alpha.tt,
        "AQs",
        alpha.aqs,
        s18,
        10,
    );

    let alpha18 = calc_alpha18(equitizer, s18);
    let beta18 = calc_beta18(equitizer, s18);
    println!("alpha18(s18) = {}", alpha18);
    println!("beta18(s18) = {}", beta18);
}

fn search_s18_for_alpha17_aqs_ev_equals_0(equitizer: &mut Equitizer) -> S {
    let f = |s| calc_alpha17(equitizer, s).aqs;
    binary_search(130.into(), 140.into(), f)
}

fn search_s_for_attacker_ev_of_a3s_equals_0(equitizer: &mut Equitizer) -> S {
    let f = |s| -> f64 {
        let beta = calc_beta17(equitizer, s);
        aux::calc_attacker_ev_2d(
            equitizer.query_prob_and_eq("A3s", "KK+,AKs"),
            beta.ako_1,
            equitizer.query_prob_and_eq("A3s", "AKo"),
            beta.qq_2,
            equitizer.query_prob_and_eq("A3s", "QQ"),
            s,
        )
    };

    binary_search(130.into(), 177.into(), f)
}

pub struct Alpha18 {
    pub tt_1: f64,
    pub a3s_2: f64,
}

impl fmt::Display for Alpha18 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TT:{},A3s:{}",
            pretty_percent(self.tt_1),
            pretty_percent(self.a3s_2)
        )
    }
}

pub fn calc_alpha18(equitizer: &mut Equitizer, s: S) -> Alpha18 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("AKo", "QQ+,AK,AQs,ATs,A5s,A4s");
    let p_and_eq_1 = equitizer.query_prob_and_eq("AKo", "TT");
    let p_and_eq_2 = equitizer.query_prob_and_eq("AKo", "A3s");
    let p_and_eq_3 = equitizer.query_prob_and_eq("QQ", "QQ+,AK,AQs,ATs,A5s,A4s");
    let p_and_eq_4 = equitizer.query_prob_and_eq("QQ", "TT");
    let p_and_eq_5 = equitizer.query_prob_and_eq("QQ", "A3s");

    let (tt, a3s) = aux::calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha18 {
        tt_1: tt,
        a3s_2: a3s,
    }
}

pub fn calc_beta18(equitizer: &mut Equitizer, s: S) -> BetaAKoQQ {
    let p_and_eq_0 = equitizer.query_prob_and_eq("TT", "KK+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("TT", "QQ");
    let p_and_eq_3 = equitizer.query_prob_and_eq("A3s", "KK+,AKs");
    let p_and_eq_4 = equitizer.query_prob_and_eq("A3s", "AKo");
    let p_and_eq_5 = equitizer.query_prob_and_eq("A3s", "QQ");

    let (ako_1, qq_2) = aux::calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    BetaAKoQQ { ako_1, qq_2 }
}
