use super::calc_s::calc_s;
use super::format::pretty_f64;
use hardcore_equitizer::Equitizer;

pub fn section9(equitizer: &mut Equitizer) {
    let s9 = calc_s9(equitizer);
    println!("s9: {}", pretty_f64(s9));
}

fn calc_s9(equitizer: &mut Equitizer) -> f64 {
    let (p, eq) = equitizer.query_prob_and_eq("A4s", "KK+");
    calc_s(p, eq)
}
