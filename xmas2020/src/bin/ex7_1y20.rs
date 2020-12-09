use std::collections::{HashMap, HashSet};

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
                    let num = num_s.parse().unwrap_or_else(|_| {
                        //let num = num_s.parse().unwrap_or_else(|s| {
                        //println!("{}", s);
                        0
                    });
                    let bag = bag_s.to_owned();
                    (num, bag)
                })
                .collect(),
        }
    }

    fn update_gold_holders(
        &self,
        rules: &HashMap<String, Rule>,
        containers: &mut HashSet<String>,
        empties: &mut HashSet<String>,
    ) -> bool {
        if containers.contains(&self.name) {
            return true;
        } else if empties.contains(&self.name) {
            return false;
        } else {
            for el in self.content.iter() {
                if containers.contains(&el.1) {
                    containers.insert(self.name.clone());
                    return true;
                } else if !empties.contains(&el.1.to_owned()) {
                    if el.1 != "other"
                        && rules[&el.1].update_gold_holders(&rules, containers, empties)
                    {
                        containers.insert(self.name.clone());
                        return true;
                    }
                }
            }
            empties.insert(self.name.clone());
            return false;
        }
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

    let mut gold_containers: HashSet<_> = HashSet::new();
    let mut gold_empties = HashSet::new();
    gold_containers.insert("shiny gold".to_owned());

    for (_, rule) in rules.iter() {
        //println!("Starting from '{}':", rule.name);
        rule.update_gold_holders(&rules, &mut gold_containers, &mut gold_empties);
        //println!("\n");
    }
    println!(
        "containers: {:#?}\nempties: {:#?}\nintersection: {:#?}\n#available bags: {}",
        gold_containers,
        gold_empties,
        gold_containers.intersection(&gold_empties),
        gold_containers.len() - 1
    );

    //println!("rules: {:#?}", rules);
}
