use crate::calc_attacker_ev::calc_attacker_ev_0d;
use crate::research_attacker::research_attacker_0d;
use crate::research_defender::research_defender_0d;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
use hardcore_equitizer::Range;

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

    return;

    let s0 = s1;

    research_attacker_0d(equitizer, "AA", &full_range, s0, 5);
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
