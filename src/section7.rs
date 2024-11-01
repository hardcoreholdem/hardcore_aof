use super::calc_alpha::calc_alpha;
use super::calc_beta::join_calc_s_and_beta;
use hardcore_equitizer::Equitizer;

fn s6_and_beta(equitizer: &mut Equitizer) -> (f64, f64) {
    let (p0, eq0) = equitizer.query_prob_and_eq("A5s", "AA");
    let (p1, eq1) = equitizer.query_prob_and_eq("A5s", "KK");
    let (p2, eq2) = equitizer.query_prob_and_eq("AKo", "AA");
    let (p3, eq3) = equitizer.query_prob_and_eq("AKo", "KK");

    join_calc_s_and_beta(((p0, eq0), (p1, eq1)), ((p2, eq2), (p3, eq3)))
}

fn alpha6(equitizer: &mut Equitizer, s: f64) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq("KK", "AA,AK");
    let (p1, eq1) = equitizer.query_prob_and_eq("KK", "A5s");

    calc_alpha((p0, eq0), (p1, eq1), s)
}

pub fn section7(equitizer: &mut Equitizer) {
    println!("# section 7");

    let (s6, beta) = s6_and_beta(equitizer);

    println!("s: {:.2}", s6);
    println!("beta: {:.2}%", beta * 100.0);

    println!("alpha6(s6): {:.2}%", alpha6(equitizer, s6) * 100.0);
}
