
use std::{str::FromStr, num::ParseIntError, fs};

use chrono::{NaiveDate, NaiveTime, Duration, Datelike};
use clap::Parser;
use regex::Regex;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long, required=false)]
    from_file: String,

}

#[derive(Clone, Debug)]
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

impl FromStr for Entry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_line = Regex::new(r"^(?P<date>\d{4}-\d{2}-\d{2}) +(?P<time>\d{2}:\d{2}-\d{2}:\d{2}) ?(?P<duration>\d+h\d{2})? +(?P<note>.+)$").unwrap();
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

fn main() {

    let args = Args::parse();
    println!("{:}", args.from_file);

    let entries: Vec<_> = fs::read_to_string(args.from_file).unwrap()
                     .lines()
                     .filter(|l| l.len() > 0)
                     .map(|line| Entry::from_str(line).unwrap())
                     .collect();



    let today = today();
    let this_week = today.week(chrono::Weekday::Mon);

    let duration_this_week = entries.iter()
                                    .filter(|e| e.date >= this_week.first_day())
                                    .fold(Duration::zero(), |d, e| d + (e.t_stop - e.t_start));

    println!("This week already {}h{}", duration_this_week.num_hours(), duration_this_week.num_minutes() - duration_this_week.num_hours() * 60);


}
