use hardcore_equitizer::Equitizer;

use crate::calc_alpha::calc_alpha_2d;
use crate::calc_beta::calc_beta_2d;
use crate::research_attacker::research_attacker_1d;
use crate::research_attacker::research_attacker_2d;
use crate::research_defender::research_defender_2d;
use crate::section19::calc_alpha19;
use crate::section19::calc_beta19;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::BetaAKoJJ;
use hardcore_aof::types::S;
use hardcore_equitizer::PureRange;
use std::fmt;

pub fn section20(equitizer: &mut Equitizer) {
    let s_neighbour = 127.into();
    let alpha = calc_alpha19(equitizer, s_neighbour);
    let beta = calc_beta19(equitizer, s_neighbour);

    println!("α19({}) = {}", s_neighbour, alpha);
    println!("β19({}) = {}", s_neighbour, beta);
    println!("");

    research_attacker_2d(
        equitizer,
        "KK+,AKs",
        (beta.ako_1, "AKo"),
        (beta.qq_2, "QQ"),
        s_neighbour,
        15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AQs+,A5s-A3s,AKo",
        (alpha.tt_1, "TT"),
        (alpha.ats_2, "ATs"),
        s_neighbour,
        10,
    );

    let s20 = search_s20_for_beta19_qq_equals_1(equitizer);
    let alpha_s19 = calc_alpha19(equitizer, s20);
    let beta_s19 = calc_beta19(equitizer, s20);

    for s in [125.into(), s20, 120.into()] {
        let alpha = calc_alpha19(equitizer, s);
        let beta = calc_beta19(equitizer, s);
        println!("s = {}, alpha = {}, beta = {}", s, alpha, beta);
    }
    println!("");

    println!("s20 = {}", s20);

    assert!(f64::abs(beta_s19.qq_2 - 1.0) < 1e-9);
    research_attacker_1d(equitizer, "QQ+,AKs", "AKo", beta_s19.ako_1, s20, 20);

    research_defender_2d(
        equitizer,
        "QQ+,AQs+,A5s-A3s,AKo",
        (alpha_s19.tt_1, "TT"),
        (alpha_s19.ats_2, "ATs"),
        s20,
        10,
    );

    // research_defender_1d(
    //     equitizer,
    //     "QQ+,AQs+,ATs,A5s-A3s,AKo",
    //     alpha_s19.tt_1,
    //     "TT",
    //     s20,
    //     10,
    // );

    // research_defender_1d(
    //     equitizer,
    //     "QQ+,TT,AQs+,A5s-A3s,AKo",
    //     alpha_s19.ats_2,
    //     "ATs",
    //     s20,
    //     10,
    // );

    let alpha20 = calc_alpha20(equitizer, s20);
    println!("alpha20(s20) = {}", alpha20);

    let beta20 = calc_beta20(equitizer, s20);
    println!("beta20(s20) = {}", beta20);
}

fn search_s20_for_beta19_qq_equals_1(equitizer: &mut Equitizer) -> S {
    let f = |s| calc_beta19(equitizer, s).qq_2 - 1.0;
    binary_search(120.into(), 130.into(), f)
}

pub struct Alpha20 {
    pub tt_1: f64,
    pub ats_2: f64,
}

impl fmt::Display for Alpha20 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TT:{},ATs:{}",
            pretty_percent(self.tt_1),
            pretty_percent(self.ats_2)
        )
    }
}

pub fn calc_alpha20(equitizer: &mut Equitizer, s: S) -> Alpha20 {
    let p_and_eq_0 = equitizer.query_prob_and_eq(
        &PureRange::from("AKo"),
        &PureRange::from("QQ+,AQs+,A5s-A3s,AKo"),
    );
    let p_and_eq_1 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("TT"));
    let p_and_eq_2 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("ATs"));
    let p_and_eq_3 = equitizer.query_prob_and_eq(
        &PureRange::from("JJ"),
        &PureRange::from("QQ+,AQs+,A5s-A3s,AKo"),
    );
    let p_and_eq_4 = equitizer.query_prob_and_eq(&PureRange::from("JJ"), &PureRange::from("TT"));
    let p_and_eq_5 = equitizer.query_prob_and_eq(&PureRange::from("JJ"), &PureRange::from("ATs"));

    let (tt_1, ats_2) = calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha20 { tt_1, ats_2 }
}

pub fn calc_beta20(equitizer: &mut Equitizer, s: S) -> BetaAKoJJ {
    let p_and_eq_0 =
        equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("QQ+,AKs"));
    let p_and_eq_1 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("AKo"));
    let p_and_eq_2 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("JJ"));
    let p_and_eq_3 =
        equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("QQ+,AKs"));
    let p_and_eq_4 = equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("AKo"));
    let p_and_eq_5 = equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("JJ"));

    let (ako_1, jj_2) = calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    BetaAKoJJ { ako_1, jj_2 }
}
