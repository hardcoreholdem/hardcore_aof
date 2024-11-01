use super::calc_s::calc_s;
use super::format::pretty_s;
use hardcore_equitizer::Equitizer;

pub fn section10(equitizer: &mut Equitizer) {
    let s10 = calc_s10(equitizer);
    println!("s10: {}", pretty_s(s10));
}

fn calc_s10(equitizer: &mut Equitizer) -> f64 {
    let (p, eq) = equitizer.query_prob_and_eq("ATs", "KK+");
    calc_s(p, eq)
}
