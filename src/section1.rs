use super::calc_beta::calc_beta;
use super::calc_s::calc_s;
use super::combos::calc_all_combos;
use hardcore_equitizer::Equitizer;

pub fn calc_alpha_old(p1: f64, eq1: f64, p2: f64, eq2: f64, s: f64) -> f64 {
    // a x + b = 0
    let a = p2 * eq2 * (2.0 * s + 1.0) - p2 * s;
    let b = p1 * eq1 * (2.0 * s + 1.0) - p1 * s;

    -b / a
}

pub fn calc_alpha_new((p1, eq1): (f64, f64), (p2, eq2): (f64, f64), s: f64) -> f64 {
    // a x + b = 0
    let a = p1 * eq1 * (2.0 * s + 1.0) - p1 * s;
    let b = p2 * eq2 * (2.0 * s + 1.0) - p2 * s;

    -b / a
}

fn alpha1(equitizer: &mut Equitizer, s: f64) -> f64 {
    let p1 = equitizer.query_prob("AKs", "AA,ATs");
    let eq1 = equitizer.query_eq("AKs", "AA,ATs");
    let p2 = equitizer.query_prob("AKs", "A5s");
    let eq2 = equitizer.query_eq("AKs", "A5s");
    calc_alpha_old(p1, eq1, p2, eq2, s)
}

fn beta1(equitizer: &mut Equitizer, s: f64) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq("A5s", "AA");
    let (p1, eq1) = equitizer.query_prob_and_eq("A5s", "AKs");
    let eq1 = equitizer.query_eq("A5s", "AKs");
    calc_beta((p0, eq0), (p1, eq1), s)
}

pub fn section1(equitizer: &mut Equitizer) {
    println!("# section 2");

    const C_50_2: f64 = 50.0 * 49.0 / 2.0;

    let s1 = calc_s(
        equitizer.query_prob("A5s", "AA"),
        equitizer.query_eq("A5s", "AA"),
    );
    println!("s1=s(A5s;AA)={:.2}", s1);

    {
        let attacker_range = "AA,ATs,A5s";

        let all_combos = calc_all_combos();

        let mut combo_and_eq_vs_attacker = all_combos
            .iter()
            .map(|c| (c, equitizer.query_eq(c, attacker_range)))
            .collect::<Vec<_>>();
        combo_and_eq_vs_attacker.sort_by(|(_, eq1), (_, eq2)| eq2.partial_cmp(eq1).unwrap());

        for (combo, eq) in combo_and_eq_vs_attacker.iter().take(5) {
            println!("EQ[{combo};{attacker_range}]={:.2}%", eq * 100.0,);
        }
        println!("");
    }

    println!("alpha1(s1)={:.2}%", alpha1(equitizer, s1) * 100.0);
    println!("beta1(s1)={:.15}%", beta1(equitizer, s1) * 100.0);

    {
        println!("AA ({:.2}%)", equitizer.query_prob("", "AA") * 100.0);
        println!(
            "AA,ATs ({:.2}%)",
            equitizer.query_prob("", "AA,ATs") * 100.0
        );

        let p_AA_ATs = equitizer.query_prob("", "AA,ATs");
        let p_A5s = equitizer.query_prob("", "A5s");
        println!(
            "AA,ATs,A5s:alpha ({:.2}%)",
            (p_AA_ATs + p_A5s * alpha1(equitizer, s1)) * 100.0
        );
    }

    println!();
}
