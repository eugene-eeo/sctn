use std::env;
use std::collections::HashSet;


fn build_map(string: String) -> HashSet<String> {
    let mut set = HashSet::new();
    for s in string.split('\n') {
        set.insert(s.to_string());
    }
    return set;
}

fn intersections(string: String,
                 mut set: HashSet<String>) -> HashSet<String> {
    for s in string.split('\n') {
        if !set.contains(s) {
            set.remove(s);
        }
    }
    return set;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let first = match args.get(2) {
        Some(v) => v.to_string(),
        None    => return,
    };

    let (_, rest) = args.split_at(2);
    match rest.to_vec().pop() {
        Some(text) => {
            let mut set = build_map(first);
            for string in rest {
                set = intersections(string.to_owned(), set);
            }
            for line in text.split('\n') {
                if set.contains(line) {
                    println!("{}", line);
                }
            }
        },
        None => return
    }
}
