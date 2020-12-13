use itertools::Itertools;

use aocinput::request;

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn main() {
    let resp = request::get_input(2020, 13);

    let (_, buses) = resp.trim().split("\n").collect_tuple().unwrap();

    let buses: Vec<(i64, i64)> = buses
        .split(",")
        .enumerate()
        .filter(|(_, bus)| *bus != "x")
        .map(|(i, bus)| (i as i64, bus.parse().unwrap()))
        .map(|(i, bus)| {
            (
                {
                    let x = (bus - i) % bus;
                    if x < 0 {
                        x + bus
                    } else {
                        x
                    }
                },
                bus,
            )
        })
        .collect();

    println!("{:?}", buses);

    let (shifts, buses): (Vec<_>, Vec<_>) = buses.iter().cloned().unzip();

    println!("time: {}", chinese_remainder(&shifts, &buses).unwrap());
}
