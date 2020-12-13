use itertools::Itertools;

use aocinput::request;

fn main() {
    let resp = request::get_input(2020, 13);

    let (time, buses) = resp.trim().split("\n").collect_tuple().unwrap();

    let time: u32 = time.parse().unwrap();
    let buses: Vec<u32> = buses
        .split(",")
        .filter(|bus| *bus != "x")
        .map(|bus| bus.parse().unwrap())
        .collect();

    let mut btimes: Vec<(u32, u32)> = buses
        .iter()
        .map(|bus| (*bus, ((time / bus) + 1) * bus))
        .collect();
    btimes.sort_by_key(|t| t.1);

    let (my_bus, my_time) = btimes[0];

    println!("{}", my_bus * (my_time - time));
}
