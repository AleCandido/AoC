use std::collections::HashMap;

use itertools::Itertools;

use aocinput::request;

#[derive(Clone, Debug)]
struct BitMask {
    ones: u64,
    floats: Vec<u8>,
}

impl BitMask {
    fn new() -> BitMask {
        BitMask {
            ones: 0,
            floats: Vec::new(),
        }
    }

    fn from_str(s: &str) -> BitMask {
        let mut ones = 0;
        let mut floats: Vec<u8> = Vec::new();
        for (i, c) in s.trim().chars().rev().enumerate() {
            match c {
                '0' => {}
                '1' => {
                    ones += 1 << i;
                }
                'X' => {
                    floats.push(i as u8);
                }
                a => {
                    panic!("Char not recognized '{}'", a)
                }
            }
        }
        BitMask { ones, floats }
    }

    fn apply(&self, n: u64) -> Vec<u64> {
        let mut v = Vec::new();
        v.push(n | self.ones);

        for i in self.floats.iter() {
            let w = v.clone();
            v = Vec::new();
            for m in w.iter() {
                let float_mask = 1 << i;
                v.push(m | float_mask);
                v.push(m & !float_mask);
            }
        }
        v
    }
}

fn main() {
    let resp = request::get_input(2020, 14);

    let mut memory = HashMap::new();
    let mut mask = BitMask::new();

    for line in resp.trim().split("\n") {
        if line.contains('[') {
            let (loc, value): (u64, u64) = line
                .trim()
                .split(|c| ['[', ']', '='].contains(&c))
                .filter(|s| ![" ", "mem"].contains(&s))
                .map(|s| s.trim().parse().unwrap())
                .collect_tuple()
                .unwrap();
            for loci in mask.apply(loc) {
                memory.insert(loci, value);
            }
        } else {
            mask = BitMask::from_str(line.split("=").collect::<Vec<&str>>()[1]);
        }
    }

    println!("{}", memory.values().sum::<u64>());
    //let value = 0;
    //let mask = BitMask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    //println!("value: {}\nresult: {}", value, mask.apply(value));
}
