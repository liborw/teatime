
use std::{str::FromStr, num::ParseIntError, fs, collections::HashMap};

use chrono::{NaiveDate, NaiveTime, Duration, Datelike};
use regex::Regex;

use clap::{Parser, Subcommand, Args};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    action: Option<Action>,
}

#[derive(Subcommand, Debug)]
enum Action {
    Edit(EditArgs),
    Report(ReportArgs)
}

#[derive(Args, Debug)]
struct EditArgs {

    #[arg(short, long)]
    span: Option<String>,

}

#[derive(Args, Debug)]
struct ReportArgs {

    #[arg(long)]
    from_file: String,

    #[arg(short, long)]
    filter: Option<String>,

    #[arg(short, long)]
    span: Option<String>,
}




#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Tag {
    Label(String),
}

#[derive(Clone, Debug)]
struct Entry {
    date: NaiveDate,
    t_start: NaiveTime,
    t_stop: NaiveTime,
    note: String,
    tags: Vec<Tag>
}

impl Entry {

    fn duration(&self) -> Duration {
        self.t_stop - self.t_start
    }
}

impl FromStr for Entry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_line = Regex::new(r"^(?P<date>\d{4}-\d{2}-\d{2}) +(?P<time>\d{2}:\d{2}-\d{2}:\d{2}) +(?P<note>.+)$").unwrap();
        let re_time = Regex::new(r"^(?P<h0>\d{2}):(?P<m0>\d{2})-(?P<h1>\d{2}):(?P<m1>\d{2})").unwrap();
        let re_tag  = Regex::new(r"@(?P<label>\w+)(\((?P<value>\w+)?\))?").unwrap();

        let cap = re_line.captures(s).expect("Failed to find entry in the string");
        let date = NaiveDate::from_str(cap.name("date").unwrap().as_str()).expect("Failed to create date from str");
        let note = cap.name("note").unwrap().as_str().to_owned();

        let cap = re_time.captures(cap.name("time").unwrap().as_str()).unwrap();

        let t_start = NaiveTime::from_hms_opt(cap.name("h0").unwrap().as_str().parse().unwrap(), cap.name("m0").unwrap().as_str().parse().unwrap(), 0).unwrap();
        let t_stop = NaiveTime::from_hms_opt(cap.name("h1").unwrap().as_str().parse().unwrap(), cap.name("m1").unwrap().as_str().parse().unwrap(), 0).unwrap();

        let mut tags = vec![];
        for cap in re_tag.captures_iter(note.as_str()) {
            tags.push(Tag::Label(cap.name("label").unwrap().as_str().to_owned()));
        }


        Ok(Entry { date, t_start, t_stop, note, tags })
    }

}

fn today() -> NaiveDate {
    let date = chrono::Utc::now();
    NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap()
}

fn fmt_duration(d: &Duration) -> String {
    format!("{}h{}", d.num_hours(), d.num_minutes() - d.num_hours() * 60)
}

fn action_report(args: ReportArgs) {
    println!("{:}", args.from_file);

    let entries: Vec<_> = fs::read_to_string(args.from_file).unwrap()
                     .lines()
                     .filter(|l| l.len() > 0)
                     .map(|line| Entry::from_str(line).unwrap())
                     .collect();

    let mut spent: HashMap<Tag, Duration> = HashMap::new();
    entries.iter()
           .for_each(|e| {
               e.tags.iter()
                     .for_each(|t| {
                         spent.entry(t.clone()).and_modify(|d| *d = *d + e.duration()).or_insert(e.duration());
                     })
           });

    spent.iter()
         .for_each(|(k, v)| {
            println!("{:?}: {}", k, fmt_duration(v));
         });




    let today = today();

    let this_week = today.week(chrono::Weekday::Mon);
    println!("{this_week:?}");
    for day in this_week.first_day().iter_days().take_while(|d| d <= &this_week.last_day()) {
    }

    let mut all = Duration::zero();
    for entry in entries.iter() {
        if this_week.days().contains(&entry.date) {
            all = all + entry.duration();
            println!("{}", fmt_duration(&entry.duration()));
        }
    }
    println!("{}", fmt_duration(&all));

    let duration_this_week = entries.iter()
                                    .filter(|e| this_week.days().contains(&e.date))
                                    .fold(Duration::zero(), |d, e| d + (e.t_stop - e.t_start));

    let this_month = today.month();
    let duration_this_month = entries.iter()
                                     .filter(|e| e.date.month() == this_month)
                                     .fold(Duration::zero(), |d, e| d + (e.t_stop - e.t_start));


    println!("This month already {}h{}", duration_this_month.num_hours(), duration_this_month.num_minutes() - duration_this_month.num_hours() * 60);
    println!("This week already {}h{}", duration_this_week.num_hours(), duration_this_week.num_minutes() - duration_this_week.num_hours() * 60);
}



use std::process::Command;


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

fn main() {

    let cli = Cli::parse();
    println!("{cli:#?}");

    match cli.action {

        Some(Action::Edit(args)) => {
            action_edit(args);
        },
        Some(Action::Report(args)) => {
            action_report(args);
        }
        None => {

        }
    }
}
