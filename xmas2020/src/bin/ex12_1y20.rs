use aocinput::request;

#[derive(Debug)]
struct Instruction {
    code: char,
    value: isize,
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        Instruction {
            code: s.chars().next().unwrap(),
            value: s[1..].parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ferry {
    east: isize,
    north: isize,
    degree: u8,
}

impl Ferry {
    fn new() -> Ferry {
        Ferry {
            east: 0,
            north: 0,
            degree: 0,
        }
    }

    fn rotate(degree: u8, rotate: isize) -> u8 {
        let mut i = degree as isize + rotate / 90;
        i %= 4;
        if i < 0 {
            i += 4;
        }
        i as u8
    }

    fn exec(&self, instr: &Instruction) -> Ferry {
        let mut new = self.clone();

        match instr.code {
            'E' => new.east += instr.value,
            'N' => new.north += instr.value,
            'W' => new.east -= instr.value,
            'S' => new.north -= instr.value,
            'L' => new.degree = Ferry::rotate(new.degree, instr.value),
            'R' => new.degree = Ferry::rotate(new.degree, -instr.value),
            'F' => match new.degree {
                0 => new.east += instr.value,
                1 => new.north += instr.value,
                2 => new.east -= instr.value,
                3 => new.north -= instr.value,
                d => panic!("Invalid direction: '{}'", d),
            },
            c => {
                panic!("Invalid instruction code: '{}'", c)
            }
        };
        new
    }

    /// compute distance from origin
    fn manhattan_distance(&self) -> usize {
        (self.east.abs() + self.north.abs()) as usize
    }
}

fn main() {
    let resp = request::get_input(2020, 12);

    let instructions: Vec<Instruction> = resp
        .trim()
        .split("\n")
        .map(|instr| Instruction::from_str(instr))
        .collect();

    let mut ferry = Ferry::new();
    for instr in instructions.iter() {
        ferry = ferry.exec(instr);
    }

    println!("{:#?}", ferry.manhattan_distance());
    //let orientations = ['∆', '∇', '⊲', '⊳']
}
