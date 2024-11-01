use hardcore_equitizer::Equitizer;
use hardcore_equitizer::HandRanker;

mod calc_alpha;
mod calc_beta;
mod calc_s;
mod combos;
mod format;
mod search;
mod section0;
mod section1;
mod section10;
mod section2;
mod section3;
mod section4;
mod section5;
mod section6;
mod section7;
mod section8;
mod section9;

use clap::Parser;
use section0::section0;
use section1::section1;
use section10::section10;
use section2::section2;
use section3::section3;
use section4::section4;
use section5::section5;
use section6::section6;
use section7::section7;
use section8::section8;
use section9::section9;

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
        0 => section0(&mut equitizer),
        1 => section1(&mut equitizer),
        2 => section2(&mut equitizer),
        3 => section3(&mut equitizer),
        4 => section4(&mut equitizer),
        5 => section5(&mut equitizer),
        6 => section6(&mut equitizer),
        7 => section7(&mut equitizer),
        8 => section8(&mut equitizer),
        9 => section9(&mut equitizer),
        10 => section10(&mut equitizer),
        _ => panic!("invalid section"),
    }
}
