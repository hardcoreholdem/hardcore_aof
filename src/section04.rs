use crate::aux::calc_alpha_2d;
use crate::aux::calc_beta_2d;
use crate::aux::join_calc_s_and_beta;
use crate::format::pretty_percent;
use hardcore_equitizer::Equitizer;

pub fn section04(equitizer: &mut Equitizer) {
    let (s4, beta) = calc_s4_and_beta(equitizer);

    println!("s: {:.2}", s4);
    println!("beta: {:.2}%", beta * 100.0);

    {
        let mut combo_and_eq_and_ev_vec = Vec::new();
        for combo in [
            "AKs", "AQs", "AJs", "ATs", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s", "A2s",
            "AKo", "AQo", "AJo", "ATo", "A9o", "A8o", "A7o", "A6o", "A5o", "A4o", "A3o", "A2o",
        ]
        .iter()
        {
            let p1 = equitizer.query_prob(&combo, "AA");
            let eq1 = equitizer.query_eq(&combo, "AA");
            let p2 = equitizer.query_prob(&combo, "AKs");
            let eq2 = equitizer.query_eq(&combo, "AKs");
            let eq = (eq1 * p1 + eq2 * p2 * beta) / (p1 + p2 * beta);

            let ev = p1 * (eq1 * (2.0 * s4 + 1.0) - s4)
                + beta * p2 * (eq2 * (2.0 * s4 + 1.0) - s4)
                + (1.0 - p1 - beta * p2);

            combo_and_eq_and_ev_vec.push((combo, eq, ev));
        }
        combo_and_eq_and_ev_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        println!("ratio={:.2}", beta);
        for (combo, eq, ev) in combo_and_eq_and_ev_vec.iter().take(5) {
            println!("{:?}: {:.2}%, {:.6}", combo, eq * 100.0, ev);
        }
    }

    {
        {
            let defender = "AKs";
            let attacker = "AA,AKs,A5s";
            println!(
                "EQ[{defender};{attacker}]={:.2}%",
                equitizer.query_eq(defender, attacker) * 100.0
            );
        }

        {
            let defender = "AKs";
            let attacker = "AA,AKs,A5s,AKo";
            println!(
                "EQ[{defender};{attacker}]={:.2}%",
                equitizer.query_eq(defender, attacker) * 100.0
            );
        }

        {
            let defender = "KK";
            let attacker = "AA,AKs,A5s,AKo";
            println!(
                "EQ[{defender};{attacker}]={:.2}%",
                equitizer.query_eq(defender, attacker) * 100.0
            );
        }

        println!("");
    }

    let alpha4 = calc_alpha4(equitizer, s4);

    println!("alpha4_ATs: {}", pretty_percent(alpha4.ats));
    println!("alpha4_AKo: {}", pretty_percent(alpha4.ako));

    let beta4 = calc_beta4(equitizer, s4);
    println!("beta4_AKs: {}", pretty_percent(beta4.aks));
    println!("beta4_KK: {}", pretty_percent(beta4.kk));
}

pub struct Alpha4 {
    pub ats: f64,
    pub ako: f64,
}

pub fn calc_alpha4(equitizer: &mut Equitizer, s: f64) -> Alpha4 {
    let p_and_eq_0 = equitizer.query_prob_and_eq("AKs", "AA,AKs,A5s");
    let p_and_eq_1 = equitizer.query_prob_and_eq("AKs", "ATs");
    let p_and_eq_2 = equitizer.query_prob_and_eq("AKs", "AKo");
    let p_and_eq_3 = equitizer.query_prob_and_eq("KK", "AA,AKs,A5s");
    let p_and_eq_4 = equitizer.query_prob_and_eq("KK", "ATs");
    let p_and_eq_5 = equitizer.query_prob_and_eq("KK", "AKo");

    let (ats, ako) = calc_alpha_2d(
        (p_and_eq_0, p_and_eq_1, p_and_eq_2),
        (p_and_eq_3, p_and_eq_4, p_and_eq_5),
        s,
    );

    Alpha4 { ats, ako }
}

pub struct Beta4 {
    pub aks: f64,
    pub kk: f64,
}

pub fn calc_beta4(equitizer: &mut Equitizer, s: f64) -> Beta4 {
    let (p1, eq1) = equitizer.query_prob_and_eq("ATs", "AKs");
    let (p2, eq2) = equitizer.query_prob_and_eq("ATs", "KK");
    let (p0, eq0) = equitizer.query_prob_and_eq("ATs", "AA");
    let (p4, eq4) = equitizer.query_prob_and_eq("AKo", "AKs");
    let (p5, eq5) = equitizer.query_prob_and_eq("AKo", "KK");
    let (p3, eq3) = equitizer.query_prob_and_eq("AKo", "AA");

    let (aks, kk) = calc_beta_2d(
        ((p0, eq0), (p1, eq1), (p2, eq2)),
        ((p3, eq3), (p4, eq4), (p5, eq5)),
        s,
    );

    Beta4 { aks, kk }
}

pub fn calc_s4_and_beta(equitizer: &mut Equitizer) -> (f64, f64) {
    // TODO: refactor
    let (p1, eq1, p2, eq2) = (
        equitizer.query_prob("ATs", "AA"),
        equitizer.query_eq("ATs", "AA"),
        equitizer.query_prob("ATs", "AKs"),
        equitizer.query_eq("ATs", "AKs"),
    );
    let (p3, eq3, p4, eq4) = (
        equitizer.query_prob("AKo", "AA"),
        equitizer.query_eq("AKo", "AA"),
        equitizer.query_prob("AKo", "AKs"),
        equitizer.query_eq("AKo", "AKs"),
    );

    join_calc_s_and_beta(((p1, eq1), (p2, eq2)), ((p3, eq3), (p4, eq4)))
}
