use super::aux::calc_s;
use super::format::pretty_s;
use hardcore_equitizer::Equitizer;

pub fn section09(equitizer: &mut Equitizer) {
    let s9 = calc_s9(equitizer);
    println!("s9: {}", pretty_s(s9));
}

fn calc_s9(equitizer: &mut Equitizer) -> f64 {
    let p_and_eq = equitizer.query_prob_and_eq("A4s", "KK+");
    calc_s(p_and_eq)
}
