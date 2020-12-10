use aocinput::request;

fn is_sum(n: &usize, window: &[usize]) -> bool {
    let mut window = window.clone().to_vec();
    window.sort();
    for m in window.iter() {
        if 2 * m > *n {
            return false;
        }
        if window.contains(&(*n - m)) {
            return true;
        }
    }
    //println!("{:#?}, len: {}", window, window.len());
    false
}

fn main() {
    let resp = request::get_input(2020, 9);
    let nums: Vec<usize> = resp
        .trim()
        .split("\n")
        .map(|instr| instr.parse().unwrap())
        .collect();

    for (i, n) in nums[25..].iter().enumerate() {
        if !is_sum(&n, &nums[i..(i + 25)]) {
            println!("{:3}, {:15}", i, n);
        }
    }
}
