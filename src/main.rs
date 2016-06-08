use std::env;
use std::collections::BTreeSet;

type StrHash<'a> = BTreeSet<&'a str>;

fn build_index(string: &str) -> StrHash {
    string.split('\n').collect()
}

fn intersection<'a>(string: &'a str, index: &StrHash<'a>) -> StrHash<'a> {
    build_index(string)
        .intersection(&index)
        .map(|x| *x)
        .collect()
}

fn main() {
    let mut args: Vec<String> = env::args()
        .skip(1)
        .collect();
    if args.len() < 2 {
        return;
    }

    let last    = args.pop().unwrap();
    let initial = args.pop().unwrap();

    let mut index = build_index(&initial);
    for string in &args {
        index = intersection(string, &index);
    }
    for line in last.split('\n') {
        if index.contains(line) {
            println!("{}", line);
        }
    }
}
