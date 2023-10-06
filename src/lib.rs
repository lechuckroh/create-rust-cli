use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use chrono::{Datelike, DateTime, NaiveDateTime, Timelike};
use regex::Regex;
use strfmt::strfmt;

pub type Vars = HashMap<String, String>;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn split_exif_line(s: &str) -> Option<(String, String)> {
    let tokens: Vec<&str> = s.splitn(2, ":").collect();
    if tokens.len() == 2 {
        Some((tokens[0].trim().to_string(), tokens[1].trim().to_string()))
    } else {
        None
    }
}

pub fn read_exif_file(filepath: &str) -> MyResult<Vars> {
    let mut vars: Vars = HashMap::new();
    match open(filepath) {
        Err(err) => eprintln!("failed to open {}: {}", filepath, err),
        Ok(buf_read) => {
            for line_result in buf_read.lines() {
                let line = line_result?;
                if let Some((key, value)) = split_exif_line(line.as_str()) {
                    vars.insert(key, value);
                }
            }
        }
    }
    Ok(vars)
}

pub fn create_vars_from_create_date(create_date: &str) -> Vars {
    let mut vars: Vars = HashMap::new();
    if let Some(datetime) = parse_datetime_from_string(create_date) {
        let date = datetime.date();
        let time = datetime.time();

        // Y - 4-digit year
        vars.insert("Y".to_string(), date.year().to_string());
        // y - 2-digit year
        vars.insert("y".to_string(), (date.year() % 100).to_string());
        // m - month (01-12)
        vars.insert("m".to_string(), format!("{:02}", date.month()));
        // D - Day of the month (01-31)
        vars.insert("D".to_string(), format!("{:02}", date.day()));
        // t - time HHMMSS
        vars.insert("t".to_string(), format!("{:02}{:02}{:02}", time.hour(), time.minute(), time.second()));
        // H - hour (00-23)
        vars.insert("H".to_string(), format!("{:02}", time.hour()));
        // h - hour (01-12)
        vars.insert("h".to_string(), format!("{:02}", time.hour12().1));
        // M - minutes (00-59)
        vars.insert("M".to_string(), format!("{:02}", time.minute()));
        // S - seconds (00-59)
        vars.insert("S".to_string(), format!("{:02}", time.second()));
    }
    vars
}

fn parse_datetime_from_string(s: &str) -> Option<NaiveDateTime> {
    return match NaiveDateTime::parse_from_str(s, "%Y:%m:%d %H:%M:%S") {
        Ok(dt) => Some(dt),
        Err(_) => {
            match DateTime::parse_from_str(s, "%Y:%m:%d %H:%M:%S%z") {
                Ok(dt) => Some(dt.naive_local()),
                Err(e) => {
                    eprintln!("parse error: {}", e.to_string());
                    None
                }
            }
        }
    };
}

pub fn create_vars_from_filename(filename: &str) -> Vars {
    let mut vars: Vars = HashMap::new();

    let regex = Regex::new(r"(.*\D)(\d*)\.([a-zA-Z0-9]+)").unwrap();
    if let Some(groups) = regex.captures(filename) {
        let image_name = groups.get(1).map_or("", |m| m.as_str());
        let image_number = groups.get(2).map_or("", |m| m.as_str());
        let extension = groups.get(3).map_or("", |m| m.as_str());
        vars.insert("f".to_string(), image_name.to_string());
        vars.insert("r".to_string(), image_number.to_string());
        vars.insert("e".to_string(), extension.to_string());
    }

    return vars;
}

pub fn extend_vars(exif_vars: &Vars) -> Vars {
    let mut vars: Vars = HashMap::new();
    if let Some(create_date) = exif_vars.get("CreateDate") {
        vars.extend(create_vars_from_create_date(create_date));
    }
    if let Some(filename) = exif_vars.get("FileName") {
        vars.extend(create_vars_from_filename(filename));
    }
    if let Some(model) = exif_vars.get("Model") {
        vars.insert("T2".to_string(), model.to_string());
    }
    return vars;
}

pub fn format_filename(pattern: &str, exif_vars: Vars) -> String {
    let mut vars: Vars = extend_vars(&exif_vars);
    vars.extend(exif_vars);
    return strfmt(pattern, &vars).unwrap();
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
