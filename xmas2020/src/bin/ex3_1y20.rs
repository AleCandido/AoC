use aocinput::request;

fn main() {
    let resp = request::get_input(2020, 3);
    let trees: Vec<Vec<char>> = resp
        .trim()
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();

    let step = 3;
    let width = trees[0].len();

    let mut x = 0;
    let mut trees_crossed = 0;

    for row in trees.iter() {
        if row[x] == '#' {
            trees_crossed += 1;
        }
        x = (x + step) % width;
    }

    println!("#trees: {:#?}", trees_crossed);
}
