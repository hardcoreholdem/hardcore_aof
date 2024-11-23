use hardcore_aof::aux;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
pub fn section09(equitizer: &mut Equitizer) {
    let s9 = calc_s9(equitizer);
    println!("s9: {}", s9);
}

fn calc_s9(equitizer: &mut Equitizer) -> S {
    let p_and_eq = equitizer.query_prob_and_eq(&PureRange::from("A4s"), &PureRange::from("KK+"));
    aux::calc_s(p_and_eq)
}
