use hardcore_aof::aux;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;

pub fn section12(equitizer: &mut Equitizer) {
    let s12 = calc_s12(equitizer);
    println!("s12: {}", s12);

    let alpha12 = calc_alpha12(equitizer, s12);
    println!("alpha12: {}", pretty_percent(alpha12));

    let beta12 = calc_beta12(equitizer, s12);
    println!("beta12: {}", pretty_percent(beta12));
}

pub fn calc_s12(equitizer: &mut Equitizer) -> S {
    let p_and_e = equitizer.query_prob_and_eq("A3s", "KK+,AKs");

    aux::calc_s(p_and_e)
}

pub fn calc_alpha12(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq("AKo", "KK+,AK,ATs,A5s,A4s");
    let (p1, eq1) = equitizer.query_prob_and_eq("AKo", "A3s");

    aux::calc_alpha_1d((p0, eq0), (p1, eq1), s)
}

pub fn calc_beta12(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq("A3s", "KK+,AKs");
    let (p1, eq1) = equitizer.query_prob_and_eq("A3s", "AKo");

    aux::calc_beta_1d((p0, eq0), (p1, eq1), s)
}
