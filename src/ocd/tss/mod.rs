use crate::ocd::config::{directory_value, verbosity_value, Verbosity};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::io;
use std::option;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct TimeStampSortConfig {
    pub verbosity: Verbosity,
    pub dir: PathBuf,
    pub dryrun: bool,
    pub undo: bool,
    pub yes: bool,
}

impl TimeStampSortConfig {
    pub fn new() -> TimeStampSortConfig {
        TimeStampSortConfig {
            verbosity: Verbosity::Low,
            dir: PathBuf::new(),
            dryrun: true,
            undo: false,
            yes: false,
        }
    }

    pub fn with_args(&self, matches: &clap::ArgMatches) -> TimeStampSortConfig {
        TimeStampSortConfig {
            verbosity: verbosity_value(matches),
            dir: directory_value(matches.value_of("dir").unwrap()),
            dryrun: matches.is_present("dry-run"),
            undo: matches.is_present("undo"),
            yes: matches.is_present("yes"),
        }
    }
}

pub fn run(config: &TimeStampSortConfig) -> Result<(), String> {
    for entry in WalkDir::new(&config.dir) {
        process_entry(config, &entry.unwrap().path())
    }

    if !config.dryrun && config.undo {
        crate::ocd::output::undo_script(config.verbosity);
        // TODO: implement undo
    }
    Ok(()) // TODO change this so that it can return the error if something goes wrong.
}

fn process_entry(config: &TimeStampSortConfig, entry: &Path) {
    if !entry.is_dir() {
        if let Some(destination) = destination(&config.dir, &entry) {
            match create_directory(config, &destination) {
                Ok(_) => match move_file(config, &entry, &destination) {
                    Ok(_) => {}
                    Err(reason) => {
                        crate::ocd::output::file_move_error(config.verbosity, entry, &reason);
                    }
                },
                Err(reason) => crate::ocd::output::create_directory_error(
                    config.verbosity,
                    destination,
                    &reason,
                ),
            }
        }
    }
}

fn destination(base_dir: &Path, file_name: &Path) -> option::Option<PathBuf> {
    // let file = std::fs::File::open(file_name).unwrap();
    // let reader = exif::Reader::new(&mut std::io::BufReader::new(&file)).unwrap();
    // for f in reader.fields() {
    //     f.tag, f.thumbnail, f.value.display_as(f.tag));
    // }
    file_name
        .to_str()
        .and_then(date)
        .map(|(year, month, day)| base_dir.join(format!("{}-{}-{}", year, month, day)))
}

fn date(filename: &str) -> Option<(&str, &str, &str)> {
    lazy_static! {
        // YYYY?MM?DD or YYYYMMDD,
        // where YYYY in [2000-2019], MM in [01-12], DD in [01-31]
        static ref RE: Regex = Regex::new(r"\D*(20[01]\d).?(0[1-9]|1[012]).?(0[1-9]|[12]\d|30|31)\D*").unwrap();
    }
    RE.captures(filename).map(|captures| {
        let year = captures.get(1).unwrap().as_str();
        let month = captures.get(2).unwrap().as_str();
        let day = captures.get(3).unwrap().as_str();
        (year, month, day)
    })
}

fn create_directory(config: &TimeStampSortConfig, directory: &Path) -> io::Result<()> {
    if !config.dryrun {
        let mut full_path = PathBuf::new();
        full_path.push(directory);
        match fs::create_dir(&full_path) {
            Ok(_) => Ok(()),
            Err(reason) => match reason.kind() {
                io::ErrorKind::AlreadyExists => Ok(()),
                _ => {
                    crate::ocd::output::create_directory_error(
                        config.verbosity,
                        full_path,
                        &reason,
                    );
                    Err(reason)
                }
            },
        }
    } else {
        Ok(())
    }
}

fn move_file(config: &TimeStampSortConfig, from: &Path, dest: &Path) -> io::Result<()> {
    let mut to = PathBuf::new();
    to.push(dest);
    to.push(from.file_name().unwrap());

    crate::ocd::output::file_move(config.verbosity, from, &to);

    if !config.dryrun {
        match fs::rename(from, to) {
            Ok(_) => {
                if config.undo {
                    // TODO implement undo script
                }
                Ok(())
            }
            Err(reason) => {
                crate::ocd::output::rename_error(config.verbosity, from, &reason);
                Err(reason)
            }
        }
    } else {
        Ok(())
    }
}