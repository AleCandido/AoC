use itertools::iproduct;
use ndarray::prelude::*;

use aocinput::request;

fn step_occupancy(seats: &Array2<char>, row: usize, col: usize) -> char {
    let mut adjacents = Vec::new();

    for (dr, dc) in iproduct!(-1..2, -1..2) {
        // strip the element itself
        if dr == 0 && dc == 0 {
            continue;
        }
        // define the element in the window
        let row_w = row as isize + dr;
        let col_w = col as isize + dc;

        // check box boundaries
        if row_w < 0 || seats.shape()[0] as isize <= row_w {
            continue;
        }
        if col_w < 0 || seats.shape()[1] as isize <= col_w {
            continue;
        }

        let row_w = row_w as usize;
        let col_w = col_w as usize;

        //println!("{} - {}", row_w, col_w);

        adjacents.push(seats[[row_w, col_w]]);
    }

    let curr = seats[[row, col]];
    if curr == 'L' && !adjacents.contains(&'#') {
        '#'
    } else if curr == '#' && adjacents.iter().filter(|c| **c == '#').count() >= 4 {
        'L'
    } else {
        curr
    }
}

fn sweep(seats: &Array2<char>) -> Array2<char> {
    let mut seats_next = seats.clone();
    seats_next.indexed_iter_mut().for_each(|(idx, el)| {
        *el = step_occupancy(&seats, idx.0, idx.1);
    });
    seats_next
}

fn main() {
    let resp = request::get_input(2020, 11);

    let seats_v: Vec<Vec<char>> = resp
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let seats_flat: Vec<char> = seats_v.iter().flatten().cloned().collect();
    let seats = Array2::from_shape_vec((seats_v.len(), seats_v[0].len()), seats_flat).unwrap();

    //let seats = seats.slice(s![..10, ..10]).to_owned();
    let mut curr = seats.clone();
    let mut next = sweep(&curr);
    loop {
        if next == curr {
            break;
        }
        curr = next;
        next = sweep(&curr);
    }

    println!("{}", curr);
    println!("occupancy: {}", curr.iter().filter(|c| **c == '#').count());
}
