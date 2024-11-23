use hardcore_aof::aux::calc_alpha_1d;
use hardcore_aof::aux::join_calc_s_and_beta;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;

pub fn section07(equitizer: &mut Equitizer) {
    let (s7, beta) = calc_s7_and_beta(equitizer);

    println!("s: {:.2}", s7);
    println!("beta: {:.2}%", beta * 100.0);

    println!("alpha7(s7): {:.2}%", calc_alpha7(equitizer, s7) * 100.0);
}

fn calc_s7_and_beta(equitizer: &mut Equitizer) -> (S, f64) {
    let (p0, eq0) = equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("AA"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("KK"));
    let (p2, eq2) = equitizer.query_prob_and_eq(&PureRange::from("A5s"), &PureRange::from("AA"));
    let (p3, eq3) = equitizer.query_prob_and_eq(&PureRange::from("A5s"), &PureRange::from("KK"));

    join_calc_s_and_beta(((p0, eq0), (p1, eq1)), ((p2, eq2), (p3, eq3)))
}

pub fn calc_alpha7(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("KK+AK"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("KK"), &PureRange::from("A5s"));

    calc_alpha_1d((p0, eq0), (p1, eq1), s)
}
