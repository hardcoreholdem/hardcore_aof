use crate::calc_alpha::calc_alpha_1d;
use crate::calc_attacker_ev::calc_attacker_ev_0d;
use crate::calc_beta::calc_beta_1d;
use crate::research_attacker::research_attacker_0d;
use crate::research_defender::research_defender_0d;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
use hardcore_equitizer::Range;
use std::fmt::Debug;
use std::fmt::Display;

pub fn section01(equitizer: &mut Equitizer) {
    let full_range = PureRange::from("99+,AJs+,AQo+");

    let s1 = search_s1_for_atk_eq_of_a4s_equals_0(equitizer, &full_range);
    for s in [25.8.into(), 25.7.into(), s1] {
        research_attacker_0d(equitizer, "AA", &full_range, s, 5);
    }

    println!("s1={}", s1);
    println!("");

    research_defender_0d(equitizer, "AA,A5s", &full_range, s1, 5);
    research_defender_0d(equitizer, "AA,A5s,A4s", &full_range, s1, 5);

    let alpha = calc_alpha1(equitizer, s1);
    println!("{}", alpha);

    let beta = calc_beta1(&full_range, equitizer, s1);
    println!("{}", beta);
}

pub fn search_s1_for_atk_eq_of_a4s_equals_0(
    equitizer: &mut Equitizer,
    full_range: &impl Range,
) -> S {
    let f = |s| -> f64 {
        let (p_0, eq_0) = equitizer.query_sub_prob_and_eq(
            &PureRange::from("A4s"),
            &PureRange::from("AA"),
            full_range,
        );
        calc_attacker_ev_0d((p_0, eq_0), s)
    };

    binary_search(20.into(), 30.into(), f)
}

pub struct Alpha1 {
    pub a4s: f64,
}

impl Display for Alpha1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A4s={}", pretty_percent(self.a4s))
    }
}

impl Debug for Alpha1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AA,A5s,{}", self)
    }
}

pub fn calc_alpha1(equitizer: &mut Equitizer, s: S) -> Alpha1 {
    let p_and_eq_0 =
        equitizer.query_prob_and_eq(&PureRange::from("AKs"), &PureRange::from("AA,A5s"));
    let p_and_eq_1 = equitizer.query_prob_and_eq(&PureRange::from("AKs"), &PureRange::from("A4s"));

    let a4s_1 = calc_alpha_1d(p_and_eq_0, p_and_eq_1, s);

    Alpha1 { a4s: a4s_1 }
}

pub struct Beta1 {
    pub aks_1: f64,
}

impl Display for Beta1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AKs={}", pretty_percent(self.aks_1))
    }
}

impl Debug for Beta1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AA,{}", self)
    }
}

pub fn calc_beta1(full_range: &impl Range, equitizer: &mut Equitizer, s: S) -> Beta1 {
    let p_and_eq_0 = equitizer.query_sub_prob_and_eq(
        &PureRange::from("A4s"),
        &PureRange::from("AA"),
        full_range,
    );
    let p_and_eq_1 = equitizer.query_sub_prob_and_eq(
        &PureRange::from("A4s"),
        &PureRange::from("AKs"),
        full_range,
    );

    let aks_1 = calc_beta_1d(p_and_eq_0, p_and_eq_1, s);

    Beta1 { aks_1 }
}
