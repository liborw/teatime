

use std::{str::FromStr, error::Error, fmt::Display};

use chrono::NaiveDate;
use clap::{Parser, Subcommand, Args};
use crate::duration::{Duration, ParseDurationError};
use crate::tag::{Tag, ParseTagsError, Tags};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[arg(long)]
    pub db: Option<String>,

    #[arg(long)]
    pub debug: Option<bool>,

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

    #[arg(short, long, value_parser = parse_tags)]
    pub tags: Option<Tag>,

    #[arg(short, long, value_parser = parse_group_by)]
    pub group_by: Option<GroupBy>,
}

#[derive(Debug, Clone)]
pub enum GroupBy {
    Day,
    Week,
    Month
}

#[derive(Debug, Clone)]
pub enum ParseGroupByError {
    Unknown
}

impl Error for ParseGroupByError {}

impl Display for ParseGroupByError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for GroupBy {
    type Err = ParseGroupByError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s.to_lowercase().as_str() {
            "day" => Ok(GroupBy::Day),
            "week" => Ok(GroupBy::Week),
            "month" => Ok(GroupBy::Month),
            _ => Err(ParseGroupByError::Unknown)
        }
    }
}

fn parse_group_by(arg: &str) -> Result<GroupBy, ParseGroupByError> {
    Ok(arg.parse()?)
}

fn parse_duration(arg: &str) -> Result<Duration, ParseDurationError> {
    Ok(arg.parse()?)
}


fn parse_tags(arg: &str) -> Result<Tag, ParseTagsError> {
    Ok(Tag::from_str(arg).expect("test"))
}
