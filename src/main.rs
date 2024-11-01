use hardcore_equitizer::Equitizer;
use hardcore_equitizer::HandRanker;

mod calc_alpha;
mod calc_beta;
mod calc_s;
mod combos;
mod format;
mod search;
mod section00;
mod section01;
mod section02;
mod section03;
mod section04;
mod section05;
mod section06;
mod section07;
mod section08;
mod section09;
mod section10;
mod section11;
mod section12;

use clap::Parser;
use section00::section0;
use section01::section1;
use section02::section2;
use section03::section3;
use section04::section4;
use section05::section5;
use section06::section6;
use section07::section7;
use section08::section8;
use section09::section9;
use section10::section10;
use section11::section11;
use section12::section12;

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

    println!("section {}", cli.section);
    println!("");

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
        11 => section11(&mut equitizer),
        12 => section12(&mut equitizer),
        _ => panic!("invalid section"),
    }
}
