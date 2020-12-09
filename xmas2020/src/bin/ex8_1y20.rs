use std::collections::HashSet;

use aocinput::request;

#[derive(Debug)]
struct Instruction {
    kind: String,
    val: isize,
}

impl Instruction {
    fn from_string(s: &str) -> Instruction {
        let pair: Vec<&str> = s.split_ascii_whitespace().collect();
        let kind = pair[0].to_owned();
        let val = pair[1].parse().unwrap();

        Instruction { kind, val }
    }
}

fn main() {
    let resp = request::get_input(2020, 8);
    let instructions: Vec<Instruction> = resp
        .trim()
        .split("\n")
        .map(|instr| Instruction::from_string(instr))
        .collect();

    let mut explored = HashSet::new();
    let mut accumulator = 0;
    let mut instr_n: isize = 0;

    loop {
        if explored.contains(&instr_n) {
            break;
        }
        explored.insert(instr_n);
        let instr = &instructions[instr_n as usize];
        match instr.kind.as_str() {
            "acc" => accumulator += instr.val,
            "jmp" => {
                instr_n += instr.val;

                continue;
            }
            "nop" => {}
            ik => {
                panic!("Instruction '{}' not recognized", ik)
            }
        }
        instr_n += 1;
    }

    println!("accumulator: {}", accumulator);
}
