use std::env;
use std::collections::HashSet;

type StrHash<'a> = HashSet<&'a str>;

fn build_set(string: &str) -> StrHash {
    let mut set = HashSet::new();
    for line in string.split('\n') {
        set.insert(line);
    }
    return set;
}

fn intersections<'a>(string: &str, mut set: StrHash<'a>) -> StrHash<'a> {
    for line in string.split('\n') {
        if !set.contains(line) {
            set.remove(line);
        }
    }
    return set;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return;
    };

    let ref first = args[1];
    let (_, rest) = args.split_at(2);
    let last = rest.to_vec().pop().unwrap();

    let mut set: StrHash = build_set(&first);
    for string in rest {
        set = intersections(string, set);
    }
    for line in last.split('\n') {
        if set.contains(line) {
            println!("{}", line);
        }
    }
}
