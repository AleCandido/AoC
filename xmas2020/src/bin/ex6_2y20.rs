use std::collections::HashSet;
use std::iter::FromIterator;

use aocinput::request;

fn main() {
    let resp = request::get_input(2020, 6);
    let groups: Vec<String> = resp.trim().split("\n\n").map(|x| x.to_owned()).collect();

    let mut sum = 0;

    for group in groups.iter() {
        let answers: Vec<HashSet<_>> = group
            .trim()
            .split("\n")
            .map(|x| HashSet::from_iter(x.as_bytes().to_vec().iter().cloned()))
            .collect();

        sum += answers[1..]
            .iter()
            .fold(answers[0].clone(), |a, b| {
                a.intersection(&b).cloned().collect()
            })
            .len();
    }
    println!("sum: {}", sum);
}
