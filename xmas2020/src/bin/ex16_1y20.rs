use itertools::Itertools;

use aocinput::request;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(u32, u32)>,
}

impl Rule {
    fn from_str(s: &str) -> Rule {
        let (name, desc) = s
            .split(":")
            .map(|s_el| s_el.trim().to_owned())
            .collect_tuple()
            .unwrap();
        let ranges = desc
            .split("or")
            .map(|r| {
                r.trim()
                    .split("-")
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        Rule { name, ranges }
    }

    fn value_in_range(&self, n: u32) -> bool {
        for range in self.ranges.iter() {
            if range.0 <= n && n <= range.1 {
                return true;
            }
        }
        false
    }
}

fn is_valid(n: u32, rules: &Vec<Rule>) -> bool {
    for rule in rules.iter() {
        if rule.value_in_range(n) {
            return true;
        }
    }
    false
}

fn main() {
    let resp = request::get_input(2020, 16);

    let (rules_s, _, nearby_s) = resp.trim().split("\n\n").collect_tuple().unwrap();

    let mut rules = Vec::new();
    for r in rules_s.split("\n") {
        rules.push(Rule::from_str(r));
    }

    let nearby: Vec<Vec<u32>> = nearby_s
        .split("\n")
        .skip(1)
        .map(|ticket| ticket.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut scanning_error_rate = 0;

    for ticket in nearby.iter() {
        scanning_error_rate += ticket
            .iter()
            .filter(|n| !is_valid(**n, &rules))
            .fold(0, |acc, x| acc + x);
    }

    println!("scanning error rate: {}", scanning_error_rate);
}
