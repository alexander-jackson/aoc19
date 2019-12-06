use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn get_total_orbits<'a>(orbits: &Vec<Vec<&'a str>>, total_orbits: &mut HashMap<&'a str, u32>, key: &'a str) -> u32 {
    let mut total: u32 = 0;
    let orbitters = get_direct_orbits(orbits, key);

    if orbitters.is_empty() {
        return 0;
    }

    for o in &orbitters {
        if total_orbits.contains_key(o) {
            total += total_orbits.get(o).unwrap();
        } else {
            total += get_total_orbits(&orbits, total_orbits, o);
        }
    }

    let result: u32 = total + orbitters.len() as u32;
    total_orbits.insert(key, result);

    result
}

fn get_direct_orbits<'a>(orbits: &Vec<Vec<&'a str>>, key: &str) -> HashSet<&'a str> {
    let mut orbitters: HashSet<&str> = HashSet::new();

    for or in orbits {
        if or[0] == key {
            orbitters.insert(or[1]);
        }
    }

    orbitters
}

fn main() {
    let filename: &str = "input.txt";

    let mut keys: HashSet<&str> = HashSet::new();

    let data: Vec<String> = fs::read_to_string(filename)
        .expect("Failed to read the file")
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x))
        .collect();

    let orbits: Vec<Vec<&str>> = data
        .iter()
        .map(|x| x.split(")").collect::<Vec<&str>>())
        .collect();

    for o in &orbits {
        keys.insert(o[0]);
        keys.insert(o[1]);
    }

    let mut total_orbits: HashMap<&str, u32> = HashMap::new();

    let total: u32 = keys.iter().map(|x| {
        dbg!(x); get_total_orbits(&orbits, &mut total_orbits, x)
    }).sum();

    dbg!(total);
}
