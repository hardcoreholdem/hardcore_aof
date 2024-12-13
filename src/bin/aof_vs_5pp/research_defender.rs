use hardcore_aof::aux;
use hardcore_aof::combos;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
use hardcore_equitizer::Range;
// 攻方的不同组合面对 attacker_0 + attacker_1:beta1, attacker_2:beta2 的 EQ
// 2d表示两个自由度
pub fn research_defender_2d(
    // TODO: update BE
    equitizer: &mut Equitizer,
    defender_0: &str,
    (alpha_1, defender_1): (f64, &str),
    (alpha_2, defender_2): (f64, &str),
    full_range: &impl Range,
    s: S,
    limit: usize,
) {
    let range_0 = PureRange::from(defender_0);
    let range_1 = PureRange::from(defender_1);
    let range_2 = PureRange::from(defender_2);
    if !range_0.is_disjoint(&range_1)
        || !range_0.is_disjoint(&range_2)
        || !range_1.is_disjoint(&range_2)
    {
        panic!("defender ranges are not disjoint");
    }

    println!(
        "EQ vs {defender_0},{defender_1}:{},{defender_2}:{}",
        pretty_percent(alpha_1),
        pretty_percent(alpha_2),
    );
    let mut combo_and_eq_vec = Vec::new();

    for combo in combos::calc_all_combos() {
        let combo_range = PureRange::from(&combo);
        if !full_range
            .iter_combos()
            .any(|c| c == combo_range.iter_combos().next().unwrap())
        {
            continue;
        }

        let (p_0, eq_0) = equitizer.query_prob_and_eq(&combo_range, &range_0);
        let (p_1, eq_1) = equitizer.query_prob_and_eq(&combo_range, &range_1);
        let (p_2, eq_2) = equitizer.query_prob_and_eq(&combo_range, &range_2);

        let eq = aux::calc_eq_2d((p_0, eq_0), (alpha_1, p_1, eq_1), (alpha_2, p_2, eq_2));

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
    (alpha_1, defender_1): (f64, &str),
    full_range: &impl Range,
    s: S,
    limit: usize,
) {
    let mut combo_and_eq_vec = Vec::new();

    for combo in combos::calc_all_combos() {
        let combo_range = PureRange::from(&combo);
        if !full_range
            .iter_combos()
            .any(|c| c == combo_range.iter_combos().next().unwrap())
        {
            continue;
        }
        let p_and_eq_0 = equitizer.query_prob_and_eq(&combo_range, &PureRange::from(defender_0));
        let p_and_eq_1 = equitizer.query_prob_and_eq(&combo_range, &PureRange::from(defender_1));

        let eq = aux::calc_eq_1d(p_and_eq_0, alpha_1, p_and_eq_1);

        combo_and_eq_vec.push((combo, eq));
    }

    combo_and_eq_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let s: f64 = s.into();
    let be_point = (s - 1.0) / (2.0 * s);

    let mut output = String::new();
    for (combo, eq) in combo_and_eq_vec.iter().take(limit) {
        if !output.is_empty() {
            output += ", ";
        }
        output += &format!("{combo}:{}", pretty_percent(*eq));
        if f64::abs(*eq - be_point) < 1e-9 {
            output += "(BE)";
        }
    }
    println!("{output}");
}

// 攻方的不同组合面对 attacker_0 的 EQ
// 0d表示零个自由度
pub fn research_defender_0d(
    // TODO: update BE
    equitizer: &mut Equitizer,
    defender_0: &str,
    full_range: &impl Range,
    s: S,
    limit: usize,
) {
    println!("EQ vs {defender_0}");
    let mut combo_and_eq_vec = Vec::new();

    for combo in combos::calc_all_combos() {
        let combo_range = PureRange::from(&combo);
        if !full_range
            .iter_combos()
            .any(|c| c == combo_range.iter_combos().next().unwrap())
        {
            continue;
        }
        let p_and_eq_0 = equitizer.query_prob_and_eq(&combo_range, &PureRange::from(defender_0));
        let eq = aux::calc_eq_0d(p_and_eq_0);
        combo_and_eq_vec.push((combo, eq));
    }

    combo_and_eq_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let s: f64 = s.into();
    let be_point = (s - 1.0) / (2.0 * s);

    for (combo, eq) in combo_and_eq_vec.iter().take(limit) {
        let extra = if f64::abs(eq - be_point) < 1e-9 {
            "(BE)"
        } else {
            ""
        };
        println!("{combo}, eq={}{extra}", pretty_percent(*eq));
    }
    println!("");
}
