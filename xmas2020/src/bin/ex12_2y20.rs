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
struct Waypoint {
    east: isize,
    north: isize,
}

impl Waypoint {
    fn new() -> Waypoint {
        Waypoint { east: 0, north: 0 }
    }

    fn rotate(&self, rotate: isize) -> Waypoint {
        let mut new = self.clone();

        let mut i = rotate / 90;
        i %= 4;
        if i < 0 {
            i += 4;
        }
        match i {
            0 => new,
            1 => {
                new.east = -self.north;
                new.north = self.east;
                new
            }
            2 => {
                new.east = -self.east;
                new.north = -self.north;
                new
            }
            3 => {
                new.east = self.north;
                new.north = -self.east;
                new
            }
            d => {
                panic!("Invalid direction: '{}'", d)
            }
        }
    }

    fn exec(&self, instr: &Instruction) -> Waypoint {
        let mut new = self.clone();
        match instr {
            Instruction { code: 'E', value } => new.east += value,
            Instruction { code: 'N', value } => new.north += value,
            Instruction { code: 'W', value } => new.east -= value,
            Instruction { code: 'S', value } => new.north -= value,
            Instruction { code: 'L', value } => new = new.rotate(*value),
            Instruction { code: 'R', value } => new = new.rotate(-value),
            c => {
                panic!("Invalid instruction code: '{:#?}'", c)
            }
        }
        new
    }
}

#[derive(Debug, Clone, Copy)]
struct Ferry {
    east: isize,
    north: isize,
    waypoint: Waypoint,
}

impl Ferry {
    fn new() -> Ferry {
        Ferry {
            east: 0,
            north: 0,
            waypoint: Waypoint::new(),
        }
    }

    fn exec(&self, instr: &Instruction) -> Ferry {
        let mut new = self.clone();

        match instr {
            Instruction { code: 'F', value } => {
                new.east += value * new.waypoint.east;
                new.north += value * new.waypoint.north;
            }
            instr => new.waypoint = new.waypoint.exec(instr),
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
    ferry.waypoint = Waypoint { east: 10, north: 1 };

    for instr in instructions.iter() {
        ferry = ferry.exec(instr);
    }

    println!("{:#?}", ferry);
    println!("{:#?}", ferry.manhattan_distance());
    //let orientations = ['∆', '∇', '⊲', '⊳']
}
