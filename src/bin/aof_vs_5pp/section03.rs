use crate::calc_alpha::calc_alpha_2d;
use crate::calc_attacker_ev::calc_attacker_ev_1d;
use crate::calc_beta::calc_beta_2d;
use crate::research_attacker::research_attacker_1d;
use crate::research_defender::research_defender_0d;
use crate::research_defender::research_defender_1d;
use crate::section02::calc_alpha2;
use crate::section02::calc_beta2;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
use hardcore_equitizer::Range;
use std::fmt::Debug;
use std::fmt::Display;

pub fn section03(equitizer: &mut Equitizer) {
    let full_range = PureRange::from("99+,AJs+,AQo+");

    let s3 = search_s3_for_atk_eq_of_ako_equals_0(equitizer, &full_range);

    for s in [22.1.into(), 22.into(), 20.into(), s3.into(), 18.8.into()] {
        println!("s={}", s);

        let alpha = calc_alpha2(equitizer, s);
        println!("{:?}", alpha);

        let beta = calc_beta2(&full_range, equitizer, s);
        println!("{:?}", beta);

        research_attacker_1d(&full_range, equitizer, "AA", "AKs", beta.aks_1, s, 5);

        research_defender_1d(
            equitizer,
            "AA,AKs,A5s",
            (alpha.a4s, "A4s"),
            &full_range,
            s,
            4,
        );

        println!("");
    }

    research_defender_0d(equitizer, "AA,AK,A5s", &full_range, s3, 4);

    println!("");

    let alpha = calc_alpha3(equitizer, s3);
    println!("{:?}", alpha);

    let beta = calc_beta3(&full_range, equitizer, s3);
    println!("{:?}", beta);
}

pub fn search_s3_for_atk_eq_of_ako_equals_0(
    equitizer: &mut Equitizer,
    full_range: &impl Range,
) -> S {
    let f = |s| -> f64 {
        let (p_0, eq_0) = equitizer.query_sub_prob_and_eq(
            &PureRange::from("AKo"),
            &PureRange::from("AA"),
            full_range,
        );
        let (p_1, eq_1) = equitizer.query_sub_prob_and_eq(
            &PureRange::from("AKo"),
            &PureRange::from("AKs"),
            full_range,
        );
        let beta = calc_beta2(full_range, equitizer, s);
        calc_attacker_ev_1d((p_0, eq_0), (beta.aks_1, p_1, eq_1), s)
    };
    binary_search(18.into(), 19.into(), f)
}

pub struct Alpha3 {
    a4s_1: f64,
    ako_2: f64,
}

impl Display for Alpha3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A4s={},AKo={}",
            pretty_percent(self.a4s_1),
            pretty_percent(self.ako_2)
        )
    }
}

impl Debug for Alpha3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AA,AKs,A5s,{}", self)
    }
}

pub fn calc_alpha3(equitizer: &mut Equitizer, s: S) -> Alpha3 {
    let (p0, eq0) =
        equitizer.query_prob_and_eq(&PureRange::from("AKs"), &PureRange::from("AA,AKs,A5s"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("AKs"), &PureRange::from("A4s"));
    let (p2, eq2) = equitizer.query_prob_and_eq(&PureRange::from("AKs"), &PureRange::from("AKs"));

    let (p3, eq3) =
        equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("AA,AKs,A5s"));
    let (p4, eq4) = equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("A4s"));
    let (p5, eq5) = equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("AKs"));

    let (a4s_1, ako_2) = calc_alpha_2d(
        ((p0, eq0), (p1, eq1), (p2, eq2)),
        ((p3, eq3), (p4, eq4), (p5, eq5)),
        s,
    );

    Alpha3 { a4s_1, ako_2 }
}

pub struct Beta3 {
    aks_1: f64,
    kk_2: f64,
}

impl Display for Beta3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AKs={},KK={}",
            pretty_percent(self.aks_1),
            pretty_percent(self.kk_2)
        )
    }
}

impl Debug for Beta3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AA,{}", self)
    }
}

pub fn calc_beta3(full_range: &impl Range, equitizer: &mut Equitizer, s: S) -> Beta3 {
    let (p0, eq0) = equitizer.query_sub_prob_and_eq(
        &PureRange::from("A4s"),
        &PureRange::from("AA"),
        full_range,
    );
    let (p1, eq1) = equitizer.query_sub_prob_and_eq(
        &PureRange::from("A4s"),
        &PureRange::from("AKs"),
        full_range,
    );
    let (p2, eq2) = equitizer.query_sub_prob_and_eq(
        &PureRange::from("A4s"),
        &PureRange::from("KK"),
        full_range,
    );

    let (p3, eq3) = equitizer.query_sub_prob_and_eq(
        &PureRange::from("AKo"),
        &PureRange::from("AA"),
        full_range,
    );
    let (p4, eq4) = equitizer.query_sub_prob_and_eq(
        &PureRange::from("AKo"),
        &PureRange::from("AKs"),
        full_range,
    );
    let (p5, eq5) = equitizer.query_sub_prob_and_eq(
        &PureRange::from("AKo"),
        &PureRange::from("KK"),
        full_range,
    );

    let (aks_1, kk_2) = calc_beta_2d(
        ((p0, eq0), (p1, eq1), (p2, eq2)),
        ((p3, eq3), (p4, eq4), (p5, eq5)),
        s,
    );

    Beta3 { aks_1, kk_2 }
}
