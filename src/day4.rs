use std::collections::HashMap;
use std::str::FromStr;

pub fn input_generator(input: &str) -> Vec<HashMap<String, String>> {
    let mut vec = Vec::new();
    for passport in input.split("\x0D\x0A") {
        let mut map = HashMap::<String, String>::new();
        for field in passport.split(" ") {
            let (k, v) = field.split_once(":").expect("invalid field");
            map.insert(k.into(), v.into());
        }
        vec.push(map);
    }
    return vec;
}

fn has_required_fields(passport: &HashMap<String, String>) -> bool {
    for key in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter() {
        if !passport.contains_key(&key.to_string()) {
            return false;
        }
    }
    return true;
}

fn byr_validate(input: &HashMap<String, String>) -> bool {
    if let Some(value) = input.get("byr") {
        return match u32::from_str(value) {
            Ok(1920..=2002) => { true }
            _ => { false }
        };
    }
    return false;
}

fn iyr_validate(input: &HashMap<String, String>) -> bool {
    if let Some(value) = input.get("iyr") {
        return match u32::from_str(value) {
            Ok(2010..=2020) => { true }
            _ => { false }
        };
    }
    return false;
}

fn eyr_validate(input: &HashMap<String, String>) -> bool {
    if let Some(value) = input.get("eyr") {
        return match u32::from_str(value) {
            Ok(2020..=2030) => { true }
            _ => { false }
        };
    }
    return false;
}

fn hgt_validate(input: &HashMap<String, String>) -> bool {
    if let Some(value) = input.get("hgt") {
        if let Some(cm_value_raw) = value.strip_suffix("cm") {
            return match u32::from_str(cm_value_raw) {
                Ok(150..=193) => { true }
                _ => { false }
            };
        }
        if let Some(in_value_raw) = value.strip_suffix("in") {
            return match u32::from_str(in_value_raw) {
                Ok(59..=96) => { true }
                _ => { false }
            };
        }
    }
    return false;
}

fn hcl_validate(input: &HashMap<String, String>) -> bool {
    if let Some(value) = input.get("hcl") {
        if let Some(value_hex) = value.strip_prefix("#") {
            return match u32::from_str_radix(value_hex, 16) {
                Ok(_) => { true }
                Err(_) => { false }
            };
        }
    }
    return false;
}

fn ecl_validate(input: &HashMap<String, String>) -> bool {
    if let Some(value) = input.get("ecl") {
        return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str());
    }
    return false;
}

fn pid_validate(input: &HashMap<String, String>) -> bool {
    if let Some(value) = input.get("pid") {
        if value.len() == 9 {
            return match u32::from_str(value) {
                Ok(_) => { true }
                Err(_) => { false }
            };
        }
    }
    return false;
}

pub fn part_1(input: &Vec<HashMap<String, String>>) -> usize {
    let mut count: usize = 0;
    for passport in input {
        count += has_required_fields(passport) as usize;
    }
    return count;
}

pub fn part_2(input: &Vec<HashMap<String, String>>) -> usize {
    //println!("before size {}", input.len());
    let test = input.iter()
        .filter(|x| has_required_fields(x))
        .filter(|x| byr_validate(x))
        .filter(|x| iyr_validate(x))
        .filter(|x| eyr_validate(x))
        .filter(|x| hgt_validate(x))
        .filter(|x| hcl_validate(x))
        .filter(|x| ecl_validate(x))
        .filter(|x| pid_validate(x))
        .count();
    //println!("after size {}, test: {}", input.len(), test);
    return test;
}