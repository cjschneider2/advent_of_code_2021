fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part_1(input));
    println!("part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let (fwd, depth) = input
        .lines()
        .map(|cmd| {
            let mut itr = cmd.split_whitespace();
            (itr.next().unwrap(), itr.next().unwrap())
        })
        .fold((0, 0), |(fwd, depth), (cmd, val)| {
            let val = val.parse::<usize>().unwrap();
            match cmd {
                "forward" => (fwd + val, depth),
                "down" => (fwd, depth + val),
                "up" => (fwd, depth - val),
                _ => panic!("got unexpected: `{}`", cmd),
            }
        });
    fwd * depth
}

fn part_2(input: &str) -> usize {
    let (fwd, depth, _aim) = input
        .lines()
        .map(|cmd| {
            let mut itr = cmd.split_whitespace();
            (itr.next().unwrap(), itr.next().unwrap())
        })
        .fold((0, 0, 0), |(fwd, depth, aim), (cmd, val)| {
            let val = val.parse::<i64>().unwrap();
            match cmd {
                "forward" => (fwd + val, depth + (val * aim), aim),
                "down" => (fwd, depth, aim + val),
                "up" => (fwd, depth, aim - val),
                _ => panic!("got unexpected: `{}`", cmd),
            }
        });
    (fwd * depth) as usize
}
