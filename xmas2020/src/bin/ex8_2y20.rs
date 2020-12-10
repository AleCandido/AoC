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
    fn exec(instr: &Instruction, acc: &mut isize, instr_n: &mut isize) {
        match instr.kind.as_str() {
            "acc" => *acc += instr.val,
            "jmp" => {
                *instr_n += instr.val;

                return;
            }
            "nop" => {}
            ik => {
                panic!("Instruction '{}' not recognized", ik)
            }
        }
        *instr_n += 1;
    }
}

fn main() {
    let resp = request::get_input(2020, 8);
    let instructions: Vec<Instruction> = resp
        .trim()
        .split("\n")
        .map(|instr| Instruction::from_string(instr))
        .collect();

    // regular segment
    let mut explored = HashSet::new();
    let mut accumulator = 0;
    let mut instr_n: isize = 0;
    explored.insert(instr_n);

    // experiment segment
    let mut exp = false;
    let mut exp_start = 0;
    let mut exp_buffer: HashSet<isize> = HashSet::new();
    let mut exp_acc = 0;
    let mut skip_exp = false;

    loop {
        // load Instruction
        let instr = &instructions[instr_n as usize];
        //println!("instr: {} - {:?}", instr_n, instr);
        // exec as regular
        if !exp {
            explored.insert(instr_n);
            // start experiment
            if !skip_exp && ["jmp", "nop"].contains(&instr.kind.as_str()) {
                exp = true;
                exp_start = instr_n;
                let instr = Instruction {
                    kind: if instr.kind.as_str() == "jmp" {
                        "nop".to_owned()
                    } else {
                        "jmp".to_owned()
                    },
                    ..*instr
                };
                Instruction::exec(&instr, &mut exp_acc, &mut instr_n);
            }
            // regular flow
            else {
                if skip_exp {
                    skip_exp = false;
                }
                Instruction::exec(&instr, &mut accumulator, &mut instr_n);
            }
        }
        // exec as experiment
        else {
            exp_buffer.insert(instr_n);
            Instruction::exec(&instr, &mut exp_acc, &mut instr_n);
        }
        println!(
            "actual: {} - experiment: {}",
            explored.len(),
            exp_buffer.len()
        );

        // reset experiment segment
        if explored.contains(&instr_n) || exp_buffer.contains(&instr_n) {
            if instr_n == exp_start {
                panic!("{}", instr_n);
            }
            exp = false;
            exp_buffer.clear();
            exp_acc = 0;
            instr_n = exp_start;
            skip_exp = true;
        }

        // last instruction executed, terminate condition encountered:
        // "attempting to execute an instruction immediately after the last
        // instruction in the file"
        if instr_n as usize == instructions.len() {
            break;
        }
    }

    println!("accumulator: {}", accumulator + exp_acc);
}
