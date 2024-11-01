use hardcore_equitizer::Equitizer;

use crate::format::pretty_percent;

use super::format::pretty_s;
use super::search::binary_search;
use super::section06::calc_beta6;
use super::section07::calc_alpha7;

pub fn section08(equitizer: &mut Equitizer) {
    println!("# section 8");

    let s8 = search_s8_for_beta6_equals_1(equitizer);
    println!("s8: {:.2}", s8);

    for s in [315.0, 310.0, 305.0, 300.0, 295.0, s8] {
        println!(
            "s: {}, alpha7: {}, beta6: {}",
            pretty_s(s),
            pretty_percent(calc_alpha7(equitizer, s)),
            pretty_percent(calc_beta6(equitizer, s)),
        );
    }

    // for s in [318.0, 315.0, 310.0, 300.0] {
    //     println!("s: {}", s);
    //     println!("alpha6(s): {:.2}%", alpha6(equitizer, s) * 100.0);
    //     //        println!("beta6(): {:.2}%", beta6(equitizer, s) * 100);
    // }
}

pub fn search_s8_for_beta6_equals_1(equitizer: &mut Equitizer) -> f64 {
    let f = |s| calc_beta6(equitizer, s) - 1.0;
    binary_search(0.0, 315.0, f)
}
