use std::collections::HashMap;

use itertools::Itertools;

use aocinput::request;

#[derive(Clone, Copy, Debug)]
struct BitMask {
    ones: u64,
    zeros: u64,
}

impl BitMask {
    fn new() -> BitMask {
        BitMask { ones: 0, zeros: 0 }
    }

    fn from_str(s: &str) -> BitMask {
        let mut zeros = 0;
        let mut ones = 0;
        for (i, c) in s.trim().chars().rev().enumerate() {
            match c {
                '0' => {
                    zeros += 1 << i;
                }
                '1' => {
                    ones += 1 << i;
                }
                'X' => {}
                a => {
                    panic!("Char not recognized '{}'", a)
                }
            }
        }
        BitMask { ones, zeros }
    }

    fn apply(&self, n: u64) -> u64 {
        //println!("{}", self.ones);
        (n | self.ones) & !self.zeros
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
            memory.insert(loc, mask.apply(value));
        //println!("{}: {}", loc, mask.apply(value));
        } else {
            mask = BitMask::from_str(line.split("=").collect::<Vec<&str>>()[1]);
            //println!("{:?}", mask);
        }
    }

    println!("{}", memory.values().sum::<u64>());
    //let value = 0;
    //let mask = BitMask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    //println!("value: {}\nresult: {}", value, mask.apply(value));
}
