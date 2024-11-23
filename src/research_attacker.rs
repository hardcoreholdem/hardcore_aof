use crate::aux;
use crate::combos;
use crate::format::pretty_percent;
use crate::format::pretty_s;
use crate::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;

// 攻方的不同组合面对 defender_0 + defender_1:beta1, defender_2:beta2 的 EQ 和 EV
// 2d表示两个自由度
pub fn research_attacker_2d(
    equitizer: &mut Equitizer,
    defender_0: &str,
    (beta_1, defender_1): (f64, &str),
    (beta_2, defender_2): (f64, &str),
    s: S,
    limit: usize,
) {
    println!(
        "EQ & EV, vs {defender_0},{defender_1}:{},{defender_2}:{}",
        pretty_percent(beta_1),
        pretty_percent(beta_2)
    );
    let mut combo_and_eq_and_ev_vec = Vec::new();

    for combo in combos::calc_all_combos() {
        let combo_range = PureRange::from(&combo);
        let (p_0, eq_0) = equitizer.query_prob_and_eq(&combo_range, &PureRange::from(defender_0));
        let (p_1, eq_1) = equitizer.query_prob_and_eq(&combo_range, &PureRange::from(defender_1));
        let (p_2, eq_2) = equitizer.query_prob_and_eq(&combo_range, &PureRange::from(defender_2));

        let eq = aux::calc_eq_2d((p_0, eq_0), (beta_1, p_1, eq_1), (beta_2, p_2, eq_2));
        let ev = aux::calc_attacker_ev_2d((p_0, eq_0), (beta_1, p_1, eq_1), (beta_2, p_2, eq_2), s);

        combo_and_eq_and_ev_vec.push((combo, eq, ev));
    }

    combo_and_eq_and_ev_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    for (combo, eq, ev) in combo_and_eq_and_ev_vec.iter().take(limit) {
        println!("{combo}, eq={}, ev={}", pretty_percent(*eq), pretty_s(*ev));
    }
    println!("");
}

// 攻方的不同组合面对 defender_0 + defender_1:beta1 的 EQ 和 EV
// 1d表示一个自由度
pub fn research_attacker_1d(
    equitizer: &mut Equitizer,
    defender_0: &str,
    defender_1: &str,
    beta_1: f64,
    s: S,
    limit: usize,
) {
    println!(
        "EQ & EV, vs {defender_0},{defender_1}:{}",
        pretty_percent(beta_1),
    );
    let mut combo_and_eq_and_ev_vec = Vec::new();

    for combo in combos::calc_all_combos() {
        let combo_range = PureRange::from(&combo);
        let p_and_eq_0 = equitizer.query_prob_and_eq(&combo_range, &PureRange::from(defender_0));
        let p_and_eq_1 = equitizer.query_prob_and_eq(&combo_range, &PureRange::from(defender_1));

        let eq = aux::calc_eq_1d(p_and_eq_0, beta_1, p_and_eq_1);
        let ev = aux::calc_attacker_ev_1d(p_and_eq_0, beta_1, p_and_eq_1, s);

        combo_and_eq_and_ev_vec.push((combo, eq, ev));
    }

    combo_and_eq_and_ev_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    for (combo, eq, ev) in combo_and_eq_and_ev_vec.iter().take(limit) {
        println!("{combo}, eq={}, ev={}", pretty_percent(*eq), pretty_s(*ev));
    }
    println!("");
}
