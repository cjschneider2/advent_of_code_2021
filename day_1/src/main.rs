fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: `{}`", part_1(input));
    println!("Part 2: `{}`", part_2(input));
}

/// In part_1 we're finding the count of times that the depth has increased over
/// the previous depth measured, based on our input.
fn part_1(input: &str) -> usize {
    let (count, _last) = input.lines().map(|x| x.parse::<usize>().unwrap())
        .fold((0 /* count */, usize::MAX /* prev */), |mut state, depth| {
            if depth > state.1 {
                state.0 += 1;
            }
            state.1 = depth;
            state
        });
    count
}

/// Consider sums of a three-measurement sliding window.
/// How many sums are larger than the previous sum?
fn part_2(input: &str) -> usize {
    let nums: Vec<usize> = input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let (count, _last) = nums
        .iter()
        .scan((nums[0], nums[1], nums[2]), |win, &next| {
            win.0 = win.1;
            win.1 = win.2;
            win.2 = next;
            Some((win.0, win.1, win.2))
        })
        .map(|(a, b, c)| a + b + c)
        .fold((0, usize::MAX), |mut state, depth| {
            if depth > state.1 {
                state.0 += 1;
            }
            state.1 = depth;
            state
        });
    count
}
