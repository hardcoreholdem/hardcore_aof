use super::search::binary_search;
use super::section10::calc_beta10;
use crate::types::S;
use hardcore_equitizer::Equitizer;

pub fn section11(equitizer: &mut Equitizer) {
    let s11 = search_s11_for_beta_10_equals_1(equitizer);
    println!("s11: {}", s11);
}

pub fn search_s11_for_beta_10_equals_1(equitizer: &mut Equitizer) -> S {
    let f = |s| calc_beta10(equitizer, s) - 1.0;
    binary_search(0.into(), 300.into(), f)
}
