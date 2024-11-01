use hardcore_equitizer::Equitizer;
use hardcore_equitizer::HandRanker;

pub mod all_combos;
mod calc_alpha;
pub mod calc_beta;
pub mod section1;
pub mod section2;
pub mod section3;
pub mod section4;
pub mod section5;
pub mod section6;
mod section7;
mod section8;

use section1::section1;
use section2::section2;
use section3::section3;
use section4::section4;
use section5::section5;
use section6::section6;
use section7::section7;
use section8::section8;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    section: u32,
}

fn main() {
    println!("Hello, world!");

    let hand_hanker = HandRanker::new("data").unwrap();
    let mut equitizer = Equitizer::new(&hand_hanker);

    let cli = Cli::parse();

    match cli.section {
        1 => section1(&mut equitizer),
        2 => section2(&mut equitizer),
        3 => section3(&mut equitizer),
        4 => section4(&mut equitizer),
        5 => section5(&mut equitizer),
        6 => section6(&mut equitizer),
        7 => section7(&mut equitizer),
        8 => section8(&mut equitizer),
        _ => panic!("invalid section"),
    }
}
