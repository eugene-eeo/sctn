use std::env;
use std::collections::{HashMap,BTreeMap};

fn build_map(string: String) -> HashMap<String,usize> {
    let mut map = HashMap::new();
    for (idx, line) in string.split('\n').enumerate() {
        map.insert(line.to_string(), idx);
    }
    return map;
}

fn intersections(string: String,
                 mut map: HashMap<String,usize>) -> HashMap<String,usize> {
    for line in string.split('\n') {
        if !map.contains_key(line) {
            map.remove(line);
        }
    }
    return map;
}

fn order_map(map: HashMap<String,usize>) -> BTreeMap<usize,String> {
    let mut strs = BTreeMap::new();
    for (line, idx) in map {
        strs.insert(idx, line);
    }
    return strs;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let first = match args.get(2) {
        Some(v) => v.to_string(),
        None    => return,
    };

    let (_, rest) = args.split_at(2);
    let mut map   = build_map(first);
    for string in rest {
        map = intersections(string.to_owned(), map);
    }
    let lines = order_map(map);
    for line in lines.values() {
        println!("{}", line);
    }
}
