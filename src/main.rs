use hardcore_equitizer::Equitizer;
use hardcore_equitizer::HandRanker;

    pub mod all_combos;
    pub mod section1;
    pub mod section2;
    pub mod section3;
    pub mod section4;
    pub mod section5;
    pub mod section6;

use all_combos::calc_all_combos;
use section1::section1;
use section2::section2;
use section3::section3;
use section4::section4;
use section5::section5;
use section6::section6;
fn main() {
    println!("Hello, world!");

    let hand_hanker = HandRanker::new("data").unwrap();
    let mut equitizer = Equitizer::new(&hand_hanker);

    println!("开始");

    // section1(&mut equitizer);
    // section2(&mut equitizer);
    // section3(&mut equitizer);
    // section4(&mut equitizer);
    section5(&mut equitizer);
    section6(&mut equitizer);
    println!("结束");
}
