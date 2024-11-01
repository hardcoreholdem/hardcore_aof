use hardcore_equitizer::Equitizer;

use super::section01::calc_alpha_old;

fn inv_beta2(equitizer: &mut Equitizer, beta: f64) -> f64 {
    // TODO: use query_prob_and_eq
    let prob_ats_vs_aa = equitizer.query_prob("ATs", "AA");
    let eq_ats_vs_aa = equitizer.query_eq("ATs", "AA");

    // TODO: use query_prob_and_eq
    let prob_ats_vs_aks = equitizer.query_prob("ATs", "AKs");
    let eq_ats_vs_aks = equitizer.query_eq("ATs", "AKs");

    // a s + b = 0
    let a = prob_ats_vs_aa * (eq_ats_vs_aa * 2.0 - 1.0)
        + beta * prob_ats_vs_aks * (eq_ats_vs_aks * 2.0 - 1.0);

    let b = prob_ats_vs_aa * eq_ats_vs_aa + beta * prob_ats_vs_aks * eq_ats_vs_aks + 1.0
        - prob_ats_vs_aa
        - beta * prob_ats_vs_aks;

    -b / a
}

fn alpha3(equitizer: &mut Equitizer, s3: f64) -> f64 {
    let p1 = equitizer.query_prob("AKs", "AA,AKs,A5s");
    let eq1 = equitizer.query_eq("AKs", "AA,AKs,A5s");
    let p2 = equitizer.query_prob("AKs", "ATs");
    let eq2 = equitizer.query_eq("AKs", "ATs");

    calc_alpha_old(p1, eq1, p2, eq2, s3)
}

pub fn section03(equitizer: &mut Equitizer) {
    println!("# section 4");

    for ratio in (4..7).map(|i| i as f64 / 100.0) {
        let mut combo_and_eq_vec = Vec::new();
        for combo in [
            "AKs", "AQs", "AJs", "ATs", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s", "A2s",
        ]
        .iter()
        {
            let p1 = equitizer.query_prob(&combo, "AA");
            let eq1 = equitizer.query_eq(&combo, "AA");
            let p2 = equitizer.query_prob(&combo, "AKs");
            let eq2 = equitizer.query_eq(&combo, "AKs");
            let eq = (eq1 * p1 + eq2 * p2 * ratio) / (p1 + p2 * ratio);
            combo_and_eq_vec.push((combo, eq));
        }
        combo_and_eq_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        println!("ratio={:.2}", ratio);
        for (combo, eq) in combo_and_eq_vec.iter().take(5) {
            println!("{:?}: {:.2}%", combo, eq * 100.0);
        }
    }

    // TODO: use other methods to calculate beta_AKs_eq_ATs
    let beta_aks_eq_ats = {
        // ax + b = 0
        let a = equitizer.query_prob("AKs", "AKs") * equitizer.query_eq("AKs", "AKs")
            - equitizer.query_prob("ATs", "AKs") * equitizer.query_eq("ATs", "AKs");
        let b = equitizer.query_prob("AKs", "AA") * equitizer.query_eq("AKs", "AA")
            - equitizer.query_prob("ATs", "AA") * equitizer.query_eq("ATs", "AA");
        -b / a
    };

    println!("beta_AKs_eq_ATs={:.2}%", beta_aks_eq_ats * 100.0);

    let s3 = inv_beta2(equitizer, beta_aks_eq_ats);
    println!("s3=inv_beta2={:.2}", s3);

    {
        let ratio = beta_aks_eq_ats;
        let mut combo_and_eq_vec = Vec::new();
        for combo in [
            "AKs", "AQs", "AJs", "ATs", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s", "A2s",
        ]
        .iter()
        {
            let p1 = equitizer.query_prob(&combo, "AA");
            let eq1 = equitizer.query_eq(&combo, "AA");
            let p2 = equitizer.query_prob(&combo, "AKs");
            let eq2 = equitizer.query_eq(&combo, "AKs");
            let eq = (eq1 * p1 + eq2 * p2 * ratio) / (p1 + p2 * ratio);
            combo_and_eq_vec.push((combo, eq));
        }
        combo_and_eq_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        println!("ratio={:.2}", ratio);
        for (combo, eq) in combo_and_eq_vec.iter().take(5) {
            println!("{:?}: {:.2}%", combo, eq * 100.0);
        }
        println!("");
    }

    {
        for attacker in ["AA", "AA,A5s", "AA,A5s,AKs", "AA,A5s,AKs,ATs"] {
            let defender = "AKs";
            let eq = equitizer.query_eq(defender, attacker);
            println!("EQ[{defender};{attacker}]={:.2}%", eq * 100.0);
        }
        println!("");
    }

    println!("alpha3={:.2}%", alpha3(equitizer, s3) * 100.0);
}
