use hardcore_equitizer::PureRange;

pub fn calc_all_combos() -> Vec<String> {
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
}

pub fn calc_all_combo_ranges() -> Vec<PureRange> {
    calc_all_combos()
        .iter()
        .map(|s| PureRange::from(s))
        .collect()
}
