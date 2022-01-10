mod collector;
mod init;

use std::{env, fs::{self}, io, ffi::{OsStr}, time::SystemTime};

use collector::{Collector, DirectoryObject};
use init::Config;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

const DATE_FORMAT_STR: &'static str = "%Y-%m-%d %H:%M:%S";

fn main() {
    let cwd = env::current_dir().unwrap();
    let bdir_config = &Config::new(env::current_exe().unwrap().parent().unwrap().join("bdir_config.json"));

    let entries = fs::read_dir(cwd).unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
    
    let mut objects = Collector::new();
    let mut longest_name: usize = 0;

    for entry in &entries {
        let mut symbol: char = '\u{2753}'; // ❓
        let mut priority = 0b01111111;
        let mut color = &bdir_config.other.color;
        let mut bg_color = &bdir_config.other.bg_color;
        if entry.is_dir() {
            symbol = bdir_config.dir.symbol;
            color = &bdir_config.dir.color;
            bg_color = &bdir_config.dir.bg_color;
            priority = 0;
        } else if entry.is_file() {
            symbol = bdir_config.other.symbol;
            for ft in &bdir_config.files {
                let endings = &ft.endings;
                let ext = entry.extension().unwrap_or(OsStr::new(""));
                if endings.contains(&ext.to_str().unwrap().to_string()) {
                    symbol = ft.symbol;
                    priority = ft.priority;
                    color = &ft.color;
                    bg_color = &ft.bg_color;
                    break;
                }
            }
        }
        let mut not_accesible: bool = false;
        let metadata = entry.as_path().metadata().unwrap_or_else(|_e| {
            let a = env::current_exe().unwrap();
            not_accesible = true;
            a.metadata().unwrap()
        });
        if not_accesible {
            objects.dir_objects.push(DirectoryObject::new(
                entry.file_name().unwrap().to_os_string(),
                symbol,
                1,
                SystemTime::now().into(),
                0,
                (&255, &25, &30),
                (&0, &0, &0),
            ));
        } else {
            objects.dir_objects.push(DirectoryObject::new(
                entry.file_name().unwrap().to_os_string(),
                symbol,
                priority,
                metadata.modified().unwrap().into(),
                metadata.len(),
                (color.get(0).unwrap(), color.get(1).unwrap(), color.get(2).unwrap()),
                (bg_color.get(0).unwrap(), bg_color.get(1).unwrap(), bg_color.get(2).unwrap()),
            ));
        }
        let l = entry.file_name().unwrap().to_os_string().len();
        if l > longest_name {
            longest_name = l;
        }
    }

    if !((env::args().collect::<Vec<String>>()).contains(&"-u".to_string())) {
        objects.sort();
    }

    let strings: Vec<String> = objects.dir_objects.iter().map(|o| o.format(longest_name)).collect();
    let mut longest_string: usize = 0;
    for s in &strings {
        if s.len() > longest_string {
            longest_string = s.len();
        }
    }

    println!("{}", DirectoryObject::header(longest_name));
    for s in strings {
        println!("{}", s);
    }
}