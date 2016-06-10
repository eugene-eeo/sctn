extern crate argparse;

use std::collections::BTreeSet;
use argparse::{ArgumentParser, Store, List};

type StrSet<'a> = BTreeSet<&'a str>;

fn build_index(string: &str) -> StrSet {
    string.split('\n').collect()
}

fn intersection<'a>(string: &'a str, index: &StrSet<'a>) -> StrSet<'a> {
    build_index(string)
        .intersection(&index)
        .map(|&x| x)
        .collect()
}

fn main() {
    let mut first: String = "".to_string();
    let mut strings: Vec<String> = vec![];
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Intersect outputs.");
        ap.refer(&mut first)
          .add_argument("first", Store, "string");
        ap.refer(&mut strings)
          .add_argument("strings", List, "strings to intersect");
        ap.parse_args_or_exit();
    }
    let last = match strings.pop() {
        Some(v) => v,
        None    => return
    };
    let mut index = build_index(&first);
    for string in &strings {
        index = intersection(string, &index);
    }
    for line in last.split('\n') {
        if index.contains(line) {
            println!("{}", line);
        }
    }
}
