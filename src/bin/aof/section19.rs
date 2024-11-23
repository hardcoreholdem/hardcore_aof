use hardcore_equitizer::Equitizer;

use crate::calc_alpha::calc_alpha_2d;
use crate::calc_attacker_ev::calc_attacker_ev_2d;
use crate::research_attacker::research_attacker_2d;
use crate::research_defender::research_defender_2d;
use crate::section18::calc_alpha18;
use crate::section18::calc_beta18;
use hardcore_aof::aux;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::BetaAKoQQ;
use hardcore_aof::types::S;
use hardcore_equitizer::PureRange;
use std::fmt;

pub fn section19(equitizer: &mut Equitizer) {
    let s = 170.into();
    let alpha = calc_alpha18(equitizer, s);
    let beta = calc_beta18(equitizer, s);

    println!("alpha18 = {}", alpha);
    println!("beta18 = {}", beta);

    println!("");

    research_attacker_2d(
        equitizer,
        "KK+,AKs",
        (beta.ako_1, "AKo"),
        (beta.qq_2, "QQ"),
        s,
        15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AQs+,ATs,A5s,A4s,AKo",
        (alpha.tt_1, "TT"),
        (alpha.a3s_2, "A3s"),
        s,
        10,
    );

    for s in [
        170.into(),
        160.into(),
        150.into(),
        140.into(),
        130.into(),
        120.into(),
    ] {
        let alpha = calc_alpha18(equitizer, s);
        let beta = calc_beta18(equitizer, s);
        println!("s = {}, alpha = {}, beta = {}", s, alpha, beta);
    }
    println!("");

    let s = search_s_for_beta18_qq_equals_1(equitizer);
    let alpha = calc_alpha18(equitizer, s);
    let beta = calc_beta18(equitizer, s);

    println!("s = {}", s);
    println!("alpha(s) = {}", alpha);
    println!("beta(s) = {}", beta);
    println!("");

    research_attacker_2d(
        equitizer,
        "KK+,AKs",
        (beta.ako_1, "AKo"),
        (beta.qq_2, "QQ"),
        s,
        20,
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

    let s19 = search_s19_for_atk_ev_of_ats_equals_0(equitizer);
    let alpha = calc_alpha18(equitizer, s19);
    let beta = calc_beta18(equitizer, s19);

    println!("s19 = {}", s19);
    println!("alpha19(s19) = {}", alpha);
    println!("beta19(s19) = {}", beta);
    println!("");

    research_attacker_2d(
        equitizer,
        "KK+,AKs",
        (beta.ako_1, "AKo"),
        (beta.qq_2, "QQ"),
        s19,
        15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AQs+,ATs,A5s,A4s,AKo",
        (alpha.tt_1, "TT"),
        (alpha.a3s_2, "A3s"),
        s19,
        10,
    );

    let alpha19 = calc_alpha19(equitizer, s19);
    let beta19 = calc_beta19(equitizer, s19);
    println!("alpha19(s19) = {}", alpha19);
    println!("beta19(s19) = {}", beta19);
}

fn search_s_for_beta18_qq_equals_1(equitizer: &mut Equitizer) -> S {
    let f = |s| calc_beta18(equitizer, s).qq_2 - 1.0;
    binary_search(120.into(), 130.into(), f)
}

fn search_s19_for_atk_ev_of_ats_equals_0(equitizer: &mut Equitizer) -> S {
    let f = |s| -> f64 {
        let (p_0, eq_0) =
            equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("KK+,AKs"));
        let (p_1, eq_1) =
            equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("AKo"));
        let (p_2, eq_2) =
            equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("QQ"));
        let beta = calc_beta18(equitizer, s);

        calc_attacker_ev_2d(
            (p_0, eq_0),
            (beta.ako_1, p_1, eq_1),
            (beta.qq_2, p_2, eq_2),
            s,
        )
    };

    binary_search(120.into(), 170.into(), f)
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

pub fn calc_alpha19(equitizer: &mut Equitizer, s: S) -> Alpha19 {
    let p_and_eq_0 = equitizer.query_prob_and_eq(
        &PureRange::from("AKo"),
        &PureRange::from("QQ+,AQs+,A5s-A3s,AKo"),
    );
    let p_and_eq_1 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("TT"));
    let p_and_eq_2 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("ATs"));
    let p_and_eq_3 = equitizer.query_prob_and_eq(
        &PureRange::from("QQ"),
        &PureRange::from("QQ+,AQs+,A5s-A3s,AKo"),
    );
    let p_and_eq_4 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("TT"));
    let p_and_eq_5 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("ATs"));

    let (tt_1, ats_2) = calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha19 { tt_1, ats_2 }
}

pub fn calc_beta19(equitizer: &mut Equitizer, s: S) -> BetaAKoQQ {
    let p_and_eq_0 =
        equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("KK+,AKs"));
    let p_and_eq_1 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("AKo"));
    let p_and_eq_2 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("QQ"));
    let p_and_eq_3 =
        equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("KK+,AKs"));
    let p_and_eq_4 = equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("AKo"));
    let p_and_eq_5 = equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("QQ"));

    let (ako_1, qq_2) = aux::calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    BetaAKoQQ { ako_1, qq_2 }
}
