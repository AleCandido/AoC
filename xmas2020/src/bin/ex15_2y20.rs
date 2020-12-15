use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let resp = "1,0,18,10,19,6";

    //let mut history: Vec<u32> = resp.split(",").map(|n| n.parse().unwrap()).collect();
    let mut history: HashMap<u32, u32> = resp
        .split(",")
        .enumerate()
        .map(|n| n.parse().unwrap())
        .collect();
    let l = history.len();

    let mut z = 0;

    for i in l..200000 {
        match history[..(i - 1)]
            .iter()
            .rposition(|n| n == history.last().unwrap())
        {
            Some(last) => {
                history.push((i - 1 - last) as u32);
                if (i - 1 - last) as f64 > (i as f64) / 2. {
                    if (i - 1 - last) > z {
                        z = (i - 1 - last);
                        print!("{}", z);
                    } else {
                        print!(".");
                    }
                    io::stdout().flush().unwrap();
                }
            }
            None => {
                history.push(0 as u32);
            }
        }
    }

    //let l = history.len();
    //println!("{:?}", &history[(l - 1000)..l]);
}
