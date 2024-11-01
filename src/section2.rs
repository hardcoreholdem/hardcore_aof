use super::calc_beta::calc_beta;
use super::section1::calc_alpha_old;
use hardcore_equitizer::Equitizer;

const C_50_2: f64 = 1225.0;

fn inv_beta1(equitizer: &mut Equitizer, beta: f64) -> f64 {
    #[allow(non_snake_case)]
    let prob_A5s_vs_AA = equitizer.query_prob("A5s", "AA");

    #[allow(non_snake_case)]
    let eq_A5s_vs_AA = equitizer.query_eq("A5s", "AA");

    #[allow(non_snake_case)]
    let prob_A5s_vs_AKs = equitizer.query_prob("A5s", "AKs");

    #[allow(non_snake_case)]
    let eq_A5s_vs_AKs = equitizer.query_eq("A5s", "AKs");

    // a s + b = 0
    let a = prob_A5s_vs_AA * (eq_A5s_vs_AA * 2.0 - 1.0)
        + beta * prob_A5s_vs_AKs * (eq_A5s_vs_AKs * 2.0 - 1.0);

    let b = prob_A5s_vs_AA * eq_A5s_vs_AA + beta * prob_A5s_vs_AKs * eq_A5s_vs_AKs + 1.0
        - prob_A5s_vs_AA
        - beta * prob_A5s_vs_AKs;

    -b / a
}

fn alpha2(equitizer: &mut Equitizer, s: f64) -> f64 {
    let p1 = equitizer.query_prob("AKs", "AA,A5s");
    let eq1 = equitizer.query_eq("AKs", "AA,A5s");
    let n2 = equitizer.query_prob("AKs", "ATs");
    let p2 = equitizer.query_eq("AKs", "ATs");
    calc_alpha_old(p1, eq1, n2, p2, s)
}

fn beta2(equitizer: &mut Equitizer, s: f64) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq("ATs", "AA");
    let (p1, eq1) = equitizer.query_prob_and_eq("ATs", "AKs");
    calc_beta((p0, eq0), (p1, eq1), s)
}

pub fn section2(equitizer: &mut Equitizer) {
    println!("# section 3");

    for ratio in (0..6).map(|i| i as f64 / 100.0) {
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

    let beta_A5s_eq_ATs = {
        // ax + b = 0
        let a = equitizer.query_prob("A5s", "AKs") * equitizer.query_eq("A5s", "AKs")
            - equitizer.query_prob("ATs", "AKs") * equitizer.query_eq("ATs", "AKs");
        let b = equitizer.query_prob("A5s", "AA") * equitizer.query_eq("A5s", "AA")
            - equitizer.query_prob("ATs", "AA") * equitizer.query_eq("ATs", "AA");
        -b / a
    };
    println!("beta_A5s_eq_ATs={:.2}%", beta_A5s_eq_ATs * 100.0);

    let s2 = inv_beta1(equitizer, beta_A5s_eq_ATs);

    println!("s2=inv_beta1={:.2}", s2);
    println!("");

    {
        let ratio = beta_A5s_eq_ATs;
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

        println!("ratio={:.2}%", ratio * 100.0);
        for (combo, eq) in combo_and_eq_vec.iter().take(5) {
            println!("{:?}: {:.2}%", combo, eq * 100.0);
        }
        println!("");
    }

    println!("alpha2(s2)={:.2}%", alpha2(equitizer, s2) * 100.0);
    println!("beta2(s2)={:.2}%", beta2(equitizer, s2) * 100.0);
}
