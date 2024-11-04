use crate::aux;
use crate::aux::calc_attacker_ev_2d;
use crate::format::pretty_percent;
use crate::format::pretty_s;
use crate::research_attacker::research_attacker_2d;
use crate::research_defender::research_defender_2d;
use crate::search::binary_search;
use crate::section16::calc_alpha16;
use crate::section16::calc_beta16;
use crate::types::BetaAKoQQ;
use hardcore_equitizer::Equitizer;
use std::fmt;

pub fn section17(equitizer: &mut Equitizer) {
    let s = 177.0;
    let alpha = calc_alpha16(equitizer, s);
    let beta = calc_beta16(equitizer, s);
    println!("s = {}, alpha = {}, beta = {}", pretty_s(s), alpha, beta);

    research_attacker_2d(
        equitizer,
        "KK+,AKs",
        "AKo",
        beta.v1_ako,
        "QQ",
        beta.v2_qq,
        s,
        15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AK,ATs,A5s-A3s",
        "TT",
        alpha.tt,
        "AJs",
        alpha.ajs,
        s,
        10,
    );

    for s in [
        177.0, 176.0, 175.0, 170.0, 165.0, 160.0, 150.0, 140.0, 130.0,
    ] {
        let alpha = calc_alpha16(equitizer, s);
        let beta = calc_beta16(equitizer, s);
        println!("s = {}, alpha = {}, beta = {}", pretty_s(s), alpha, beta);
    }
    println!("");

    let s = search_s_for_alpha16_ajs_ev_equals_0(equitizer);
    let alpha = calc_alpha16(equitizer, s);
    let beta = calc_beta16(equitizer, s);

    println!("s = {}", pretty_s(s));
    println!("alpha(s) = {}", alpha);
    println!("beta(s) = {}", beta);

    research_attacker_2d(
        equitizer,
        "KK+,AKs",
        "AKo",
        beta.v1_ako,
        "QQ",
        beta.v2_qq,
        s,
        15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AK,ATs,A5s-A3s",
        "TT",
        alpha.tt,
        "AJs",
        alpha.ajs,
        s,
        10,
    );

    let s = search_s_for_attacker_ev_of_aqs_equals_0(equitizer);
    let alpha = calc_alpha16(equitizer, s);
    let beta = calc_beta16(equitizer, s);

    println!("s = {}", pretty_s(s));
    println!("alpha(s) = {}", alpha);
    println!("beta(s) = {}", beta);
    println!("");

    research_attacker_2d(
        equitizer,
        "KK+,AKs",
        "AKo",
        beta.v1_ako,
        "QQ",
        beta.v2_qq,
        s,
        15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AK,ATs,A5s-A3s",
        "TT",
        alpha.tt,
        "AJs",
        alpha.ajs,
        s,
        10,
    );

    let alpha17 = calc_alpha17(equitizer, s);
    println!("alpha17(s17) = {}", alpha17);
    println!("beta17(s17) = {}", calc_beta17(equitizer, s));
}

fn search_s_for_alpha16_ajs_ev_equals_0(equitizer: &mut Equitizer) -> f64 {
    let f = |s: f64| calc_alpha16(equitizer, s).ajs;
    binary_search(130.0, 140.0, f)
}

fn search_s_for_attacker_ev_of_aqs_equals_0(equitizer: &mut Equitizer) -> f64 {
    let f = |s: f64| -> f64 {
        let beta = calc_beta16(equitizer, s);
        calc_attacker_ev_2d(
            equitizer.query_prob_and_eq("AQs", "KK+,AKs"),
            equitizer.query_prob_and_eq("AQs", "AKo"),
            beta.v1_ako,
            equitizer.query_prob_and_eq("AQs", "QQ"),
            beta.v2_qq,
            s,
        )
    };

    binary_search(130.0, 177.0, f)
}

pub struct Alpha17 {
    pub tt: f64,
    pub aqs: f64,
}

impl fmt::Display for Alpha17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TT:{},AQs:{}",
            pretty_percent(self.tt),
            pretty_percent(self.aqs)
        )
    }
}

pub fn calc_alpha17(equitizer: &mut Equitizer, s: f64) -> Alpha17 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("AKo", "QQ+,AK,ATs,A5s-A3s");
    let p_and_eq_1 = equitizer.query_prob_and_eq("AKo", "TT");
    let p_and_eq_2 = equitizer.query_prob_and_eq("AKo", "AQs");
    let p_and_eq_3 = equitizer.query_prob_and_eq("QQ", "QQ+,AK,ATs,A5s-A3s");
    let p_and_eq_4 = equitizer.query_prob_and_eq("QQ", "TT");
    let p_and_eq_5 = equitizer.query_prob_and_eq("QQ", "AQs");

    let (tt, aqs) = aux::calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha17 { tt, aqs }
}

pub fn calc_beta17(equitizer: &mut Equitizer, s: f64) -> BetaAKoQQ {
    let p_and_eq_0 = equitizer.query_prob_and_eq("TT", "KK+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("TT", "QQ");
    let p_and_eq_3 = equitizer.query_prob_and_eq("AQs", "KK+,AKs");
    let p_and_eq_4 = equitizer.query_prob_and_eq("AQs", "AKo");
    let p_and_eq_5 = equitizer.query_prob_and_eq("AQs", "QQ");

    let (ako_1, qq_2) = aux::calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    BetaAKoQQ {
        v1_ako: ako_1,
        v2_qq: qq_2,
    }
}
