use crate::section01::calc_alpha_old;
use hardcore_aof::aux::calc_beta_1d;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;

fn inv_beta1(equitizer: &mut Equitizer, beta: f64) -> S {
    // TODO: rename to p_and_eq_xxx
    let prob_a5s_vs_a = equitizer.query_prob(&PureRange::from("A5s"), &PureRange::from("AA"));
    let eq_a5s_vs_aa = equitizer.query_eq(&PureRange::from("A5s"), &PureRange::from("AA"));

    // TODO: rename to p_and_eq_xxx
    let prob_a5s_vs_aks = equitizer.query_prob(&PureRange::from("A5s"), &PureRange::from("AKs"));
    let eq_a5s_vs_aks = equitizer.query_eq(&PureRange::from("A5s"), &PureRange::from("AKs"));

    // a s + b = 0
    let a = prob_a5s_vs_a * (eq_a5s_vs_aa * 2.0 - 1.0)
        + beta * prob_a5s_vs_aks * (eq_a5s_vs_aks * 2.0 - 1.0);

    let b = prob_a5s_vs_a * eq_a5s_vs_aa + beta * prob_a5s_vs_aks * eq_a5s_vs_aks + 1.0
        - prob_a5s_vs_a
        - beta * prob_a5s_vs_aks;

    (-b / a).into()
}

fn calc_alpha2(equitizer: &mut Equitizer, s: S) -> f64 {
    // TODO: use query_prob_and_eq
    let p1 = equitizer.query_prob(&PureRange::from("AKs"), &PureRange::from("AA,A5s"));
    let eq1 = equitizer.query_eq(&PureRange::from("AKs"), &PureRange::from("AA,A5s"));
    let n2 = equitizer.query_prob(&PureRange::from("AKs"), &PureRange::from("ATs"));
    let p2 = equitizer.query_eq(&PureRange::from("AKs"), &PureRange::from("ATs"));
    calc_alpha_old(p1, eq1, n2, p2, s)
}

fn calc_beta2(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("AA"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("ATs"), &PureRange::from("AKs"));
    calc_beta_1d((p0, eq0), (p1, eq1), s)
}

pub fn section02(equitizer: &mut Equitizer) {
    println!("# section 3");

    for ratio in (0..6).map(|i| i as f64 / 100.0) {
        let mut combo_and_eq_vec = Vec::new();
        for &combo in [
            "AKs", "AQs", "AJs", "ATs", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s", "A2s",
        ]
        .iter()
        {
            let p1 = equitizer.query_prob(&PureRange::from(combo), &PureRange::from("AA"));
            let eq1 = equitizer.query_eq(&PureRange::from(combo), &PureRange::from("AA"));
            let p2 = equitizer.query_prob(&PureRange::from(combo), &PureRange::from("AKs"));
            let eq2 = equitizer.query_eq(&PureRange::from(combo), &PureRange::from("AKs"));
            let eq = (eq1 * p1 + eq2 * p2 * ratio) / (p1 + p2 * ratio);
            combo_and_eq_vec.push((combo, eq));
        }
        combo_and_eq_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        println!("ratio={:.2}", ratio);
        for (combo, eq) in combo_and_eq_vec.iter().take(5) {
            println!("{:?}: {:.2}%", combo, eq * 100.0);
        }
    }

    let beta_a5s_eq_ats = {
        // ax + b = 0
        let a = equitizer.query_prob(&PureRange::from("A5s"), &PureRange::from("AKs"))
            * equitizer.query_eq(&PureRange::from("A5s"), &PureRange::from("AKs"))
            - equitizer.query_prob(&PureRange::from("ATs"), &PureRange::from("AKs"))
                * equitizer.query_eq(&PureRange::from("ATs"), &PureRange::from("AKs"));
        let b = equitizer.query_prob(&PureRange::from("A5s"), &PureRange::from("AA"))
            * equitizer.query_eq(&PureRange::from("A5s"), &PureRange::from("AA"))
            - equitizer.query_prob(&PureRange::from("ATs"), &PureRange::from("AA"))
                * equitizer.query_eq(&PureRange::from("ATs"), &PureRange::from("AA"));
        -b / a
    };
    println!("beta_A5s_eq_ATs={:.2}%", beta_a5s_eq_ats * 100.0);

    let s2 = inv_beta1(equitizer, beta_a5s_eq_ats);

    println!("s2=inv_beta1={:.2}", s2);
    println!("");

    {
        let ratio = beta_a5s_eq_ats;
        let mut combo_and_eq_vec = Vec::new();
        for &combo in [
            "AKs", "AQs", "AJs", "ATs", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s", "A2s",
        ]
        .iter()
        {
            let p1 = equitizer.query_prob(&PureRange::from(combo), &PureRange::from("AA"));
            let eq1 = equitizer.query_eq(&PureRange::from(combo), &PureRange::from("AA"));
            let p2 = equitizer.query_prob(&PureRange::from(combo), &PureRange::from("AKs"));
            let eq2 = equitizer.query_eq(&PureRange::from(combo), &PureRange::from("AKs"));
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

    println!("alpha2(s2)={}", pretty_percent(calc_alpha2(equitizer, s2)));
    println!("beta2(s2)={}", pretty_percent(calc_beta2(equitizer, s2)));
}
