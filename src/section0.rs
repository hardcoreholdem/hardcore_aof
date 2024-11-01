use hardcore_equitizer::Equitizer;

const C_50_2: f64 = 50.0 * 49.0 / 2.0;

pub fn f1(p: f64, eq: f64) -> f64 {
    // a s + b = 0
    let a = p * (eq * 2.0 - 1.0);
    let b = p * eq + 1.0 - p;
    -b / a
}

pub fn section1(equitizer: &mut Equitizer) {
    println!("# section 1");

    const C_50_2: f64 = 50.0 * 49.0 / 2.0;
    const C_52_2: f64 = 52.0 * 51.0 / 2.0;

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
        f1(
            equitizer.query_prob("65s", "AA"),
            equitizer.query_eq("65s", "AA")
        )
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

    let s0 = f1(
        equitizer.query_prob("ATs", "AA"),
        equitizer.query_eq("ATs", "AA"),
    );

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

        let mut cum_num_combos = 0.0;
        for (combo, eq) in combo_and_eq_vs_attacker.iter().take(5) {
            cum_num_combos += equitizer.query_avg_num_combos(attacker_range, combo);
            println!(
                "EQ[{combo};{attacker_range}]={:.2}%, 累计占比={:.2}%",
                eq * 100.0,
                cum_num_combos as f64 / C_50_2 * 100.0
            );
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
