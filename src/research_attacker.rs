use crate::aux::calc_attacker_ev_2d;
use crate::aux::calc_eq_2d;
use crate::combos;
use crate::format::pretty_percent;
use crate::format::pretty_s;
use hardcore_equitizer::Equitizer;

// 攻方的不同组合面对 defender_0 + defender_1:beta1, defender_2:beta2 的 EQ 和 EV
// 2d表示两个自由度
pub fn research_attacker_2d(
    equitizer: &mut Equitizer,
    defender_0: &str,
    defender_1: &str,
    beta_1: f64,
    defender_2: &str,
    beta_2: f64,
    s: f64,
    limit: usize,
) {
    println!(
        "EQ & EV, vs {defender_0},{defender_1}:{},{defender_2}:{}",
        pretty_percent(beta_1),
        pretty_percent(beta_2)
    );
    let mut combo_and_eq_and_ev_vec = Vec::new();

    for combo in combos::calc_all_combos() {
        let p_and_eq_0 = equitizer.query_prob_and_eq(&combo, defender_0);
        let p_and_eq_1 = equitizer.query_prob_and_eq(&combo, defender_1);
        let p_and_eq_2 = equitizer.query_prob_and_eq(&combo, defender_2);

        let eq = calc_eq_2d(p_and_eq_0, p_and_eq_1, p_and_eq_2, beta_1, beta_2);

        let ev = calc_attacker_ev_2d(p_and_eq_0, p_and_eq_1, beta_1, p_and_eq_2, beta_2, s);

        combo_and_eq_and_ev_vec.push((combo, eq, ev));
    }

    combo_and_eq_and_ev_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    for (combo, eq, ev) in combo_and_eq_and_ev_vec.iter().take(limit) {
        println!("{combo}, eq={}, ev={}", pretty_percent(*eq), pretty_s(*ev));
    }
    println!("");
}
