use std::env;
use std::collections::BTreeSet;

type StrHash<'a> = BTreeSet<&'a str>;

fn build_index(string: &str) -> StrHash {
    string.split('\n').collect()
}

fn intersection<'a>(string: &'a str, index: &StrHash<'a>) -> StrHash<'a> {
    let new = build_index(string);
    index.intersection(&new)
         .map(|x| *x)
         .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return;
    }

    let ref first = args.get(1).unwrap();
    let (_, rest) = args.split_at(2);
    let mut rest = rest.to_vec();
    let last = rest.pop().unwrap();

    let mut index = build_index(first);
    for string in &rest {
        index = intersection(string, &index);
    }
    for line in last.split('\n') {
        if index.contains(line) {
            println!("{}", line);
        }
    }
}
