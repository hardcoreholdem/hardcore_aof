use crate::calc_beta::calc_beta_1d;
use hardcore_aof::aux;
use hardcore_aof::combos::calc_all_combos;
use hardcore_aof::format::pretty_percent;
use hardcore_aof::types::S;
use hardcore_equitizer::Equitizer;
use hardcore_equitizer::PureRange;
pub fn section01(equitizer: &mut Equitizer) {
    let s1 =
        aux::calc_s(equitizer.query_prob_and_eq(&PureRange::from("A5s"), &PureRange::from("AA")));
    println!("s1=s(A5s;AA)={:.2}", s1);

    {
        let attacker_range_str = "AA,ATs,A5s";
        let attacker_range = PureRange::from(attacker_range_str);

        let all_combos = calc_all_combos();

        let mut combo_and_eq_vs_attacker = all_combos
            .iter()
            .map(|c| (c, equitizer.query_eq(&PureRange::from(c), &attacker_range)))
            .collect::<Vec<_>>();
        combo_and_eq_vs_attacker.sort_by(|(_, eq1), (_, eq2)| eq2.partial_cmp(eq1).unwrap());

        for (combo, eq) in combo_and_eq_vs_attacker.iter().take(5) {
            println!("EQ[{combo};{attacker_range_str}]={:.2}%", eq * 100.0,);
        }
        println!("");
    }

    println!("alpha1(s1)={}", pretty_percent(calc_alpha1(equitizer, s1)));
    println!("beta1(s1)={}", pretty_percent(calc_beta1(equitizer, s1)));

    println!();
}

#[deprecated]
pub fn calc_alpha_old(p1: f64, eq1: f64, p2: f64, eq2: f64, s: S) -> f64 {
    let s: f64 = s.into();

    // a x + b = 0
    let a = p2 * eq2 * (2.0 * s + 1.0) - p2 * s;
    let b = p1 * eq1 * (2.0 * s + 1.0) - p1 * s;

    -b / a
}

#[deprecated]
pub fn calc_alpha_new((p1, eq1): (f64, f64), (p2, eq2): (f64, f64), s: S) -> f64 {
    let s: f64 = s.into();

    // a x + b = 0
    let a = p1 * eq1 * (2.0 * s + 1.0) - p1 * s;
    let b = p2 * eq2 * (2.0 * s + 1.0) - p2 * s;

    -b / a
}

fn calc_alpha1(equitizer: &mut Equitizer, s: S) -> f64 {
    let p1 = equitizer.query_prob(&PureRange::from("AKs"), &PureRange::from("AA,ATs"));
    let eq1 = equitizer.query_eq(&PureRange::from("AKs"), &PureRange::from("AA,ATs"));
    let p2 = equitizer.query_prob(&PureRange::from("AKs"), &PureRange::from("A5s"));
    let eq2 = equitizer.query_eq(&PureRange::from("AKs"), &PureRange::from("A5s"));
    calc_alpha_old(p1, eq1, p2, eq2, s)
}

fn calc_beta1(equitizer: &mut Equitizer, s: S) -> f64 {
    let (p0, eq0) = equitizer.query_prob_and_eq(&PureRange::from("A5s"), &PureRange::from("AA"));
    let (p1, eq1) = equitizer.query_prob_and_eq(&PureRange::from("A5s"), &PureRange::from("AKs"));
    calc_beta_1d((p0, eq0), (p1, eq1), s)
}
