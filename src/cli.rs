

use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    action: Option<Action>,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Edit,
    Report
}

