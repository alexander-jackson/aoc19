use std::fs;

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Coordinate,
    end: Coordinate,
}

fn get_wire(line: &str) -> Vec<Line> {
    let mut wire: Vec<Line> = Vec::new();
    let mut curr: Coordinate = Coordinate { x: 0, y: 0 };
    let movements: Vec<(char, i32)> = line.split(',')
        .map(|x| {
            let mut chars = x.chars();
            (chars.next().unwrap(), chars.collect::<String>().parse().unwrap())
        })
        .collect();

    for m in &movements {
        let next: Coordinate = match m.0 {
            'U' => Coordinate { x: curr.x, y: curr.y + m.1 },
            'D' => Coordinate { x: curr.x, y: curr.y - m.1 },
            'L' => Coordinate { x: curr.x - m.1, y: curr.y },
            'R' => Coordinate { x: curr.x + m.1, y: curr.y },
            _ => panic!("Unexpected direction char"),
        };

        wire.push(Line { start: curr, end: next });
        curr = next;
    }

    wire
}

fn get_wires(filename: &str) -> (Vec<Line>, Vec<Line>) {
    // Read the file to a Vec<String>
    let file_lines: Vec<String> = fs::read_to_string(filename)
        .expect("Failed to read the problem input")
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x))
        .collect();

    (get_wire(&file_lines[0]), get_wire(&file_lines[1]))
}

fn main() {
    let (wire_a, wire_b): (Vec<Line>, Vec<Line>) = get_wires("input.txt");

    dbg!((&wire_a, &wire_b));
}
