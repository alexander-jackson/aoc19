use std::collections::HashSet;
use std::fs;

/// AAA)BBB means BBB orbits AAA

fn get_total_orbits(orbits: &Vec<Vec<&str>>, key: &str) -> u32 {
    let mut total: u32 = 0;
    let orbitters = get_direct_orbits(orbits, key);

    if orbitters.is_empty() {
        return 0;
    }

    for o in &orbitters {
        total += get_total_orbits(&orbits, o);
    }

    total + orbitters.len() as u32
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

    let total: u32 = keys.iter().map(|x| {
        dbg!(x); get_total_orbits(&orbits, x)
    }).sum();

    dbg!(total);
}
