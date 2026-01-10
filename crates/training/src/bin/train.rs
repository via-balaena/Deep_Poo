use clap::Parser;
use training::util::{run_train, TrainArgs};

fn main() -> anyhow::Result<()> {
    let args = TrainArgs::parse();
    run_train(args)
}
