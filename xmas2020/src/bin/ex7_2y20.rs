use std::collections::HashMap;

use itertools::Itertools;

use aocinput::request;

#[derive(Debug)]
struct Rule {
    name: String,
    content: Vec<(usize, String)>,
}

impl Rule {
    fn from_string(s: &String) -> Rule {
        let mut s = s.clone();
        let separators: Vec<&str> = vec!["bags contain", "bags,", "bag,"];
        let ends: Vec<&str> = vec!["bags.", "bag."];
        for sep in separators.iter() {
            s = s.replace(sep, "-");
        }
        for end in ends.iter() {
            s = s.replace(end, "");
        }

        let mut elems: Vec<_> = s.split("-").map(|s| s.trim().to_owned()).collect();
        //let name = elems[0].clone();
        //let content = elems[1..];

        Rule {
            name: elems.remove(0),
            content: elems
                .iter()
                .map(|el| {
                    let (num_s, bag_s) = el.splitn(2, " ").next_tuple().unwrap();
                    let num = num_s.parse().unwrap_or_else(|_| 0);
                    let bag = bag_s.to_owned();
                    (num, bag)
                })
                .collect(),
        }
    }

    fn count_content(&self, rules: &HashMap<String, Rule>) -> usize {
        let mut count = 1; // it always contains at least 1: itself
        for el in self.content.iter() {
            if el.0 > 0 {
                count += el.0 * rules[&el.1].count_content(rules);
            }
        }
        println!("{}: {}", self.name, count);
        return count;
    }
}

fn main() {
    let resp = request::get_input(2020, 7);
    let rules_s: Vec<String> = resp.trim().split("\n").map(|x| x.to_owned()).collect();

    let mut rules = HashMap::new();

    for rule_s in rules_s.iter() {
        let rule = Rule::from_string(&rule_s);
        rules.insert(rule.name.clone(), rule);
    }

    println!(
        "gold content: {}",
        rules["shiny gold"].count_content(&rules) - 1 // let's subtract itself, since the problem does not count it
    );
}
