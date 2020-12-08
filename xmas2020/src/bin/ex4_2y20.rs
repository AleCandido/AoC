use aocinput::request;

const FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn valid_field(k: &str, v: &str) -> bool {
    if k == "byr" {
        if v.len() != 4 {
            return false;
        }
        match v.parse::<usize>() {
            Ok(byr) => {
                if byr < 1920 || byr > 2002 {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        }
    } else if k == "iyr" {
        if v.len() != 4 {
            return false;
        }
        match v.parse::<usize>() {
            Ok(byr) => {
                if byr < 2010 || byr > 2020 {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        }
    } else if k == "eyr" {
        if v.len() != 4 {
            return false;
        }
        match v.parse::<usize>() {
            Ok(byr) => {
                if byr < 2020 || byr > 2030 {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        }
    } else if k == "hgt" {
        if v.ends_with("cm") {
            match v.split_at(v.len() - 2).0.parse::<usize>() {
                Ok(num) => {
                    if num < 150 || num > 193 {
                        return false;
                    }
                }
                Err(_) => {
                    return false;
                }
            }
        } else if v.ends_with("in") {
            match v.split_at(v.len() - 2).0.parse::<usize>() {
                Ok(num) => {
                    if num < 59 || num > 76 {
                        return false;
                    }
                }
                Err(_) => {
                    return false;
                }
            }
        } else {
            return false;
        }
    } else if k == "hcl" {
        let (first, elems) = v.split_at(1);
        if first != "#" {
            return false;
        }
        if elems.len() != 6 {
            return false;
        }
        if elems
            .matches(|c| !(char::is_digit(c, 16) && !c.is_ascii_uppercase()))
            .count()
            != 0
        {
            return false;
        }
    } else if k == "ecl" {
        let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !colors.contains(&v) {
            return false;
        }
    } else if k == "pid" {
        if v.len() != 9 {
            return false;
        }
        if v.matches(|c: char| !c.is_ascii_digit()).count() != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let resp = request::get_input(2020, 4);
    let passports: Vec<String> = resp.trim().split("\n\n").map(|s| s.to_owned()).collect();

    let mut fields = FIELDS.to_vec();
    fields.sort();
    let mut fields_complete = fields.clone();
    fields_complete.push("cid");
    fields_complete.sort();

    let mut valid_pps = 0;

    for passport in passports.iter() {
        //collect passport
        let pp: Vec<(&str, &str)> = passport
            .trim()
            .split_whitespace()
            .map(|s| {
                let mut spl = s.split(":");
                (spl.next().unwrap(), spl.next().unwrap())
            })
            .collect();

        //extract fields
        let mut pp_fields: Vec<&str> = pp.iter().map(|t| t.0).collect();
        pp_fields.sort();

        //validate passport
        if pp_fields == fields || pp_fields == fields_complete {
            let mut valid_pp = true;
            for (k, v) in pp.iter() {
                if !valid_field(k, v) {
                    valid_pp = false;
                    break;
                }
            }
            if valid_pp {
                valid_pps += 1;
            }
        }
    }

    println!("#valid passports: {}", valid_pps);
}
