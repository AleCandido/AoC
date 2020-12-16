fn main() {
    let resp = "1,0,18,10,19,6";

    let mut history: Vec<u32> = resp.split(",").map(|n| n.parse().unwrap()).collect();
    let l = history.len();

    for i in l..2020 {
        match history[..(i - 1)]
            .iter()
            .rposition(|n| n == history.last().unwrap())
        {
            Some(last) => {
                history.push((i - 1 - last) as u32);
            }
            None => history.push(0 as u32),
        }
    }

    println!("{}", history.last().unwrap());
}
