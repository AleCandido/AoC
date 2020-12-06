//1. Quick_sort on the numbers' list
//2. start from the beginning and loop over:
//- subtract the number from the target
//- binary search if the difference it's present
use aocinput::request;

fn main() {
    let resp = request::get_input(2020, 1);
    let mut expenses: Vec<i32> = resp
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    expenses.sort_unstable();

    println!("{:#?}", expenses);
}
