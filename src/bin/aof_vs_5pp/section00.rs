use crate::calc_attacker_ev::calc_attacker_ev_0d;
use hardcore_aof::aux;
use hardcore_aof::search::binary_search;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
use hardcore_equitizer::Range;

pub fn section00(equitizer: &mut Equitizer) {
    let full_range = PureRange::from("99+,AJs+,AQo+");
    for s in [
        1000.into(),
        500.into(),
        250.into(),
        100.into(),
        50.into(),
        40.into(),
        30.into(),
    ] {
        crate::research_attacker::research_attacker_0d(equitizer, &full_range, "AA", s, 5);
    }

    let s0 = search_s0_for_atk_eq_of_ats_equals_0(equitizer, &full_range);
    println!("s0={}", s0);
    return;

    let all_combos = hardcore_aof::combos::calc_all_combos();

    {
        let mut combo_and_eq_vs_aa = all_combos
            .iter()
            .map(|c| {
                (
                    c,
                    equitizer.query_eq(&PureRange::from(c), &PureRange::from("AA")),
                )
            })
            .collect::<Vec<_>>();
        combo_and_eq_vs_aa.sort_by(|(_, eq1), (_, eq2)| eq2.partial_cmp(eq1).unwrap());

        for (combo, eq) in combo_and_eq_vs_aa.iter().take(10) {
            println!("EQ[{combo};AA]={:.2}%", eq * 100.0,);
        }
        println!("");
    }

    println!(
        "s(65s;AA)={:.2}",
        aux::calc_s(equitizer.query_prob_and_eq(&PureRange::from("65s"), &PureRange::from("AA")))
    );

    {
        let mut combo_and_eq_vs_aa = [
            "AKs", "AQs", "AJs", "ATs", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s", "A2s",
        ]
        .iter()
        .map(|&c| {
            (
                c,
                equitizer.query_eq(&PureRange::from(c), &PureRange::from("AA")),
            )
        })
        .collect::<Vec<_>>();
        combo_and_eq_vs_aa.sort_by(|(_, eq1), (_, eq2)| eq2.partial_cmp(eq1).unwrap());

        for (combo, eq) in combo_and_eq_vs_aa.iter().take(5) {
            println!("EQ[{combo};AA]={:.2}%", eq * 100.0,);
        }
        println!("");
    }

    let s0 =
        aux::calc_s(equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("AA")));

    println!("s0=s(ATs;AA)={:.2}", s0);

    println!("");

    {
        let attacker_range_str = "AA,ATs";
        let attacker_range = PureRange::from(attacker_range_str);
        let mut combo_and_eq_vs_attacker = all_combos
            .iter()
            .map(|c| (c, equitizer.query_eq(&PureRange::from(c), &attacker_range)))
            .collect::<Vec<_>>();
        combo_and_eq_vs_attacker.sort_by(|(_, eq1), (_, eq2)| eq2.partial_cmp(eq1).unwrap());

        for (combo, eq) in combo_and_eq_vs_attacker.iter().take(5) {
            println!("EQ[{combo};{attacker_range_str}]={:.2}%", eq * 100.0,);
        }
        println!("");
    }

    println!("");
}

pub fn search_s0_for_atk_eq_of_ats_equals_0(
    equitizer: &mut Equitizer,
    full_range: &impl Range,
) -> S {
    let f = |s| -> f64 {
        let (p_0, eq_0) = equitizer.query_sub_prob_and_eq(
            &PureRange::from("ATs"),
            &PureRange::from("AA"),
            full_range,
        );
        calc_attacker_ev_0d((p_0, eq_0), s)
    };

    binary_search(30.into(), 40.into(), f)
}
