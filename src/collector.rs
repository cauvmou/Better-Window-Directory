use std::{ffi::OsString, cmp::Ordering};
use colored::Colorize;
use time::OffsetDateTime;

use crate::DATE_FORMAT_STR;

#[derive(PartialEq, Eq)]
pub struct DirectoryObject {
    pub name: String,
    pub symbol: char,
    pub time: OffsetDateTime,
    pub color: (u8, u8, u8),
    pub bg_color: (u8, u8, u8),
    pub priority: i8,
    pub size: u64, // Bytes
}

impl DirectoryObject {
    pub fn new(name: OsString, symbol: char, priority: i8, time: OffsetDateTime, size: u64, color: (&u8, &u8, &u8), bg_color: (&u8, &u8, &u8)) -> Self {
        Self {
            name: String::from(name.to_str().unwrap()),
            symbol,
            time,
            priority,
            color: (*color.0, *color.1, *color.2),
            bg_color: (*bg_color.0, *bg_color.1, *bg_color.2),
            size,
        }
    }

    pub fn format(&self, max_len: usize) -> String {
        format!("{symbol}   {name:l$}  │ {size:b$} │  {time}",
            b=20, l=max_len, symbol=format!("{}", self.symbol), name=self.name.truecolor(self.color.0, self.color.1, self.color.2)
                .on_truecolor(self.bg_color.0, self.bg_color.1, self.bg_color.2), size=self.size, time=self.time.format(DATE_FORMAT_STR))
    }

    pub fn header(max_len: usize) -> String {
        format!("{}", format!("{symbol}    {name:l$}    {size:b$}    {time}  ",
            l=max_len, b=20, symbol=" ", name="Filename", size="Bytes", time="Last time modified").truecolor(0,0,0).on_truecolor(255,255,255))
    }
}

impl Ord for DirectoryObject {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.priority > other.priority {
            Ordering::Greater
        } else if self.priority < other.priority {
            Ordering::Less
        } else {
            self.name.cmp(&other.name)
        }
    }
}

impl PartialOrd for DirectoryObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

pub struct Collector {
    pub dir_objects: Vec<DirectoryObject>,
}

impl Collector {

    pub fn new() -> Self {
        Self {
            dir_objects: Vec::new(),
        }
    }
    
    pub fn sort(&mut self) {
        self.dir_objects.sort_by(|a, b| a.cmp(b));
    }

}