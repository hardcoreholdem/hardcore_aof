use crate::aux;
use crate::combos;
use crate::format::pretty_percent;
use crate::types::S;
use hardcore_equitizer::Equitizer;

// 攻方的不同组合面对 attacker_0 + attacker_1:beta1, attacker_2:beta2 的 EQ
// 2d表示两个自由度
pub fn research_defender_2d(
    equitizer: &mut Equitizer,
    defender_0: &str,
    defender_1: &str,
    alpha_1: f64,
    defender_2: &str,
    alpha_2: f64,
    s: S,
    limit: usize,
) {
    println!(
        "EQ vs {defender_0},{defender_1}:{},{defender_2}:{}",
        pretty_percent(alpha_1),
        pretty_percent(alpha_2),
    );
    let mut combo_and_eq_vec = Vec::new();

    for combo in combos::calc_all_combos() {
        let p_and_eq_0 = equitizer.query_prob_and_eq(&combo, defender_0);
        let p_and_eq_1 = equitizer.query_prob_and_eq(&combo, defender_1);
        let p_and_eq_2 = equitizer.query_prob_and_eq(&combo, defender_2);

        let eq = aux::calc_eq_2d(p_and_eq_0, p_and_eq_1, p_and_eq_2, alpha_1, alpha_2);

        combo_and_eq_vec.push((combo, eq));
    }

    combo_and_eq_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (combo, eq) in combo_and_eq_vec.iter().take(limit) {
        let s: f64 = s.into();
        let extra = if f64::abs(eq - s / (2.0 * s + 1.0)) < 1e-9 {
            " (BE)"
        } else {
            ""
        };
        println!("{combo}, eq={}{extra}", pretty_percent(*eq));
    }
    println!("");
}

// 攻方的不同组合面对 attacker_0 + attacker_1:beta1 的 EQ
// 1d表示零个自由度
pub fn research_defender_1d(
    equitizer: &mut Equitizer,
    defender_0: &str,
    alpha_1: f64,
    defender_1: &str,
    s: f64,
    limit: usize,
) {
    println!(
        "EQ vs {defender_0},{defender_1}:{}",
        pretty_percent(alpha_1),
    );
    let mut combo_and_eq_vec = Vec::new();

    for combo in combos::calc_all_combos() {
        let p_and_eq_0 = equitizer.query_prob_and_eq(&combo, defender_0);
        let p_and_eq_1 = equitizer.query_prob_and_eq(&combo, defender_1);

        let eq = aux::calc_eq_1d(p_and_eq_0, alpha_1, p_and_eq_1);

        combo_and_eq_vec.push((combo, eq));
    }

    combo_and_eq_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (combo, eq) in combo_and_eq_vec.iter().take(limit) {
        let extra = if f64::abs(eq - s / (2.0 * s + 1.0)) < 1e-9 {
            " (BE)"
        } else {
            ""
        };
        println!("{combo}, eq={}{extra}", pretty_percent(*eq));
    }
    println!("");
}
