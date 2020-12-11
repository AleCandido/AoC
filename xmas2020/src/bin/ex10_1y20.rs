use ndarray::prelude::*;

use aocinput::request;

fn count_elems(a: &Array1<usize>, e: usize) -> usize {
    a.iter().filter(|&n| *n == e).count()
}

fn main() {
    let resp = request::get_input(2020, 10);
    let mut adapters_v: Vec<usize> = resp
        .trim()
        .split("\n")
        .map(|adap| adap.parse().unwrap())
        .collect();
    adapters_v.sort();
    adapters_v.insert(0, 0);
    adapters_v.push(adapters_v.last().unwrap() + 3);

    let adapters = Array::from(adapters_v);
    let diffs = &adapters.slice(s![1..]) - &adapters.slice(s![..(adapters.len() - 1)]);

    println!(
        "1: {}\n3: {}\nprod: {}",
        count_elems(&diffs, 1),
        count_elems(&diffs, 3),
        count_elems(&diffs, 1) * count_elems(&diffs, 3),
    );
}
