use super::format::pretty_s;
use super::search::binary_search;
use super::section10::calc_beta10;
use hardcore_equitizer::Equitizer;

pub fn section11(equitizer: &mut Equitizer) {
    let s11 = search_s11_for_beta_10_equals_1(equitizer);
    println!("s11: {}", pretty_s(s11));
}

pub fn search_s11_for_beta_10_equals_1(equitizer: &mut Equitizer) -> f64 {
    let f = |s| calc_beta10(equitizer, s) - 1.0;
    binary_search(0.0, 300.0, f)
}
