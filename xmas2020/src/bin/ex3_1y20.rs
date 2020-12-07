use aocinput::request;

fn main() {
    let resp = request::get_input(2020, 3);
    let trees: Vec<Vec<char>> = resp
        .trim()
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();

    println!("#correct: {:#?}", trees);
}
