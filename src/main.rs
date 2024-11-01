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
use section00::section00;
use section01::section01;
use section02::section02;
use section03::section03;
use section04::section04;
use section05::section05;
use section06::section06;
use section07::section07;
use section08::section08;
use section09::section09;
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
        0 => section00(&mut equitizer),
        1 => section01(&mut equitizer),
        2 => section02(&mut equitizer),
        3 => section03(&mut equitizer),
        4 => section04(&mut equitizer),
        5 => section05(&mut equitizer),
        6 => section06(&mut equitizer),
        7 => section07(&mut equitizer),
        8 => section08(&mut equitizer),
        9 => section09(&mut equitizer),
        10 => section10(&mut equitizer),
        11 => section11(&mut equitizer),
        12 => section12(&mut equitizer),
        _ => panic!("invalid section"),
    }
}
