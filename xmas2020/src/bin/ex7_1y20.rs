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
                    let num = num_s.parse().unwrap_or_else(|s| {
                        println!("{}", s);
                        0
                    });
                    let bag = bag_s.to_owned();
                    (num, bag)
                })
                .collect(),
        }
    }
}

fn main() {
    let resp = request::get_input(2020, 7);
    let rules: Vec<String> = resp.trim().split("\n").map(|x| x.to_owned()).collect();

    for rule in rules.iter() {
        println!("{:#?}", Rule::from_string(&rule));
    }

    //println!("rules: {:#?}", rules);
}
