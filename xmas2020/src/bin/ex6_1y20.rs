use aocinput::request;

fn main() {
    let resp = request::get_input(2020, 6);
    let groups: Vec<String> = resp.trim().split("\n\n").map(|x| x.to_owned()).collect();

    let mut sum = 0;

    for group in groups.iter() {
        //let answers: Vec<String> = group.trim().split("\n").map(|x| x.to_owned()).collect();
        let mut answers: Vec<u8> = group.trim().replace("\n", "").as_bytes().to_vec();
        answers.sort();
        answers.dedup();

        sum += answers.len();
    }
    println!("sum: {}", sum);
}
