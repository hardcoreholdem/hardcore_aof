use crate::section13::calc_alpha13;
use crate::section13::calc_beta13;
use hardcore_aof::aux;
use hardcore_aof::aux::calc_alpha_3d;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
use std::fmt;

pub fn section14(equitizer: &mut Equitizer) {
    let s14 = search_s14_for_ev_qq_equals_0(equitizer);
    println!("s14={}", s14);

    let alpha13 = calc_alpha13(equitizer, s14);
    println!("alpha_A3s={}", pretty_percent(alpha13.a3s));
    println!("alpha_TT={}", pretty_percent(alpha13.tt));
    println!("");

    let beta = calc_beta13(equitizer, s14);
    println!("beta_AKo={}", pretty_percent(beta.ako_1));
    println!("beta_JJ={}", pretty_percent(beta.jj_2));
    println!("");

    let alpha = calc_alpha14(equitizer, s14);
    println!("alpha={}", alpha);
    println!("");

    let beta14 = calc_beta14(equitizer, s14);
    println!("beta_AKo={}", pretty_percent(beta14.ako));
    println!("beta_JJ={}", pretty_percent(beta14.jj));
    println!("beta_QQ={}", pretty_percent(beta14.qq));
    println!("");
}

fn search_s14_for_ev_qq_equals_0(equitizer: &mut Equitizer) -> S {
    let (p_0, eq_0) =
        equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("KK+AKs"));
    let (p_1, eq_1) = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("AKo"));
    let (p_2, eq_2) = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("JJ"));

    let f = |s| {
        let beta = calc_beta13(equitizer, s);
        aux::calc_attacker_ev_2d(
            (p_0, eq_0),
            (beta.ako_1, p_1, eq_1),
            (beta.jj_2, p_2, eq_2),
            s,
        )
    };

    binary_search(0.into(), 300.into(), f)
}

pub struct Alpha14 {
    pub a3s: f64,
    pub tt: f64,
    pub qq: f64,
}

impl fmt::Display for Alpha14 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A3s:{},TT:{},QQ:{}",
            pretty_percent(self.a3s),
            pretty_percent(self.tt),
            pretty_percent(self.qq)
        )
    }
}

pub fn calc_alpha14(equitizer: &mut Equitizer, s: S) -> Alpha14 {
    let p_and_eq_00 =
        equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("KK+AKs"));
    let p_and_eq_01 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("A3s"));
    let p_and_eq_02 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("TT"));
    let p_and_eq_03 = equitizer.query_prob_and_eq(&PureRange::from("AKo"), &PureRange::from("QQ"));
    let p_and_eq_10 =
        equitizer.query_prob_and_eq(&PureRange::from("JJ"), &PureRange::from("KK+AKs"));
    let p_and_eq_11 = equitizer.query_prob_and_eq(&PureRange::from("JJ"), &PureRange::from("A3s"));
    let p_and_eq_12 = equitizer.query_prob_and_eq(&PureRange::from("JJ"), &PureRange::from("TT"));
    let p_and_eq_13 = equitizer.query_prob_and_eq(&PureRange::from("JJ"), &PureRange::from("QQ"));
    let p_and_eq_20 =
        equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("KK+AKs"));
    let p_and_eq_21 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("A3s"));
    let p_and_eq_22 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("TT"));
    let p_and_eq_23 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("QQ"));

    let (a3s, tt, qq) = calc_alpha_3d(
        (p_and_eq_00, p_and_eq_01, p_and_eq_02, p_and_eq_03),
        (p_and_eq_10, p_and_eq_11, p_and_eq_12, p_and_eq_13),
        (p_and_eq_20, p_and_eq_21, p_and_eq_22, p_and_eq_23),
        s,
    );

    Alpha14 { a3s, tt, qq }
}

pub struct Beta14 {
    pub ako: f64,
    pub jj: f64,
    pub qq: f64,
}

impl fmt::Display for Beta14 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AKo:{},JJ:{},QQ:{}",
            pretty_percent(self.ako),
            pretty_percent(self.jj),
            pretty_percent(self.qq)
        )
    }
}

pub fn calc_beta14(equitizer: &mut Equitizer, s: S) -> Beta14 {
    let p_and_eq_00 =
        equitizer.query_prob_and_eq(&PureRange::from("A3s"), &PureRange::from("KK+AKs"));
    let p_and_eq_01 = equitizer.query_prob_and_eq(&PureRange::from("A3s"), &PureRange::from("AKo"));
    let p_and_eq_02 = equitizer.query_prob_and_eq(&PureRange::from("A3s"), &PureRange::from("JJ"));
    let p_and_eq_03 = equitizer.query_prob_and_eq(&PureRange::from("A3s"), &PureRange::from("QQ"));
    let p_and_eq_10 =
        equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("KK+AKs"));
    let p_and_eq_11 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("AKo"));
    let p_and_eq_12 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("JJ"));
    let p_and_eq_13 = equitizer.query_prob_and_eq(&PureRange::from("TT"), &PureRange::from("QQ"));
    let p_and_eq_20 =
        equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("KK+AKs"));
    let p_and_eq_21 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("AKo"));
    let p_and_eq_22 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("JJ"));
    let p_and_eq_23 = equitizer.query_prob_and_eq(&PureRange::from("QQ"), &PureRange::from("QQ"));

    let (ako, jj, qq) = aux::calc_beta_3d(
        (p_and_eq_00, p_and_eq_01, p_and_eq_02, p_and_eq_03),
        (p_and_eq_10, p_and_eq_11, p_and_eq_12, p_and_eq_13),
        (p_and_eq_20, p_and_eq_21, p_and_eq_22, p_and_eq_23),
        s,
    );

    Beta14 { ako, jj, qq }
}
