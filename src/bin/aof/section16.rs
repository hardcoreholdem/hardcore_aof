use crate::calc_alpha::calc_alpha_2d;
use crate::calc_attacker_ev::calc_attacker_ev_2d;
use crate::research_attacker::research_attacker_2d;
use crate::research_defender::research_defender_2d;
use crate::section15::{calc_alpha15, calc_beta15};
use hardcore_aof::aux;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::BetaAKoQQ;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
use std::fmt;

pub fn section16(equitizer: &mut Equitizer) {
    let s = 178.into();
    let alpha = calc_alpha15(equitizer, s);
    let beta = calc_beta15(equitizer, s);

    println!("alpha = {}", alpha);
    println!("beta = {}", beta);

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
        "KK+,AK,ATs,A5s-A3s",
        (alpha.tt, "TT"),
        (alpha.qq, "QQ"),
        s,
        10,
    );

    for s in [178.into(), 177.into(), 175.into(), 170.into(), 165.into()] {
        let alpha = calc_alpha15(equitizer, s);
        let beta = calc_beta15(equitizer, s);
        println!("s = {}, alpha = {}, beta = {}", s, alpha, beta);
    }

    let s16 = search_s16_for_attacker_ev_eof_ajs_quals_0(equitizer);
    println!("s16 = {}", s16);
    println!("alpha(s16) = {}", calc_alpha15(equitizer, s16));
    println!("beta(s16) = {}", calc_beta15(equitizer, s16));
    println!("");

    let alpha16 = calc_alpha16(equitizer, s16);
    println!("alpha16(s16) = {}", alpha16);

    let beta16 = calc_beta16(equitizer, s16);
    println!("beta16(s16) = {}", beta16);
}

fn search_s16_for_attacker_ev_eof_ajs_quals_0(equitizer: &mut Equitizer) -> S {
    let f = |s| -> f64 {
        let beta = calc_beta15(equitizer, s);
        let (p_0, eq_0) =
            equitizer.query_prob_and_eq(&PureRange::from("AJs"), &PureRange::from("KK+,AKs"));
        let (p_1, eq_1) =
            equitizer.query_prob_and_eq(&PureRange::from("AJs"), &PureRange::from("AKo"));
        let (p_2, eq_2) =
            equitizer.query_prob_and_eq(&PureRange::from("AJs"), &PureRange::from("QQ"));
        calc_attacker_ev_2d(
            (p_0, eq_0),
            (beta.ako_1, p_1, eq_1),
            (beta.qq_2, p_2, eq_2),
            s,
        )
    };

    binary_search(165.into(), 178.into(), f)
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

pub fn calc_alpha16(equitizer: &mut Equitizer, s: S) -> Alpha16 {
    let p_and_eq_0 = equitizer.query_prob_and_eq(
        &PureRange::from("AKo"),
        &PureRange::from("QQ+,AK,ATs,A5s-A3s"),
    );
    let p_and_eq_1 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("TT"));
    let p_and_eq_2 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("AJs"));
    let p_and_eq_3 = equitizer.query_prob_and_eq(
        &PureRange::from("QQ"),
        &PureRange::from("QQ+,AK,ATs,A5s-A3s"),
    );
    let p_and_eq_4 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("TT"));
    let p_and_eq_5 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("AJs"));

    let (tt, ajs) = calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha16 { tt, ajs }
}

pub fn calc_beta16(equitizer: &mut Equitizer, s: S) -> BetaAKoQQ {
    let p_and_eq_0 =
        equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("KK+,AKs"));
    let p_and_eq_1 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("AKo"));
    let p_and_eq_2 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("QQ"));
    let p_and_eq_3 =
        equitizer.query_prob_and_eq(&PureRange::from("AJs"), &PureRange::from("KK+,AKs"));
    let p_and_eq_4 = equitizer.query_prob_and_eq(&PureRange::from("AJs"), &PureRange::from("AKo"));
    let p_and_eq_5 = equitizer.query_prob_and_eq(&PureRange::from("AJs"), &PureRange::from("QQ"));

    let (ako_1, qq_2) = aux::calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    BetaAKoQQ { ako_1, qq_2 }
}
