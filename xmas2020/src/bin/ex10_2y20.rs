use std::collections::HashMap;

use ndarray::prelude::*;

use aocinput::request;

fn get_bit_at(input: u32, n: u8) -> u8 {
    if n < 32 {
        if input & (1 << n) != 0 {
            1
        } else {
            0
        }
    } else {
        0
    }
}

fn bits_repr(n: u32) -> Vec<u8> {
    let mut repr = Vec::new();
    let max = (n as f64 + 1.).log2().ceil() as u8;

    for i in 0..max {
        repr.push(get_bit_at(n, i));
    }

    repr
}

fn combinations(n: usize, cache: &mut HashMap<usize, u64>) -> u64 {
    if cache.contains_key(&n) {
        *cache.get(&n).unwrap()
    } else {
        let mut combs = 0;

        for i in 0..u32::pow(2, n as u32) {
            let mut count = 0;
            let mut valid = true;

            for bit in bits_repr(i) {
                if bit == 0 {
                    count += 1;
                } else {
                    count = 0;
                }
                if count > 1 {
                    valid = false;
                    break;
                }
            }

            if valid {
                combs += 1
            };
        }
        cache.insert(n, combs);
        println!("{:#?}", cache);
        combs
    }
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

    let mut gaps: Vec<usize> = Vec::new();
    let mut count = 0;

    for d in diffs.iter() {
        if *d == 1 {
            count += 1;
        } else if *d == 3 {
            if count != 0 {
                gaps.push(count);
            }
            count = 0;
        } else {
            panic!("Diff not expected: {}", d);
        }
    }

    let mut cache: HashMap<usize, u64> = HashMap::new();

    println!(
        "combinations: {}",
        gaps.iter()
            .map(|n| combinations(*n - 1, &mut cache))
            .product::<u64>()
    );
}
