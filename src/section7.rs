use hardcore_equitizer::Equitizer;

// use crate::section7::alpha6;

use super::calc_beta::join_calc_s_and_beta;
use super::section6::beta6;

fn s7_and_beta(equitizer: &mut Equitizer) -> (f64, f64) {
    let (p0, eq0) = equitizer.query_prob_and_eq("KK", "AA");
    let (p1, eq1) = equitizer.query_prob_and_eq("KK", "KK");
    let (p2, eq2) = equitizer.query_prob_and_eq("A5s", "AA");
    let (p3, eq3) = equitizer.query_prob_and_eq("A5s", "KK");

    join_calc_s_and_beta(((p0, eq0), (p1, eq1)), ((p2, eq2), (p3, eq3)))
}

pub fn section8(equitizer: &mut Equitizer) {
    println!("# section 8");

    // for s in [318.0, 315.0, 310.0, 300.0] {
    //     println!("s: {}", s);
    //     println!("alpha6(s): {:.2}%", alpha6(equitizer, s) * 100.0);
    //     //        println!("beta6(): {:.2}%", beta6(equitizer, s) * 100);
    // }

    let (s7, beta) = s7_and_beta(equitizer);

    println!("s: {:.2}", s7);
    println!("beta: {:.2}%", beta * 100.0);
}
