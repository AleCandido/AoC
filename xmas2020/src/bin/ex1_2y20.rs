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

    for (idx, e) in (&expenses).iter().enumerate() {
        for (jdx, f) in (&expenses[(idx + 1)..]).iter().enumerate() {
            match expenses.binary_search(&(2020 - e - f)) {
                Ok(compl_idx) => {
                    println!(
                        "el: {}\nfl: {}\ncompl: {}\nidx: {}\njdx: {}\nprod: {}",
                        e,
                        f,
                        expenses[compl_idx],
                        idx,
                        jdx,
                        e * f * expenses[compl_idx]
                    );
                    return;
                }
                Err(_) => {}
            }
        }
    }
}
