use std::collections::HashMap;

fn main() {
    let resp = "1,0,18,10,19,6";

    let mut history: HashMap<usize, usize> = resp
        .split(",")
        .enumerate()
        .map(|(i, n)| (n.parse().unwrap(), i + 1))
        .collect();
    let l = history.len();

    let mut last: usize = resp.split(",").last().unwrap().parse().unwrap();
    for i in l..30000000 {
        let last_time = history.entry(last).or_insert(i);
        last = i - *last_time;
        *last_time = i;
    }

    println!("{}", last);
}
