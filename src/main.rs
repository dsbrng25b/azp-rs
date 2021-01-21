use dirs;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use chrono::prelude::*;
use std::error::Error as StdError;
use std::fmt;
use anyhow::{Context, Result};

const FILE_NAME: &str = "azp.txt";
const YEAR: i32 = 2020;

#[derive(Debug)]
struct Error<'a> {
    description: &'a str,
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "err: {}", self.description)
    }
}

impl<'a> StdError for Error<'a> {
}

struct Workunit {
    from: DateTime<Local>,
    to: DateTime<Local>,
    comment: Option<String>,
}

fn main() -> Result<()> {
    let mut file_path = dirs::home_dir().ok_or( Error { description: "dirs not found", })?;
    file_path.push(FILE_NAME);

    let file = File::open(&file_path)?;
    let file = BufReader::new(file);

    let mut units: Vec<Workunit> = Vec::new();
    for (i, line) in file.lines().enumerate() {
        let line = line?;

        // skip comments
        if line.starts_with('#') {
            continue
        }
        // skip empty lines
        if line == "" {
            continue
        }

        let work_unit = parse_line(&line)
            .with_context(||format!("failed to parse line {} '{}'", i + 1, line))?;
        units.push(work_unit);
    }

    for unit in units.iter() {
        let work_time = unit.to - unit.from;
        print!("{} - {}: {}h{}min", unit.from, unit.to, work_time.num_hours(), work_time.num_minutes() % 60);
        if let Some(comment) = &unit.comment {
            println!(" {}", comment);
        } else {
            println!("");
        }
    }

    let total_minutes = get_total_minutes(&units);
    println!("\nTotal working time: {}h{}min", total_minutes / 60, total_minutes % 60);

    Ok(())
}

fn get_total_minutes(units: &Vec<Workunit>) -> i64{

    let mut total_work_minutes = 0i64;

    for unit in units.iter() {
        let worktime = unit.to - unit.from;
        total_work_minutes += worktime.num_minutes();
    }

    total_work_minutes
}

fn parse_line(line: &str) -> Result<Workunit> {
    let mut iter = line.split_whitespace();

    let date = iter.next().ok_or(Error{ description: "missing date", })?;
    let from = iter.next().ok_or(Error{ description: "missing date", })?;
    let to = iter.next().ok_or(Error{ description: "missing date", })?;

    // todo get full description/comment
    let comment = iter.next().map(ToOwned::to_owned);

    let date = parse_date(date, YEAR)?;
    let from = parse_time(from, &date)?;
    let to = parse_time(to, &date)?;

    Ok(Workunit {
        from,
        to,
        comment,
    })
}

fn parse_date(s: &str, year: i32) -> Result<Date<Local>> {
    let mut iter = s.splitn(2, ".");
    let day = iter.next().ok_or(Error{ description: "missing day", })?.parse()?;
    let month = iter.next().ok_or(Error{ description: "missing month", })?.parse()?;
    Ok(Local.ymd(year, month, day))
}

fn parse_time(s: &str, date: &Date<Local>) -> Result<DateTime<Local>> {
    let mut iter = s.splitn(2, ":");
    let hour = iter.next().ok_or(Error{ description: "missing hour", })?.parse()?;
    let minute = iter.next().ok_or(Error{ description: "missing minute", })?.parse()?;
    Ok(date.and_hms(hour, minute, 0))
}