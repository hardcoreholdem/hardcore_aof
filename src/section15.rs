use crate::aux::calc_alpha_2d;
use crate::aux::calc_attacker_ev_2d;
use crate::aux::calc_beta_2d;
use crate::aux::calc_eq_2d;
use crate::aux::calc_eq_3d;
use crate::combos;
use crate::format::pretty_percent;
use crate::format::pretty_s;
use crate::search::binary_search;
use crate::section14::calc_alpha14;
use crate::section14::calc_beta14;
use hardcore_equitizer::Equitizer;
use std::fmt;

pub fn section15(equitizer: &mut Equitizer) {
    for s in [180.0, 179.0, 178.0] {
        println!("s = {}", s);
        println!("alpha = {}", calc_alpha14(equitizer, s));
        println!("beta = {}", calc_beta14(equitizer, s));
        println!("");
    }

    let s15 = search_s15_for_beta_jj_equals_0(equitizer);
    println!("s15 = {}", pretty_s(s15));

    {
        let beta = calc_beta14(equitizer, s15);

        let defender_0 = "KK+,AKs";
        let defender_1 = "AKo";
        let defender_2 = "QQ";
        let beta_1 = beta.ako;
        let beta_2 = beta.qq;

        let mut combo_and_eq_and_ev_vec = Vec::new();

        for combo in combos::calc_all_combos() {
            let p_and_eq_0 = equitizer.query_prob_and_eq(&combo, defender_0);
            let p_and_eq_1 = equitizer.query_prob_and_eq(&combo, defender_1);
            let p_and_eq_2 = equitizer.query_prob_and_eq(&combo, defender_2);

            let eq = calc_eq_2d(p_and_eq_0, p_and_eq_1, p_and_eq_2, beta_1, beta_2);
            let ev = calc_attacker_ev_2d(p_and_eq_0, beta_1, p_and_eq_1, beta_2, p_and_eq_2, s15);

            combo_and_eq_and_ev_vec.push((combo, eq, ev));
        }

        combo_and_eq_and_ev_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        for (combo, eq, ev) in combo_and_eq_and_ev_vec.iter().take(15) {
            println!("{combo}, eq={}, ev={}", pretty_percent(*eq), pretty_s(*ev));
        }
        println!("");
    }

    {
        let alpha = calc_alpha14(equitizer, s15);

        let attacker_0 = "KK+,AK,ATs,A5s,A4s";
        let attacker_1 = "A3s";
        let attacker_2 = "TT";
        let attacker_3 = "QQ";
        let beta_1 = alpha.a3s;
        let beta_2 = alpha.tt;
        let beta_3 = alpha.qq;
        let mut combo_and_eq_vec = Vec::new();

        for combo in combos::calc_all_combos() {
            let p_and_eq_0 = equitizer.query_prob_and_eq(&combo, attacker_0);
            let p_and_eq_1 = equitizer.query_prob_and_eq(&combo, attacker_1);
            let p_and_eq_2 = equitizer.query_prob_and_eq(&combo, attacker_2);
            let p_and_eq_3 = equitizer.query_prob_and_eq(&combo, attacker_3);

            let eq = calc_eq_3d(
                p_and_eq_0, p_and_eq_1, p_and_eq_2, p_and_eq_3, beta_1, beta_2, beta_3,
            );

            combo_and_eq_vec.push((combo, eq));
        }

        combo_and_eq_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (combo, eq) in combo_and_eq_vec.iter().take(10) {
            println!("{combo}, eq={}", pretty_percent(*eq));
        }
    }

    let alpha = calc_alpha15(equitizer, s15);
    println!("alpha = {}", alpha);

    let beta = calc_beta15(equitizer, s15);
    println!("beta = {}", beta);
}

fn search_s15_for_beta_jj_equals_0(equitizer: &mut Equitizer) -> f64 {
    let f = |s| calc_beta14(equitizer, s).jj;
    binary_search(178.0, 180.0, f)
}

pub struct Alpha15 {
    pub tt: f64,
    pub qq: f64,
}

impl fmt::Display for Alpha15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TT:{},QQ:{}",
            pretty_percent(self.tt),
            pretty_percent(self.qq)
        )
    }
}

pub fn calc_alpha15(equitizer: &mut Equitizer, s: f64) -> Alpha15 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("AKo", "KK+,AK,ATs,A5s-A3s");
    let p_and_eq_1 = equitizer.query_prob_and_eq("AKo", "TT");
    let p_and_eq_2 = equitizer.query_prob_and_eq("AKo", "QQ");
    let p_and_eq_3 = equitizer.query_prob_and_eq("QQ", "KK+,AK,ATs,A5s-A3s");
    let p_and_eq_4 = equitizer.query_prob_and_eq("QQ", "TT");
    let p_and_eq_5 = equitizer.query_prob_and_eq("QQ", "QQ");

    let (tt, qq) = calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha15 { tt, qq }
}

pub struct Beta15 {
    pub ako: f64,
    pub qq: f64,
}

impl fmt::Display for Beta15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AKo:{},QQ:{}",
            pretty_percent(self.ako),
            pretty_percent(self.qq)
        )
    }
}

pub fn calc_beta15(equitizer: &mut Equitizer, s: f64) -> Beta15 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("TT", "KK+,AKs");
    let p_and_eq_1 = equitizer.query_prob_and_eq("TT", "AKo");
    let p_and_eq_2 = equitizer.query_prob_and_eq("TT", "QQ");
    let p_and_eq_3 = equitizer.query_prob_and_eq("QQ", "KK+,AKs");
    let p_and_eq_4 = equitizer.query_prob_and_eq("QQ", "AKo");
    let p_and_eq_5 = equitizer.query_prob_and_eq("QQ", "QQ");

    let (ako, qq) = calc_beta_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Beta15 { ako, qq }
}
