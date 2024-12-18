use hardcore_aof::aux;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;

pub fn section00(equitizer: &mut Equitizer) {
    let all_combos = {
        let mut combos: Vec<String> = Vec::new();

        let rank_strs = "AKQJT98765432";

        // pocket pairs
        for rank_str in rank_strs.chars() {
            combos.push(format!("{}{}", rank_str, rank_str));
        }

        for i in 0..rank_strs.len() {
            for j in (i + 1)..rank_strs.len() {
                combos.push(format!("{}{}s", &rank_strs[i..i + 1], &rank_strs[j..j + 1]));
                combos.push(format!("{}{}o", &rank_strs[i..i + 1], &rank_strs[j..j + 1]));
            }
        }

        combos
    };

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
