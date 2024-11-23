use crate::section16::calc_alpha16;
use crate::section16::calc_beta16;
use hardcore_aof::aux;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::research_attacker::research_attacker_2d;
use hardcore_aof::research_defender::research_defender_2d;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::BetaAKoQQ;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
use std::fmt;

pub fn section17(equitizer: &mut Equitizer) {
    let s = 177.into();
    let alpha = calc_alpha16(equitizer, s);
    let beta = calc_beta16(equitizer, s);
    println!("s = {}, alpha = {}, beta = {}", s, alpha, beta);

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
        "QQ+,AK,ATs,A5s-A3s",
        (alpha.tt, "TT"),
        (alpha.ajs, "AJs"),
        s,
        10,
    );

    for s in [
        177.into(),
        176.into(),
        175.into(),
        170.into(),
        165.into(),
        160.into(),
        150.into(),
        140.into(),
        130.into(),
    ] {
        let alpha = calc_alpha16(equitizer, s);
        let beta = calc_beta16(equitizer, s);
        println!("s = {}, alpha = {}, beta = {}", s, alpha, beta);
    }
    println!("");

    let s = search_s_for_alpha16_ajs_ev_equals_0(equitizer);
    let alpha = calc_alpha16(equitizer, s);
    let beta = calc_beta16(equitizer, s);

    println!("s = {}", s);
    println!("alpha(s) = {}", alpha);
    println!("beta(s) = {}", beta);

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
        "QQ+,AK,ATs,A5s-A3s",
        (alpha.tt, "TT"),
        (alpha.ajs, "AJs"),
        s,
        10,
    );

    let s = search_s_for_attacker_ev_of_aqs_equals_0(equitizer);
    let alpha = calc_alpha16(equitizer, s);
    let beta = calc_beta16(equitizer, s);

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
        15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AK,ATs,A5s-A3s",
        (alpha.tt, "TT"),
        (alpha.ajs, "AJs"),
        s,
        10,
    );

    let alpha17 = calc_alpha17(equitizer, s);
    println!("alpha17(s17) = {}", alpha17);
    println!("beta17(s17) = {}", calc_beta17(equitizer, s));
}

fn search_s_for_alpha16_ajs_ev_equals_0(equitizer: &mut Equitizer) -> S {
    let f = |s| calc_alpha16(equitizer, s).ajs;
    binary_search(130.into(), 140.into(), f)
}

fn search_s_for_attacker_ev_of_aqs_equals_0(equitizer: &mut Equitizer) -> S {
    let f = |s| -> f64 {
        let beta = calc_beta16(equitizer, s);
        let (p_0, eq_0) =
            equitizer.query_prob_and_eq(&PureRange::from("AQs"), &PureRange::from("KK+,AKs"));
        let (p_1, eq_1) =
            equitizer.query_prob_and_eq(&PureRange::from("AQs"), &PureRange::from("AKo"));
        let (p_2, eq_2) =
            equitizer.query_prob_and_eq(&PureRange::from("AQs"), &PureRange::from("QQ"));
        aux::calc_attacker_ev_2d(
            (p_0, eq_0),
            (beta.ako_1, p_1, eq_1),
            (beta.qq_2, p_2, eq_2),
            s,
        )
    };

    binary_search(130.into(), 177.into(), f)
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

pub fn calc_alpha17(equitizer: &mut Equitizer, s: S) -> Alpha17 {
    let p_and_eq_0 = equitizer.query_prob_and_eq(
        &PureRange::from("AKo"),
        &PureRange::from("QQ+,AK,ATs,A5s-A3s"),
    );
    let p_and_eq_1 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("TT"));
    let p_and_eq_2 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("AQs"));
    let p_and_eq_3 = equitizer.query_prob_and_eq(
        &PureRange::from("QQ"),
        &PureRange::from("QQ+,AK,ATs,A5s-A3s"),
    );
    let p_and_eq_4 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("TT"));
    let p_and_eq_5 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("AQs"));

    let (tt, aqs) = aux::calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha17 { tt, aqs }
}

pub fn calc_beta17(equitizer: &mut Equitizer, s: S) -> BetaAKoQQ {
    let p_and_eq_0 =
        equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("KK+,AKs"));
    let p_and_eq_1 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("AKo"));
    let p_and_eq_2 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("QQ"));
    let p_and_eq_3 =
        equitizer.query_prob_and_eq(&PureRange::from("AQs"), &PureRange::from("KK+,AKs"));
    let p_and_eq_4 = equitizer.query_prob_and_eq(&PureRange::from("AQs"), &PureRange::from("AKo"));
    let p_and_eq_5 = equitizer.query_prob_and_eq(&PureRange::from("AQs"), &PureRange::from("QQ"));

    let (ako_1, qq_2) = aux::calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    BetaAKoQQ { ako_1, qq_2 }
}
