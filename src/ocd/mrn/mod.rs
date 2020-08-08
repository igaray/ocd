extern crate clap;
extern crate dialoguer;
extern crate glob;
extern crate walkdir;

mod lexer;
mod parser;

use self::dialoguer::Confirmation;
use self::walkdir::WalkDir;
use crate::ocd::config;
use crate::ocd::config::{Mode, Verbosity};
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, PartialEq)]
pub enum Position {
    End,
    Index { value: usize },
}

#[derive(Debug, PartialEq)]
pub enum Rule {
    LowerCase,
    UpperCase,
    TitleCase,
    SentenceCase,
    CamelCaseJoin,
    CamelCaseSplit,
    Replace { pattern: String, replace: String },
    ReplaceSpaceDash,
    ReplaceSpacePeriod,
    ReplaceSpaceUnder,
    ReplaceDashPeriod,
    ReplaceDashSpace,
    ReplaceDashUnder,
    ReplacePeriodDash,
    ReplacePeriodSpace,
    ReplacePeriodUnder,
    ReplaceUnderDash,
    ReplaceUnderPeriod,
    ReplaceUnderSpace,
    Sanitize,
    PatternMatch { pattern: String, replace: String },
    ExtensionAdd { extension: String },
    ExtensionRemove,
    Insert { text: String, position: Position },
    InteractiveTokenize,
    InteractivePatternMatch,
    Delete { from: usize, to: Position },
}

#[derive(Clone, Debug)]
pub struct MassRenameConfig {
    pub verbosity: Verbosity,
    pub mode: Mode,
    pub dir: PathBuf,
    pub dryrun: bool,
    pub git: bool,
    pub recurse: bool,
    pub undo: bool,
    pub yes: bool,
    pub glob: Option<String>,
    pub rules_raw: Option<String>,
}

impl MassRenameConfig {
    pub fn new() -> MassRenameConfig {
        MassRenameConfig {
            verbosity: Verbosity::Low,
            mode: Mode::Files,
            dir: PathBuf::new(),
            dryrun: true,
            git: false,
            recurse: false,
            undo: false,
            yes: false,
            glob: None,
            rules_raw: None,
        }
    }

    pub fn with_args(&self, matches: &clap::ArgMatches) -> MassRenameConfig {
        fn glob_value(glob: Option<&str>) -> Option<String> {
            match glob {
                Some(glob_input) => Some(String::from(glob_input)),
                None => None,
            }
        }

        fn rules_value(matches: &clap::ArgMatches) -> Option<String> {
            let rules = matches.value_of("rules");
            match rules {
                Some(rules_input) => Some(rules_input.to_string()),
                None => None,
            }
        }

        MassRenameConfig {
            verbosity: config::verbosity_value(matches),
            mode: config::mode_value(matches.value_of("mode").unwrap()),
            dir: config::directory_value(matches.value_of("dir").unwrap()),
            dryrun: matches.is_present("dry-run"),
            git: matches.is_present("git"),
            recurse: matches.is_present("recurse"),
            undo: matches.is_present("undo"),
            yes: matches.is_present("yes"),
            glob: glob_value(matches.value_of("glob")),
            rules_raw: rules_value(matches),
        }
    }
}

pub fn run(config: &MassRenameConfig) -> Result<(), &str> {
    let rules_raw = config.rules_raw.clone().unwrap();
    let tokens = crate::ocd::mrn::lexer::tokenize(&config, &rules_raw)?;
    let rules = crate::ocd::mrn::parser::parse(&config, &tokens)?;
    let files = entries(&config)?;

    if let Verbosity::Debug = config.verbosity {
        println!("{:#?}", &config);
        println!("Tokens:\n{:#?}", &tokens);
        println!("Rules:\n{:#?}", &rules);
        println!("Files:\n{:#?}", &files);
    }

    let buffer = apply_rules(&config, &rules, &files)?;
    if config.yes || user_confirm() {
        execute_rules(&config, &buffer)
    } else {
        Ok(())
    }
}

fn entries(config: &MassRenameConfig) -> Result<Vec<PathBuf>, &'static str> {
    /*
    recurse | glob | mode
    F       | none | f
    F       | none | m
    F       | none | b
    F       | some | f
    F       | some | m
    F       | some | b
    T       | none | f
    T       | none | m
    T       | none | b
    T       | some | f
    T       | some | m
    T       | some | b
    */
    let mut entries_vec: Vec<PathBuf> = Vec::new();

    match (config.recurse, &config.glob, &config.mode) {
        (false, None, Mode::Files) => match fs::read_dir(&config.dir) {
            Ok(iterator) => {
                for entry in iterator {
                    match entry {
                        Ok(file) => {
                            if file.file_type().unwrap().is_file() {
                                entries_vec.push(file.path());
                            }
                        }
                        Err(_err) => return Err("Error while listing files"),
                    }
                }
            }
            Err(_err) => return Err("Error while listing files"),
        },
        (false, None, Mode::Directories) => match fs::read_dir(&config.dir) {
            Ok(iterator) => {
                for entry in iterator {
                    match entry {
                        Ok(file) => {
                            if file.file_type().unwrap().is_dir() {
                                entries_vec.push(file.path());
                            }
                        }
                        Err(_err) => return Err("Error while listing files"),
                    }
                }
            }
            Err(_err) => return Err("Error while listing files"),
        },
        (false, None, Mode::All) => match fs::read_dir(&config.dir) {
            Ok(iterator) => {
                for entry in iterator {
                    match entry {
                        Ok(file) => {
                            entries_vec.push(file.path());
                        }
                        Err(_err) => return Err("Error while listing files"),
                    }
                }
            }
            Err(_err) => return Err("Error while listing files"),
        },
        (true, None, Mode::Files) => {
            let iter = WalkDir::new(&config.dir).into_iter();
            for entry in iter {
                match entry {
                    Ok(entry) => {
                        if entry.file_type().is_file() {
                            entries_vec.push(entry.path().to_path_buf());
                        }
                    }
                    Err(_err) => return Err("Error listing files"),
                }
            }
        }
        (true, None, Mode::Directories) => {
            let iter = WalkDir::new(&config.dir).into_iter();
            for entry in iter {
                match entry {
                    Ok(entry) => {
                        if entry.file_type().is_dir() {
                            entries_vec.push(entry.path().to_path_buf());
                        }
                    }
                    Err(_err) => return Err("Error listing files"),
                }
            }
        }
        (true, None, Mode::All) => {
            let iter = WalkDir::new(&config.dir).into_iter();
            for entry in iter {
                entries_vec.push(entry.unwrap().path().to_path_buf());
            }
        }
        (_, Some(ref glob_input), Mode::Files) => {
            let mut path = config.dir.clone();
            path.push(glob_input);
            let glob_path = path.as_path().to_str().unwrap();
            for entry in glob::glob(glob_path).unwrap().filter_map(Result::ok) {
                let metadata = fs::metadata(&entry).unwrap();
                if metadata.is_file() {
                    entries_vec.push(entry);
                }
            }
        }
        (_, Some(ref glob_input), Mode::Directories) => {
            let mut path = config.dir.clone();
            path.push(glob_input);
            let glob_path = path.as_path().to_str().unwrap();
            for entry in glob::glob(glob_path).unwrap().filter_map(Result::ok) {
                let metadata = fs::metadata(&entry).unwrap();
                if metadata.is_dir() {
                    entries_vec.push(entry);
                }
            }
        }
        (_, Some(ref glob_input), Mode::All) => {
            let mut path = config.dir.clone();
            path.push(glob_input);
            let glob_path = path.as_path().to_str().unwrap();
            for entry in glob::glob(glob_path).unwrap().filter_map(Result::ok) {
                entries_vec.push(entry);
            }
        }
    }
    Ok(entries_vec)
}

fn apply_rules(
    _config: &MassRenameConfig,
    rules: &[Rule],
    files: &[PathBuf],
) -> Result<BTreeMap<PathBuf, PathBuf>, &'static str> {
    let mut buffer = new_buffer(files);

    println!("Applying rules...");
    for rule in rules {
        for dst in buffer.values_mut() {
            let dst2 = dst.clone();
            if let Some(filename) = dst2.file_stem() {
                match dst2.extension() {
                    Some(extension) => {
                        println!("filename: {:?} extension: {:?}", filename, extension);
                        let extension = extension.to_str();
                        let extension = extension.unwrap();
                        let filename = filename.to_str().unwrap();
                        println!("    from: {:?}", filename);
                        let filename = apply_rule(&rule, &filename);
                        dst.set_file_name(filename);
                        dst.set_extension(extension);
                        println!("    to:   {:?}", dst);
                    }
                    None => {
                        println!("filename: {:?}", filename);
                        let filename = filename.to_str().unwrap();
                        println!("    from: {:?}", filename);
                        let filename = apply_rule(&rule, &filename);
                        dst.set_file_name(filename);
                        println!("    to:   {:?}", dst);
                    }
                }
            }
        }
    }

    println!("Result:");
    print_buffer(&buffer);

    Ok(buffer)
}

fn apply_rule(rule: &Rule, filename: &str) -> String {
    match rule {
        Rule::LowerCase => apply_lower_case(filename),
        Rule::UpperCase => apply_upper_case(filename),
        Rule::TitleCase => apply_title_case(filename),
        Rule::SentenceCase => apply_sentence_case(filename),
        Rule::CamelCaseJoin => apply_camel_case_join(filename),
        Rule::CamelCaseSplit => apply_camel_case_split(filename),
        Rule::Sanitize => apply_sanitize(filename),
        Rule::Replace { pattern, replace } => apply_replace(filename, pattern, replace),
        Rule::ReplaceSpaceDash => apply_replace(filename, " ", "-"),
        Rule::ReplaceSpacePeriod => apply_replace(filename, " ", "."),
        Rule::ReplaceSpaceUnder => apply_replace(filename, " ", "_"),
        Rule::ReplaceDashPeriod => apply_replace(filename, "-", "."),
        Rule::ReplaceDashSpace => apply_replace(filename, "-", " "),
        Rule::ReplaceDashUnder => apply_replace(filename, "-", "_"),
        Rule::ReplacePeriodDash => apply_replace(filename, ".", "-"),
        Rule::ReplacePeriodSpace => apply_replace(filename, ".", " "),
        Rule::ReplacePeriodUnder => apply_replace(filename, ".", "_"),
        Rule::ReplaceUnderDash => apply_replace(filename, "_", "-"),
        Rule::ReplaceUnderPeriod => apply_replace(filename, "_", "."),
        Rule::ReplaceUnderSpace => apply_replace(filename, "_", " "),
        Rule::PatternMatch { pattern, replace } => apply_pattern_match(filename, pattern, replace),
        Rule::ExtensionAdd { extension } => apply_extension_add(filename, extension),
        Rule::ExtensionRemove => apply_extension_remove(filename),
        Rule::Insert { text, position } => apply_insert(filename, text, position),
        Rule::InteractiveTokenize => apply_interactive_tokenize(filename),
        Rule::InteractivePatternMatch => apply_interactive_pattern_match(filename),
        Rule::Delete { from, to } => apply_delete(filename, *from, to),
    }
}

fn apply_lower_case(filename: &str) -> String {
    filename.to_lowercase()
}

fn apply_upper_case(filename: &str) -> String {
    filename.to_uppercase()
}

fn apply_title_case(filename: &str) -> String {
    voca_rs::case::title_case(filename)
}

fn apply_sentence_case(filename: &str) -> String {
    voca_rs::case::capitalize(filename, true)
}

fn apply_camel_case_join(_filename: &str) -> String {
    unimplemented!()
}

fn apply_camel_case_split(_filename: &str) -> String {
    unimplemented!()
}

fn apply_sanitize(_filename: &str) -> String {
    unimplemented!()
}

fn apply_replace(filename: &str, pattern: &str, replace: &str) -> String {
    filename.replace(pattern, replace)
}

fn apply_pattern_match(_filename: &str, _pattern: &str, _replace: &str) -> String {
    unimplemented!()
}

fn apply_extension_add(_filename: &str, _extension: &str) -> String {
    unimplemented!()
}

fn apply_extension_remove(_filename: &str) -> String {
    unimplemented!()
}

fn apply_insert(filename: &str, text: &str, position: &Position) -> String {
    let mut new = String::from(filename);
    match position {
        Position::End => new.push_str(text),
        Position::Index { value: index } => new.insert_str(*index, text),
    }
    new
}

fn apply_interactive_tokenize(_filename: &str) -> String {
    unimplemented!()
}

fn apply_interactive_pattern_match(_filename: &str) -> String {
    unimplemented!()
}

fn apply_delete(filename: &str, from_idx: usize, to: &Position) -> String {
    let to_idx = match *to {
        Position::End => filename.len(),
        Position::Index { value } => {
            if value > filename.len() {
                filename.len()
            } else {
                value
            }
        }
    };
    let mut s = String::from(filename);
    s.replace_range(from_idx..to_idx, "");
    s
}

fn create_undo_file(buffer: &BTreeMap<PathBuf, PathBuf>) {
    println!("Creating undo script.");
    match File::create("./undo.sh") {
        Ok(mut output_file) => {
            for (src, dst) in buffer {
                match writeln!(output_file, "mv -i {:?} {:?}", dst, src) {
                    Ok(_) => {}
                    Err(reason) => {
                        eprintln!("Error writing to undo file: {:?}", reason);
                    }
                }
            }
        }
        Err(reason) => {
            eprintln!("Error creating undo file: {:?}", reason);
        }
    }
}

fn execute_rules(
    config: &MassRenameConfig,
    buffer: &BTreeMap<PathBuf, PathBuf>,
) -> Result<(), &'static str> {
    for (src, dst) in buffer {
        println!("Moving\n    {:?}\nto\n    {:?}", src, dst);
        if !config.dryrun {
            if config.undo {
                create_undo_file(buffer);
            }
            if config.git {
                let src = src.to_str().unwrap();
                let dst = dst.to_str().unwrap();
                let _output = Command::new("git")
                    .args(&["mv", src, dst])
                    .output()
                    .expect("Error invoking git.");
            // TODO: do something with output
            } else {
                match fs::rename(src, dst) {
                    Ok(_) => {}
                    Err(reason) => {
                        eprintln!("Error moving file: {:?}", reason);
                        return Err("Error moving file.");
                    }
                }
            }
        }
    }
    Ok(())
}

fn new_buffer(files: &[PathBuf]) -> BTreeMap<PathBuf, PathBuf> {
    let mut buffer = BTreeMap::new();
    for file in files {
        buffer.insert(file.clone(), file.clone());
    }
    buffer
}

fn print_buffer(buffer: &BTreeMap<PathBuf, PathBuf>) {
    for (src, dst) in buffer {
        println!("    {:?} => {:?}", src, dst)
    }
}

fn user_confirm() -> bool {
    match Confirmation::new()
        .with_text("Do you want to continue?")
        .interact()
    {
        Ok(cont) => cont,
        Err(_) => false,
    }
}

#[cfg(test)]
mod test {
    use crate::ocd::mrn::apply_camel_case_join;
    use crate::ocd::mrn::apply_camel_case_split;
    use crate::ocd::mrn::apply_lower_case;
    use crate::ocd::mrn::apply_replace;
    use crate::ocd::mrn::apply_sentence_case;
    use crate::ocd::mrn::apply_title_case;
    use crate::ocd::mrn::apply_upper_case;
    use crate::ocd::mrn::Position;
    // use ocd::mrn::apply_sanitize;
    use crate::ocd::mrn::apply_delete;
    use crate::ocd::mrn::apply_insert;
    use crate::ocd::mrn::apply_pattern_match;

    #[test]
    fn lower_case_test() {
        assert_eq!(apply_lower_case("LoWeRcAsE"), "lowercase")
    }

    #[test]
    fn upper_case_test() {
        assert_eq!(apply_upper_case("UpPeRcAsE"), "UPPERCASE")
    }

    #[test]
    fn title_case_test() {
        assert_eq!(
            apply_title_case("a title has multiple words"),
            "A Title Has Multiple Words"
        );
        assert_eq!(
            apply_title_case("A TITLE HAS MULTIPLE WORDS"),
            "A Title Has Multiple Words"
        )
    }

    #[test]
    fn sentence_case_test() {
        assert_eq!(
            apply_sentence_case("a sentence has multiple words"),
            "A sentence has multiple words"
        );
        assert_eq!(
            apply_sentence_case("A SENTENCE HAS MULTIPLE WORDS"),
            "A sentence has multiple words"
        );
        assert_eq!(
            apply_sentence_case("A sEnTeNcE HaS mUlTiPlE wOrDs"),
            "A sentence has multiple words"
        )
    }

    #[test]
    fn camel_case_join_test() {
        assert_eq!(apply_camel_case_join("Camel case Join"), "CamelCaseJoin")
    }

    #[test]
    fn camel_case_split_test() {
        assert_eq!(apply_camel_case_split("CamelCaseSplit"), "Camel Case Split")
    }

    #[test]
    fn replace_test() {
        assert_eq!(apply_replace("aa bbccdd ee", "cc", "ff"), "aa bbffdd ee")
    }

    #[test]
    fn replace_space_dash_test() {
        assert_eq!(apply_replace("aa bb cc dd", " ", "-"), "aa-bb-cc-dd")
    }

    #[test]
    fn replace_space_period_test() {
        assert_eq!(apply_replace("aa bb cc dd", " ", "."), "aa.bb.cc.dd")
    }

    #[test]
    fn replace_space_under_test() {
        assert_eq!(apply_replace("aa bb cc dd", " ", "_"), "aa_bb_cc_dd")
    }

    #[test]
    fn replace_dash_period_test() {
        assert_eq!(apply_replace("aa-bb-cc-dd", "-", "."), "aa.bb.cc.dd")
    }

    #[test]
    fn replace_dash_space_test() {
        assert_eq!(apply_replace("aa-bb-cc-dd", "-", " "), "aa bb cc dd")
    }

    #[test]
    fn replace_dash_under_test() {
        assert_eq!(apply_replace("aa-bb-cc-dd", "-", "_"), "aa_bb_cc_dd")
    }

    #[test]
    fn replace_period_dash_test() {
        assert_eq!(apply_replace("aa.bb.cc.dd", ".", "-"), "aa-bb-cc-dd")
    }

    #[test]
    fn replace_period_space_test() {
        assert_eq!(apply_replace("aa.bb.cc.dd", ".", " "), "aa bb cc dd")
    }

    #[test]
    fn replace_period_under_test() {
        assert_eq!(apply_replace("aa.bb.cc.dd", ".", "_"), "aa_bb_cc_dd")
    }

    #[test]
    fn replace_under_dash_test() {
        assert_eq!(apply_replace("aa_bb_cc_dd", "_", "-"), "aa-bb-cc-dd")
    }

    #[test]
    fn replace_under_period_test() {
        assert_eq!(apply_replace("aa_bb_cc_dd", "_", "."), "aa.bb.cc.dd")
    }

    #[test]
    fn replace_under_space_test() {
        assert_eq!(apply_replace("aa_bb_cc_dd", "_", " "), "aa bb cc dd")
    }

    #[test]
    fn sanitize_test() {
        panic!("Not implemented!")
    }

    #[test]
    fn pattern_match_test() {
        assert_eq!(apply_pattern_match("aa bb", "{X} {X}", "{2} {1}"), "bb aa");
        panic!("Not implemented!")
    }

    #[test]
    fn insert_test() {
        assert_eq!(apply_insert("aa bb", " cc", &Position::End), "aa bb cc");
        assert_eq!(
            apply_insert("aa bb", " cc", &Position::Index { value: 2 }),
            "aa cc bb"
        );
        assert_eq!(
            apply_insert("aa bb", "cc ", &Position::Index { value: 0 }),
            "cc aa bb"
        );
    }

    #[test]
    fn delete_test() {
        assert_eq!(apply_delete("aa bb cc", 0, &Position::End), "");
        assert_eq!(
            apply_delete("aa bb cc", 0, &Position::Index { value: 3 }),
            "bb cc"
        );
        assert_eq!(
            apply_delete("aa bb cc", 0, &Position::Index { value: 42 }),
            ""
        );
    }
}
