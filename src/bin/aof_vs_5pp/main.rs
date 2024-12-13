use hardcore_equitizer::Equitizer;
use hardcore_equitizer::HandRanker;

mod calc_alpha;
mod calc_attacker_ev;
mod calc_beta;
mod research_attacker;
mod research_defender;
mod section00;
mod section01;
mod section02;
mod section03;

use clap::Parser;
use section00::section00;
use section01::section01;
use section02::section02;
use section03::section03;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    section: u32,
}

fn main() {
    println!("Hello, world!");

    let hand_hanker = HandRanker::new("data").unwrap();
    let mut equitizer = Equitizer::new(&hand_hanker).unwrap();

    let cli = Cli::parse();

    println!("section {}", cli.section);
    println!("");

    match cli.section {
        0 => section00(&mut equitizer),
        1 => section01(&mut equitizer),
        2 => section02(&mut equitizer),
        3 => section03(&mut equitizer),
        _ => panic!("invalid section"),
    }
}
