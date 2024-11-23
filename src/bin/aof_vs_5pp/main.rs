use hardcore_equitizer::Equitizer;
use hardcore_equitizer::HandRanker;

mod calc_attacker_ev;
mod research_attacker;
mod section00;

use clap::Parser;
use section00::section00;
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
        _ => panic!("invalid section"),
    }
}
