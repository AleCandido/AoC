use aocinput::request;

#[derive(Clone, Copy, Debug)]
struct BitMask {
    ones: u64,
    zeros: u64,
}

impl BitMask {
    fn new() -> BitMask {
        BitMask { ones: 0, zeros: 0 }
    }

    fn from_str(s: &str) -> BitMask {
        let mut zeros = 0;
        let mut ones = 0;
        for (i, c) in s.trim().chars().enumerate() {
            match c {
                '0' => {
                    zeros += 1 << i;
                }
                '1' => {
                    ones += 1 << i;
                }
                'X' => {}
                a => {
                    panic!("Char not recognized '{}'", a)
                }
            }
        }
        BitMask { ones, zeros }
    }

    fn apply(&self, n: u64) -> u64 {
        n
    }
}

fn main() {
    let resp = request::get_input(2020, 14);

    let mut mask = BitMask::new();

    for line in resp.trim().split("\n") {
        if line.contains('[') {
            //println!("{}", line);
        } else {
            //mask = line.split("=").collect()[1].parse();
            mask = BitMask::from_str(line.split("=").collect::<Vec<&str>>()[1]);
            println!("{:?}", mask);
        }
    }
}
