//1. Quick_sort on the numbers' list
//2. start from the beginning and loop over:
//- subtract the number from the target
//- binary search if the difference it's present
use aocinput::request;

struct BoardingPass {
    x: usize,
    y: usize,
}

impl BoardingPass {
    fn from_string(s: &String) -> BoardingPass {
        let (code_x, code_y) = s.split_at(7);

        let x =
            usize::from_str_radix(code_x.replace("B", "1").replace("F", "0").as_str(), 2).unwrap();
        let y =
            usize::from_str_radix(code_y.replace("R", "1").replace("L", "0").as_str(), 2).unwrap();

        BoardingPass { x, y }
    }

    fn seat_id(&self) -> usize {
        self.x * 8 + self.y
    }
}

fn main() {
    let resp = request::get_input(2020, 5);
    let passes: Vec<String> = resp.trim().split("\n").map(|x| x.to_owned()).collect();

    let mut ids: Vec<usize> = vec![];

    for pass in passes.iter() {
        ids.push(BoardingPass::from_string(&pass).seat_id());
    }
    println!("max seat_id: {}", ids.iter().max().unwrap());
}
