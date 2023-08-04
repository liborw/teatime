

use chrono::NaiveDate;
use clap::{Parser, Subcommand, Args};
use crate::duration::{Duration, ParseDurationError};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[arg(long)]
    pub db: Option<String>,

    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Edit(EditArgs),
    Report(ReportArgs),
    Log(LogArgs)
}


#[derive(Args, Debug)]
pub struct LogArgs {

    /// Date of the work, if omited today is used
    #[arg(short, long)]
    pub date: Option<NaiveDate>,

    /// Time spend on a task
    #[arg(short, long, value_parser = parse_duration)]
    pub time: Duration,

    /// Note, shoudl include tags
    pub note:Vec<String>,

}

#[derive(Args, Debug)]
pub struct EditArgs {

    #[arg(short, long)]
    pub span: Option<String>,

}

#[derive(Args, Debug, Default)]
pub struct ReportArgs {


    #[arg(long)]
    pub from: Option<NaiveDate>,

    #[arg(long)]
    pub to: Option<NaiveDate>,

    #[arg(short, long)]
    pub tags: Vec<String>,
}

fn parse_duration(arg: &str) -> Result<Duration, ParseDurationError> {
    Ok(arg.parse()?)
}
