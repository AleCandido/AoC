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

fn find_invalid(nums: &Vec<usize>) -> usize {
    for (i, n) in nums[25..].iter().enumerate() {
        if !is_sum(&n, &nums[i..(i + 25)]) {
            return *n;
        }
    }
    return 0;
}

fn window_sum(nums: &Vec<usize>, min_idx: usize, max_idx: usize) -> usize {
    return nums[min_idx..(max_idx + 1)].iter().sum::<usize>();
}

fn main() {
    let resp = request::get_input(2020, 9);
    let nums: Vec<usize> = resp
        .trim()
        .split("\n")
        .map(|instr| instr.parse().unwrap())
        .collect();

    let invalid = find_invalid(&nums);
    let mut min_idx = 0;
    let mut max_idx = 1;
    loop {
        let window_sum = window_sum(&nums, min_idx, max_idx);
        if window_sum == invalid {
            break;
        } else if window_sum < invalid {
            max_idx += 1;
        } else {
            min_idx += 1;
        }
    }
    println!(
        "{:#?}\n{:#?}, {} - {}",
        nums[min_idx..(max_idx + 1)].to_vec(),
        nums[min_idx..(max_idx + 1)].to_vec().iter().sum::<usize>(),
        nums[min_idx],
        nums[max_idx],
    );

    let result_window = nums[min_idx..(max_idx + 1)].to_vec();
    println!(
        "boundaries sum: {}",
        result_window.iter().min().unwrap() + result_window.iter().max().unwrap()
    );
}
