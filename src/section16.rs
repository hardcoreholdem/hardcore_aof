use crate::aux::calc_alpha_2d;
use crate::aux::calc_attacker_ev_2d;
use crate::aux::calc_beta_2d;
use crate::format::{pretty_percent, pretty_s};
use crate::research_attacker::research_attacker_2d;
use crate::research_defender::research_defender_2d;
use crate::search::binary_search;
use crate::section15::{calc_alpha15, calc_beta15};
use crate::types::BetaAKoQQ;
use hardcore_equitizer::Equitizer;
use std::fmt;

pub fn section16(equitizer: &mut Equitizer) {
    let s = 178.0;
    let alpha = calc_alpha15(equitizer, s);
    let beta = calc_beta15(equitizer, s);

    println!("alpha = {}", alpha);
    println!("beta = {}", beta);

    println!("");

    research_attacker_2d(equitizer, "KK+,AKs", "AKo", beta.ako, "QQ", beta.qq, s, 15);

    research_defender_2d(
        equitizer,
        "KK+,AK,ATs,A5s-A3s",
        "TT",
        alpha.tt,
        "QQ",
        alpha.qq,
        s,
        10,
    );

    for s in [178.0, 177.0, 175.0, 170.0, 165.0] {
        let alpha = calc_alpha15(equitizer, s);
        let beta = calc_beta15(equitizer, s);
        println!("s = {}, alpha = {}, beta = {}", pretty_s(s), alpha, beta);
    }

    let s16: f64 = search_s16_for_attacker_ev_eof_ajs_quals_0(equitizer);
    println!("s16 = {}", pretty_s(s16));
    println!("alpha(s16) = {}", calc_alpha15(equitizer, s16));
    println!("beta(s16) = {}", calc_beta15(equitizer, s16));
    println!("");

    let alpha16 = calc_alpha16(equitizer, s16);
    println!("alpha16(s16) = {}", alpha16);

    let beta16 = calc_beta16(equitizer, s16);
    println!("beta16(s16) = {}", beta16);
}

fn search_s16_for_attacker_ev_eof_ajs_quals_0(equitizer: &mut Equitizer) -> f64 {
    let f = |s: f64| -> f64 {
        let beta = calc_beta15(equitizer, s);
        calc_attacker_ev_2d(
            equitizer.query_prob_and_eq("AJs", "KK+,AKs"),
            equitizer.query_prob_and_eq("AJs", "AKo"),
            beta.ako,
            equitizer.query_prob_and_eq("AJs", "QQ"),
            beta.qq,
            s,
        )
    };

    binary_search(165.0, 178.0, f)
}

pub struct Alpha16 {
    pub tt: f64,
    pub ajs: f64,
}

impl fmt::Display for Alpha16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TT:{},AJs:{}",
            pretty_percent(self.tt),
            pretty_percent(self.ajs)
        )
    }
}

pub fn calc_alpha16(equitizer: &mut Equitizer, s: f64) -> Alpha16 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("AKo", "QQ+,AK,ATs,A5s-A3s");
    let p_and_eq_1 = equitizer.query_prob_and_eq("AKo", "TT");
    let p_and_eq_2 = equitizer.query_prob_and_eq("AKo", "AJs");
    let p_and_eq_3 = equitizer.query_prob_and_eq("QQ", "QQ+,AK,ATs,A5s-A3s");
    let p_and_eq_4 = equitizer.query_prob_and_eq("QQ", "TT");
    let p_and_eq_5 = equitizer.query_prob_and_eq("QQ", "AJs");

    let (tt, ajs) = calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha16 { tt, ajs }
}

pub fn calc_beta16(equitizer: &mut Equitizer, s: f64) -> BetaAKoQQ {
    let p_and_eq_0 = equitizer.query_prob_and_eq("TT", "KK+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("TT", "QQ");
    let p_and_eq_3 = equitizer.query_prob_and_eq("AJs", "KK+,AKs");
    let p_and_eq_4 = equitizer.query_prob_and_eq("AJs", "AKo");
    let p_and_eq_5 = equitizer.query_prob_and_eq("AJs", "QQ");

    let (ako_1, qq_2) = calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    BetaAKoQQ { ako_1, qq_2 }
}
