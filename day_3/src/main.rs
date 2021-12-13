fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<&str> = input.lines().collect();
    println!("part 1: {}", part_1(input.clone()));
    println!("part 2: {}", part_2(input));
    part2();
}

fn part_1(input: Vec<&str>) -> i32 {
    // get the most common bit's mask
    let total = input.len();
    let mask = input.iter()
        .fold([0; 12], |mut cnt, line| {
            line.chars()
                .enumerate()
                .for_each(|(idx, chr)| {
                    if chr == '1' { cnt[idx] += 1; }
                });
            cnt
        })
        .map(|x| if total - x > x { 0 } else { 1 });
    // calculate the first part
    let (epsilon, gamma) =
        mask.iter()
            .fold((0, 0), |(mut e, mut g), &cnt| {
                let (de, dg) = if cnt == 0 { (0, 1) } else { (1, 0) };
                e = (e << 1) + de;
                g = (g << 1) + dg;
                (e, g)
            });
    epsilon * gamma
}

fn part_2(input: Vec<&str>) -> usize {
    // do the calculation, as a fn as we're going to basically do it twice
    fn calc(mut vec: Vec<&str>, invert: bool) -> usize {
        for idx in 0..12 {
            // get current bit count for that index
            // NOTE: this changes every iteration; I didn't notice this at first
            let mut count = (0, 0);
            for &num in vec.iter() {
                let num = num.as_bytes()[idx] as i32 - 48;
                if num == 0 { count.0 += 1; } else { count.1 += 1; }
            }
            let common = if invert {
                if count.0 <= count.1 { 0 } else { 1 }
            } else {
                if count.0 > count.1 { 0 } else { 1 }
            };
            // keep the values we want
            let mut count = vec.len();
            vec.retain(| &num | {
                let val = num.as_bytes()[idx] as i32 - 48;
                let matched = val == common;
                if !matched { count -= 1; }
                matched || count == 0
            })
        }
        assert_eq!(vec.len(), 1);
        vec[0].to_int()
    }

    let oxy = calc(input.clone(), false);
    let coo = calc(input, true);

    oxy * coo
}

// just a trait cause I can
trait BinStrToInt {
    fn to_int(&self) -> usize;
}

impl BinStrToInt for &str {
    fn to_int(&self) -> usize {
        self.bytes()
            .into_iter()
            .fold(0, |acc, b| {
                (acc << 1) + (b - '0' as u8) as usize
            })
    }
}
