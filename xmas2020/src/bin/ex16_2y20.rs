use std::collections::HashMap;

use itertools::Itertools;

use aocinput::request;

#[derive(Clone, Debug)]
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
    let resp = "
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    let resp = request::get_input(2020, 16);

    let (rules_s, my_ticket_s, nearby_s) = resp.trim().split("\n\n").collect_tuple().unwrap();

    let mut rules = Vec::new();
    for r in rules_s.split("\n") {
        rules.push(Rule::from_str(r));
    }

    let my_ticket: Vec<u32> = my_ticket_s
        .split("\n")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let nearby: Vec<Vec<u32>> = nearby_s
        .split("\n")
        .skip(1)
        .map(|ticket| {
            ticket
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|ticket| ticket.iter().all(|n| is_valid(*n, &rules)))
        .collect();

    let mut invalid_matching: Vec<(usize, Vec<usize>)> = vec![];
    (0..rules.len()).for_each(|i| invalid_matching.push((i, vec![])));

    for ticket in nearby.iter() {
        for (j, field) in ticket.iter().enumerate() {
            for (i, rule) in rules.iter().enumerate() {
                if !rule.value_in_range(*field) {
                    invalid_matching[i].1.push(j);
                }
            }
        }
    }

    for line in invalid_matching.iter_mut() {
        line.1.sort();
    }
    invalid_matching.sort_by_key(|line| 100 - line.1.len());

    let mut matched_rules = HashMap::new();
    let mut fields_taken = vec![];

    for rule_exclusion in invalid_matching.iter() {
        let rule_name = rules[rule_exclusion.0].name.clone();
        let mut field_checking = 0;
        let mut field_found = false;
        for field in rule_exclusion.1.iter() {
            while fields_taken.contains(&field_checking) {
                field_checking += 1;
            }
            if *field != field_checking {
                matched_rules.insert(rule_name.clone(), field_checking);
                fields_taken.push(field_checking);
                field_found = true;
                break;
            }
            field_checking += 1;
        }
        if !field_found {
            while fields_taken.contains(&field_checking) {
                field_checking += 1;
            }
            matched_rules.insert(rule_name, field_checking);
            fields_taken.push(field_checking);
        }
    }

    let departure_prod: u64 = matched_rules
        .iter()
        .filter(|(k, _)| k.contains("departure"))
        .fold(1, |acc, (_, v)| acc * *my_ticket.get(*v).unwrap() as u64);

    println!("'departure' product: {}", departure_prod);
}
