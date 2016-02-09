use std::env;
use std::collections::HashSet;

fn build_map(string: &str) -> HashSet<&str> {
    let mut set = HashSet::new();
    for line in string.split('\n') {
        set.insert(line);
    }
    return set;
}

fn intersections<'a>(string: &str,
                     mut set: HashSet<&'a str>) -> HashSet<&'a str> {
    for line in string.split('\n') {
        if !set.contains(line) {
            set.remove(line);
        }
    }
    return set;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let first = match args.get(2) {
        Some(v) => v,
        None    => return,
    };

    let (_, rest) = args.split_at(2);
    let last = match rest.to_vec().pop() {
        Some(v) => v,
        None    => return,
    };

    let mut set = build_map(first);
    for string in rest {
        set = intersections(string, set);
    }
    for line in last.split('\n') {
        if set.contains(line) {
            println!("{}", line);
        }
    }
}
