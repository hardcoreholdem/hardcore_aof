use crate::calc_alpha::calc_alpha_1d;
use crate::calc_attacker_ev::calc_attacker_ev_1d;
use crate::research_attacker::research_attacker_1d;
use crate::research_defender::research_defender_1d;
use crate::section01::calc_alpha1;
use crate::section01::calc_beta1;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
use hardcore_equitizer::Range;
use std::fmt::Debug;
use std::fmt::Display;

pub fn section02(equitizer: &mut Equitizer) {
    let full_range = PureRange::from("99+,AJs+,AQo+");

    let s2 = search_s2_for_atk_eq_of_aks_equals_0(equitizer, &full_range);

    for s in [25.6.into(), 25.into(), s2, 22.1.into()] {
        println!("s={}", s);

        let alpha = calc_alpha1(equitizer, s);
        println!("{:?}", alpha);

        let beta = calc_beta1(&full_range, equitizer, s);
        println!("{:?}", beta);

        research_attacker_1d(&full_range, equitizer, "AA", "AKs", beta.aks_1, s, 5);

        research_defender_1d(equitizer, "AA,A5s", (alpha.a4s, "A4s"), &full_range, s, 5);

        println!("");
    }

    // println!("s1={}", s1);
    println!("");

    let alpha = calc_alpha2(equitizer, s2);
    println!("{}", alpha);

    let beta = calc_beta2(&full_range, equitizer, s2);
    println!("{}", beta);
}

pub fn search_s2_for_atk_eq_of_aks_equals_0(
    equitizer: &mut Equitizer,
    full_range: &impl Range,
) -> S {
    let f = |s| -> f64 {
        let (p_0, eq_0) = equitizer.query_sub_prob_and_eq(
            &PureRange::from("AKs"),
            &PureRange::from("AA"),
            full_range,
        );
        let (p_1, eq_1) = equitizer.query_sub_prob_and_eq(
            &PureRange::from("AKs"),
            &PureRange::from("AKs"),
            full_range,
        );
        let beta = calc_beta1(full_range, equitizer, s);
        calc_attacker_ev_1d((p_0, eq_0), (beta.aks_1, p_1, eq_1), s)
    };
    binary_search(20.into(), 30.into(), f)
}

pub struct Alpha2 {
    pub a4s: f64,
}

impl Display for Alpha2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A4s={}", pretty_percent(self.a4s))
    }
}

impl Debug for Alpha2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AA,AKs,A5s,{}", self)
    }
}

pub fn calc_alpha2(equitizer: &mut Equitizer, s: S) -> Alpha2 {
    let (p0, eq0) =
        equitizer.query_prob_and_eq(&PureRange::from("AKs"), &PureRange::from("AA,AKs,A5s"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("AKs"), &PureRange::from("A4s"));

    let a4s_1 = calc_alpha_1d((p0, eq0), (p1, eq1), s);

    Alpha2 { a4s: a4s_1 }
}

pub struct Beta2 {
    pub aks_1: f64,
}

impl Display for Beta2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AKs={}", pretty_percent(self.aks_1))
    }
}

impl Debug for Beta2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AA,{}", self)
    }
}

pub fn calc_beta2(full_range: &impl Range, equitizer: &mut Equitizer, s: S) -> Beta2 {
    let beta1 = calc_beta1(full_range, equitizer, s);

    Beta2 { aks_1: beta1.aks_1 }
}
