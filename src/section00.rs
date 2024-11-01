use hardcore_equitizer::Equitizer;

use crate::aux::calc_s;

pub fn section00(equitizer: &mut Equitizer) {
    println!("# section 1");

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
        #[allow(non_snake_case)]
        let mut combo_and_eq_vs_AA = all_combos
            .iter()
            .map(|c| (c, equitizer.query_eq(c, "AA")))
            .collect::<Vec<_>>();
        combo_and_eq_vs_AA.sort_by(|(_, eq1), (_, eq2)| eq2.partial_cmp(eq1).unwrap());

        for (combo, eq) in combo_and_eq_vs_AA.iter().take(10) {
            println!("EQ[{combo};AA]={:.2}%", eq * 100.0,);
        }
        println!("");
    }

    println!(
        "s(65s;AA)={:.2}",
        calc_s(equitizer.query_prob_and_eq("65s", "AA"),)
    );

    {
        #[allow(non_snake_case)]
        let mut combo_and_eq_vs_AA = [
            "AKs", "AQs", "AJs", "ATs", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s", "A2s",
        ]
        .iter()
        .map(|c| (c, equitizer.query_eq(c, "AA")))
        .collect::<Vec<_>>();
        combo_and_eq_vs_AA.sort_by(|(_, eq1), (_, eq2)| eq2.partial_cmp(eq1).unwrap());

        for (combo, eq) in combo_and_eq_vs_AA.iter().take(5) {
            println!("EQ[{combo};AA]={:.2}%", eq * 100.0,);
        }
        println!("");
    }

    let s0 = calc_s(equitizer.query_prob_and_eq("ATs", "AA"));

    println!("s0=s(ATs;AA)={:.2}", s0);

    println!("");

    {
        let attacker_range = "AA,ATs";
        #[allow(non_snake_case)]
        let mut combo_and_eq_vs_attacker = all_combos
            .iter()
            .map(|c| (c, equitizer.query_eq(c, attacker_range)))
            .collect::<Vec<_>>();
        combo_and_eq_vs_attacker.sort_by(|(_, eq1), (_, eq2)| eq2.partial_cmp(eq1).unwrap());

        for (combo, eq) in combo_and_eq_vs_attacker.iter().take(5) {
            println!("EQ[{combo};{attacker_range}]={:.2}%", eq * 100.0,);
        }
        println!("");
    }

    {
        println!("AA ({:.2}%)", equitizer.query_prob("", "AA") * 100.0);
        println!(
            "AA,ATs ({:.2}%)",
            equitizer.query_prob("", "AA,ATs") * 100.0
        );
    }

    println!("");
}
