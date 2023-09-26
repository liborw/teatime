
use std::{str::FromStr, num::ParseIntError, fs::{self, File, OpenOptions}, io::{Write, self, BufRead}, fmt};
use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;

use chrono::{NaiveDate, Datelike, Weekday, NaiveWeek, Month};
use regex::Regex;

use std::process::Command;

use teatime::{duration::Duration, cli::GroupBy};
use teatime::cli::{Cli, EditArgs, LogArgs, ReportArgs, Action};
use teatime::tag::Tag;



#[derive(Clone, Debug)]
struct Entry {
    date: NaiveDate,
    duration: Duration,
    note: String,
    tags: Vec<Tag>
}

impl Entry {

    fn new(date: NaiveDate, duration: Duration, note: String) -> Self {
        let tags: Vec<Tag> = Tag::find(&note).collect();
        Entry { date, duration, note, tags }
    }

}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.date, self.duration, self.note)
    }
}

enum ParseEntryError {
    WrongDate,
    RegexFailed,
    Parse(ParseIntError)
}

impl FromStr for Entry {
    // 2023-08-04 3h30 Doing something on project @teatime
    type Err = ParseEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(?P<Y>\d{4})-(?P<M>\d{2})-(?P<D>\d{2}) +(?P<h>\d{1})h(?P<m>\d+) +(?P<note>.+)$").unwrap();
        }

        let cap = RE.captures(s).ok_or(ParseEntryError::RegexFailed)?;

        let date = NaiveDate::from_ymd_opt(
            cap.name("Y").unwrap().as_str().parse().map_err(|e| ParseEntryError::Parse(e))?,
            cap.name("M").unwrap().as_str().parse().map_err(|e| ParseEntryError::Parse(e))?,
            cap.name("D").unwrap().as_str().parse().map_err(|e| ParseEntryError::Parse(e))?
        ).ok_or(ParseEntryError::WrongDate)?;

        let h: i64 = cap.name("h").unwrap().as_str().parse().map_err(|e| ParseEntryError::Parse(e))?;
        let m: i64 = cap.name("m").unwrap().as_str().parse().map_err(|e| ParseEntryError::Parse(e))?;
        let duration = Duration::minutes(h*60 + m);
        let note = cap.name("note").unwrap().as_str().to_owned();

        Ok(Entry::new( date, duration, note))
    }

}

fn today() -> NaiveDate {
    let date = chrono::Utc::now();
    NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap()
}



fn action_edit(args: EditArgs) {
    println!("{args:#?}");

    let editor = "nvim";
    let tmp_file = "/tmp/teatime_edit";

    // Write info and entries to the file

    // Start editor
    let status = Command::new(editor)
                         .arg(tmp_file)
                         .status()
                         .unwrap();

    if !status.success() {
        return;
    }

    // Read and precess edited file

    let entries = fs::read_to_string(tmp_file).unwrap();
    println!("{entries}");

    // If thre are no problem remove it
}

fn report_action(args: ReportArgs, db: &File) {

    let mut entries: Vec<_> = io::BufReader::new(db).lines()
        // filter empty lines
        .filter_map(|l| l.ok())
        .filter_map(|line| Entry::from_str(&line).ok())

        // filter out entries out of given range
        .filter(|e| e.date > args.from.unwrap_or(NaiveDate::default()))
        .filter(|e| e.date <= args.from.unwrap_or(today()))
        // Filter by tag
        .filter(|e| args.tags.iter().all(|l| e.tags.contains(l)))
        .collect();

    entries.sort_by_key(|e| e.date);
    let total_d  = entries.iter()
        // print all entries in range
        .map(|e| {println!("{e}"); e})
        .fold(Duration::zero(), |d, e| d + e.duration);
    println!("Total duration: {total_d}")
}

fn log_action(args: LogArgs, db: &mut File) {

    let entry = Entry::new(
        args.date.unwrap_or(today()),
        args.time,
        args.note.join(" "));

    writeln!(db, "{entry}").unwrap();
}

fn main() {
    let cli = Cli::parse();

    match cli.action {
        Action::Log(args) => {
            {
                let mut db = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(cli.db.clone().unwrap_or("~/teatime_test.txt".into()))
                    .unwrap();
                log_action(args, &mut db);
            }

            let db = OpenOptions::new()
                .read(true)
                .open(cli.db.unwrap_or("~/teatime_test.txt".into()))
                .unwrap();

            let args = ReportArgs::default();
            report_action(args, &db);
        },
        Action::Report(args) => {
            let db = OpenOptions::new()
                .read(true)
                .open(cli.db.unwrap_or("~/teatime_test.txt".into()))
                .unwrap();

            report_action(args, &db);
        },
        _ => {}
    }
}
