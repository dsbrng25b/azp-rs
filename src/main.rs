use dirs;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use chrono::prelude::*;
use std::error::Error;
use std::char;
use std::fmt;
use anyhow::{Context, Result};

const FILE_NAME: &str = "azp.txt";
const YEAR: i32 = 2020;


// #[derive(Debug)]
// struct MyError<'a, E: Error> {
//     inner: Option<&'a E>,
//     description: String,
// }
// 
// impl fmt::Display for MyError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}: {}", self.description, self.inner)
//     }
// }
// 
// impl Error for MyError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         self.inner
//     }
// }

struct Workunit {
    from: DateTime<Local>,
    to: DateTime<Local>,
    comment: String,
}

fn main() -> Result<()> {
    let mut file = dirs::home_dir().expect("");

    file.push(FILE_NAME);

    println!("open {}", file.to_str().unwrap_or(""));

    let file = File::open(&file).with_context(|| format!("failed to open"))?;

    let file = BufReader::new(file);

    let mut units: Vec<Workunit> = Vec::new();

    for (i, line) in file.lines().enumerate() {
        let line = line.with_context(|| format!("failed to read"))?;

        // skip comments
        if line.starts_with('#') {
            continue
        }
        // skip empty lines
        if line == "" {
            continue
        }

        units.push(parse_line(&line).with_context(|| format!("failed to parse line {}", i + 1))?);
    }

    for unit in units {
        let work_time = unit.to - unit.from;
        println!("from: {}, to: {}, comment: {}, time: {}h{}min", unit.from, unit.to, unit.comment, work_time.num_hours(), work_time.num_minutes() % 60);
    }

    Ok(())
}

fn parse_line(line: &str) -> Result<Workunit> {
    let parts: Vec<&str> = line.splitn(4, char::is_whitespace).collect();
    let date: Vec<&str> = parts[0].split(".").collect();
    let from: Vec<&str> = parts[1].split(":").collect();
    let to: Vec<&str> = parts[2].split(":").collect();

    let date: Date<Local> = Local.ymd(YEAR, date[1].parse()?, date[0].parse()?);

    let from =date.and_hms(from[0].parse()?, from[1].parse()?, 0);
    let to =date.and_hms(to[0].parse()?, to[1].parse()?, 0);

    Ok(Workunit {
        from: from,
        to: to,
        comment: parts[3].to_string(),
    })
}
