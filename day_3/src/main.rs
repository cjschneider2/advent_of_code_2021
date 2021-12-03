fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part_1(input));
    println!("part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let (total, ones_counts) = input
        .lines()
        .fold((0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), |(mut total, mut cnt), line| {
            line.chars()
                .enumerate()
                .for_each(|(idx, chr)| {
                    if chr == '1' { cnt[idx] += 1; }
                });
            total += 1;
            (total, cnt)
        });
    let (epsilon, gamma) = ones_counts
        .iter()
        .fold((0, 0), |(mut e, mut g), &cnt| {
            let (de, dg) = if total - cnt > cnt {
                (0, 1) // more zeros
            } else {
                (1, 0) // more ones
            };
            e = (e << 1) + de;
            g = (g << 1) + dg;
            (e, g)
        });
    epsilon * gamma
}

fn part_2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let total = lines.len();
    let mask = lines
        .iter()
        .fold([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], |mut cnt, line| {
            line.chars()
                .enumerate()
                .for_each(|(idx, chr)| {
                    if chr == '1' { cnt[idx] += 1; }
                });
            cnt
        })
        .map(|x| if total - x > x { 0 } else { 1 });
    let oxy = mask.iter();
    let cdox = 0;
    oxy * cdox
}
