use hardcore_equitizer::Equitizer;

use crate::section20::calc_alpha20;
use crate::section20::calc_beta20;
use core::f64;
use hardcore_aof::aux;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::research_attacker::research_attacker_2d;
use hardcore_aof::research_defender::research_defender_2d;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::BetaAKoJJ;
use hardcore_aof::types::S;
use std::fmt;

pub fn section21(equitizer: &mut Equitizer) {
    let s_neighbour = 122.into();
    let alpha_neighbour = calc_alpha20(equitizer, s_neighbour);
    let beta_neighbour = calc_beta20(equitizer, s_neighbour);

    println!("α20({}) = {}", s_neighbour, alpha_neighbour);
    println!("β20({}) = {}", s_neighbour, beta_neighbour);
    println!("");

    research_attacker_2d(
        equitizer,
        "QQ+,AKs",
        (beta_neighbour.ako_1, "AKo"),
        (beta_neighbour.jj_2, "JJ"),
        s_neighbour,
        15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AQs+,A5s-A3s,AKo",
        (alpha_neighbour.tt_1, "TT"),
        (alpha_neighbour.ats_2, "ATs"),
        s_neighbour,
        10,
    );

    let s21 = search_s21_for_attacker_ev_of_jj_equals_0(equitizer);
    let alpha20_s21 = calc_alpha20(equitizer, s21);
    let beta20_s21 = calc_beta20(equitizer, s21);
    println!("s21 = {}", s21);
    println!("alpha21(s21) = {}", alpha20_s21);
    println!("beta21(s21) = {}", beta20_s21);

    research_attacker_2d(
        equitizer,
        "QQ+,AKs",
        (beta20_s21.jj_2, "JJ"),
        (beta20_s21.ako_1, "AKo"),
        s21,
        15,
    );

    research_defender_2d(
        equitizer,
        "QQ+,AQs+,A5s-A3s,AKo",
        (alpha20_s21.tt_1, "TT"),
        (alpha20_s21.ats_2, "ATs"),
        s21,
        10,
    );

    let alpha21 = calc_alpha21(equitizer, s21);
    println!("alpha21(s21) = {}", alpha21);
    //    println!("beta21(s21) = {}", beta21);

    let beta21 = calc_beta21(equitizer, s21);
    println!("beta21(s21) = {}", beta21);
}

fn search_s21_for_attacker_ev_of_jj_equals_0(equitizer: &mut Equitizer) -> S {
    let f = |s| -> f64 {
        let (p_0, eq_0) = equitizer.query_prob_and_eq("JJ", "QQ+,AKs");
        let (p_1, eq_1) = equitizer.query_prob_and_eq("JJ", "AKo");
        let (p_2, eq_2) = equitizer.query_prob_and_eq("JJ", "JJ");
        let beta = calc_beta20(equitizer, s);
        aux::calc_attacker_ev_2d(
            (p_0, eq_0),
            (beta.ako_1, p_1, eq_1),
            (beta.jj_2, p_2, eq_2),
            s,
        )
    };

    binary_search(100.into(), 123.into(), f)
}

// pub fn section21(equitizer: &mut Equitizer) {

//     return;

//     // let s20 = search_s20_for_beta19_qq_equals_1(equitizer);
//     // let alpha_s19 = calc_alpha19(equitizer, s20);
//     // let beta_s19 = calc_beta19(equitizer, s20);

//     // for s in [125.0, s20, 120.0] {
//     //     let alpha = calc_alpha19(equitizer, s);
//     //     let beta = calc_beta19(equitizer, s);
//     //     println!("s = {}, alpha = {}, beta = {}", pretty_s(s), alpha, beta);
//     // }
//     // println!("");

//     // println!("s20 = {}", pretty_s(s20));

//     // assert!(f64::abs(beta_s19.qq_2 - 1.0) < 1e-9);
//     // research_attacker_1d(equitizer, "QQ+,AKs", "AKo", beta_s19.ako_1, s20, 20);

//     // research_defender_2d(
//     //     equitizer,
//     //     "QQ+,AQs+,A5s-A3s,AKo",
//     //     "TT",
//     //     alpha_s19.tt_1,
//     //     "ATs",
//     //     alpha_s19.ats_2,
//     //     s20,
//     //     10,
//     // );

//     // research_defender_1d(
//     //     equitizer,
//     //     "QQ+,AQs+,ATs,A5s-A3s,AKo",
//     //     alpha_s19.tt_1,
//     //     "TT",
//     //     s20,
//     //     10,
//     // );

//     // research_defender_1d(
//     //     equitizer,
//     //     "QQ+,TT,AQs+,A5s-A3s,AKo",
//     //     alpha_s19.ats_2,
//     //     "ATs",
//     //     s20,
//     //     10,
//     // );

//     // let alpha20 = calc_alpha20(equitizer, s20);
//     // println!("alpha20(s20) = {}", alpha20);

//     // let beta20 = calc_beta20(equitizer, s20);
//     // println!("beta20(s20) = {}", beta20);
// }

// fn search_s20_for_beta19_qq_equals_1(equitizer: &mut Equitizer) -> f64 {
//     let f = |s: f64| calc_beta19(equitizer, s).qq_2 - 1.0;
//     binary_search(120.0, 130.0, f)
// }

pub struct Alpha21 {
    pub tt_1: f64,
    pub jj_2: f64,
}

impl fmt::Display for Alpha21 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TT:{},JJ:{}",
            pretty_percent(self.tt_1),
            pretty_percent(self.jj_2),
        )
    }
}

pub fn calc_alpha21(equitizer: &mut Equitizer, s: S) -> Alpha21 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("AKo", "QQ+,AQs+,ATs,A5s-A3s,AKo");
    let p_and_eq_1 = equitizer.query_prob_and_eq("AKo", "TT");
    let p_and_eq_2 = equitizer.query_prob_and_eq("AKo", "JJ");
    let p_and_eq_3 = equitizer.query_prob_and_eq("JJ", "QQ+,AQs+,ATs,A5s-A3s,AKo");
    let p_and_eq_4 = equitizer.query_prob_and_eq("JJ", "TT");
    let p_and_eq_5 = equitizer.query_prob_and_eq("JJ", "JJ");

    let (tt_1, jj_2) = aux::calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha21 { tt_1, jj_2 }
}

pub fn calc_beta21(equitizer: &mut Equitizer, s: S) -> BetaAKoJJ {
    let p_and_eq_0 = equitizer.query_prob_and_eq("TT", "QQ+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("TT", "JJ");
    let p_and_eq_3 = equitizer.query_prob_and_eq("JJ", "QQ+,AKs");
    let p_and_eq_4 = equitizer.query_prob_and_eq("JJ", "AKo");
    let p_and_eq_5 = equitizer.query_prob_and_eq("JJ", "JJ");

    let (ako_1, jj_2) = aux::calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    BetaAKoJJ { ako_1, jj_2 }
}
