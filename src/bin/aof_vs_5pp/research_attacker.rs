use hardcore_aof::aux;
use hardcore_aof::combos;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::format::pretty_s;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
use hardcore_equitizer::Range;

// // 攻方的不同组合面对 defender_0 + defender_1:beta1, defender_2:beta2 的 EQ 和 EV
// // 2d表示两个自由度
// pub fn research_attacker_2d(
//     equitizer: &mut Equitizer,
//     defender_0: &str,
//     (beta_1, defender_1): (f64, &str),
//     (beta_2, defender_2): (f64, &str),
//     s: S,
//     limit: usize,
// ) {
//     println!(
//         "EQ & EV, vs {defender_0},{defender_1}:{},{defender_2}:{}",
//         pretty_percent(beta_1),
//         pretty_percent(beta_2)
//     );
//     let mut combo_and_eq_and_ev_vec = Vec::new();

//     for combo in combos::calc_all_combos() {
//         let (p_0, eq_0) = equitizer.query_prob_and_eq(&combo, defender_0);
//         let (p_1, eq_1) = equitizer.query_prob_and_eq(&combo, defender_1);
//         let (p_2, eq_2) = equitizer.query_prob_and_eq(&combo, defender_2);

//         let eq = aux::calc_eq_2d((p_0, eq_0), (beta_1, p_1, eq_1), (beta_2, p_2, eq_2));
//         let ev = aux::calc_attacker_ev_2d((p_0, eq_0), (beta_1, p_1, eq_1), (beta_2, p_2, eq_2), s);

//         combo_and_eq_and_ev_vec.push((combo, eq, ev));
//     }

//     combo_and_eq_and_ev_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

//     for (combo, eq, ev) in combo_and_eq_and_ev_vec.iter().take(limit) {
//         println!("{combo}, eq={}, ev={}", pretty_percent(*eq), pretty_s(*ev));
//     }
//     println!("");
// }

// // 攻方的不同组合面对 defender_0 + defender_1:beta1 的 EQ 和 EV
// // 1d表示一个自由度
// pub fn research_attacker_1d(
//     equitizer: &mut Equitizer,
//     defender_0: &str,
//     defender_1: &str,
//     beta_1: f64,
//     s: S,
//     limit: usize,
// ) {
//     println!(
//         "EQ & EV, vs {defender_0},{defender_1}:{}",
//         pretty_percent(beta_1),
//     );
//     let mut combo_and_eq_and_ev_vec = Vec::new();

//     for combo in combos::calc_all_combos() {
//         let p_and_eq_0 = equitizer.query_prob_and_eq(&combo, defender_0);
//         let p_and_eq_1 = equitizer.query_prob_and_eq(&combo, defender_1);

//         let eq = aux::calc_eq_1d(p_and_eq_0, beta_1, p_and_eq_1);
//         let ev = aux::calc_attacker_ev_1d(p_and_eq_0, beta_1, p_and_eq_1, s);

//         combo_and_eq_and_ev_vec.push((combo, eq, ev));
//     }

//     combo_and_eq_and_ev_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

//     for (combo, eq, ev) in combo_and_eq_and_ev_vec.iter().take(limit) {
//         println!("{combo}, eq={}, ev={}", pretty_percent(*eq), pretty_s(*ev));
//     }
//     println!("");
// }

// 攻方的不同组合面对 defender_0 + defender_1:beta1, defender_2:beta2 的 EQ 和 EV
// 2d表示两个自由度
pub fn research_attacker_0d(
    equitizer: &mut Equitizer,
    full_range: &impl Range,
    defender_0: &str,
    s: S,
    limit: usize,
) {
    println!("EQ & EV, vs {defender_0}, s={}", s);
    let mut combo_and_eq_and_ev_vec = Vec::new();

    for combo in combos::calc_all_combos() {
        let p_0 = equitizer.query_sub_prob(
            &PureRange::from(combo.as_str()),
            &PureRange::from(defender_0),
            full_range,
        );
        let eq_0 = equitizer.query_eq(
            &PureRange::from(combo.as_str()),
            &PureRange::from(defender_0),
        );

        let eq = aux::calc_eq_0d((p_0, eq_0));
        let ev = aux::calc_attacker_ev_0d((p_0, eq_0), s);

        combo_and_eq_and_ev_vec.push((combo, eq, ev));
    }

    combo_and_eq_and_ev_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    for (combo, eq, ev) in combo_and_eq_and_ev_vec.iter().take(limit) {
        println!("{combo}, eq={}, ev={}", pretty_percent(*eq), pretty_s(*ev));
    }
    println!("");
}