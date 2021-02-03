use itertools::Itertools;
use std::ops::AddAssign;
use std::borrow::BorrowMut;
use std::str::FromStr;
use std::num::ParseIntError;

enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

pub struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: bool,
}

impl Default for Passport {
    fn default() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Some(String::new()),
            hcl: Some(String::new()),
            ecl: Some(String::new()),
            pid: Some(String::new()),
            cid: false }
    }
}

pub fn generator<>(input: &str) -> Vec<Passport> {
    let mut results: Vec<Passport> = Vec::new();
    let passports_input = input.split("\x0D\x0A").collect::<Vec<&str>>();
    for passport_input in passports_input.iter() {
        let mut passport = Passport::default();
        let passport_input_fields = passport_input.split(' ').collect::<Vec<&str>>();

        for passport_input_field in passport_input_fields.iter() {
            let passport_input_split = passport_input_field.split(":").collect_vec();
            let passport_input_key = passport_input_split[0];
            let passport_input_value = passport_input_split[1];
            //println!("{} : {}", passport_input_key, passport_input_value);
            match passport_input_key {
                "byr" => {
                    passport.byr = passport_input_value.parse::<u16>().ok();
                }
                "iyr" => {
                    passport.iyr = passport_input_value.parse::<u16>().ok();
                }
                "eyr" => {
                    passport.eyr = passport_input_value.parse::<u16>().ok();
                }
                "hgt" => {
                    passport.hgt.as_mut().unwrap().insert_str(0, passport_input_value);
                }
                "hcl" => {
                    //println!("error: {}", passport_input_value);

                    passport.hcl.as_mut().unwrap().insert_str(0, passport_input_value);
                }
                "ecl" => {
                    passport.ecl.as_mut().unwrap().insert_str(0, passport_input_value);
                }
                "pid" => {
                    passport.pid.as_mut().unwrap().insert_str(0, passport_input_value);
                }
                "cid" => {
                    passport.cid = true;
                }
                _ => {}
            }
        }
        results.push(passport);
    }

    return results;
}

fn count_fields(input: &Passport) -> isize {
    let mut count: isize = 0;
    // hgt, hcl, pid
    count += input.byr.is_some() as isize;
    count += input.iyr.is_some() as isize;
    count += input.eyr.is_some() as isize;

    if input.ecl.as_ref().unwrap().len() != 0 {
        count += 1;
    }
    if input.hgt.as_ref().unwrap().len() != 0 {
        count += 1;
    }
    if input.hcl.as_ref().unwrap().len() != 0 {
        count += 1;
    }
    if input.pid.as_ref().unwrap().len() != 0 {
        count += 1;
    }
    //count += input.cid as isize;
    return count;
}

pub fn part_1(input: &Vec<Passport>) -> isize {
    let mut count: isize = 0;

    for passport in input.iter() {

        let field_count = count_fields(&passport);


        if field_count == 7 {
            count += 1;
        }
    }
    return count;
}

pub fn part_2(input: &Vec<Passport>) -> isize {
    let mut count:isize = 0;
    for passport in input.iter() {
        //println!("NEW INPUT--------------");
        match passport.byr {
            Some(1920..=2002) => {}
            _ => continue
        }
        //println!("byr check");
        match passport.iyr {
            Some(2010..=2020) => {}
            _ => continue
        }
        //println!("iyr check");

        match passport.eyr {
            Some(2020..=2030) => {}
            _ => continue
        }
        //println!("eyr check");
        let hgt = passport.hgt.as_ref().unwrap();
        if hgt.len() < 4 {
            continue;
        }
        //println!("hgt length check");
        let hgt_unit = &hgt[hgt.len()-2..];
        let hgt_value_raw = &hgt[..hgt.len()-2];
        let hgt_value = i32::from_str(hgt_value_raw).unwrap();
        //println!("hgt: {}, hgt unit: {}, hgt raw value: {}, hgt value: {}, ", hgt, hgt_unit, hgt_value_raw, hgt_value);
        match hgt_unit {
            "cm" => {
                match hgt_value {
                    150..=193 => {}
                    _ => continue
                }
            }
            "in" => {
                match hgt_value {
                    59..=76 => {}
                    _ => continue
                }
            }
            _ => continue
        }
        //println!("hgt check");

        let hcl = passport.hcl.as_ref().unwrap();
        //println!("hcl: {}", hcl);
        if hcl.len() != 7 {
            continue;
        }
        //println!("hcl length check");
        if hcl.as_bytes()[0] != b'#' {
            continue;
        }
        //println!("hcl hashtag check");
        //println!("hex string: {}", &hcl[1..]);
        match i32::from_str_radix(&hcl[1..], 16) {
            Ok(_) => {}
            Err(_) => continue
        }
        //println!("hcl check");

        let ecl = passport.ecl.as_ref().unwrap();
        match ecl.as_str() {
            "amb" |
            "blu" |
            "brn" |
            "gry" |
            "grn" |
            "hzl" |
            "oth" => {}
            _ => {
                continue;
            }
        }





        //println!("ecl check");
        
        match i32::from_str(&passport.pid.as_ref().unwrap()) {
            Ok(_) => {}
            Err(_) => continue
        }
        //println!("pid check");
        count += 1;
    }
    return count;
}

/*
pub fn input_generator<'lifetime>(input: &'lifetime str) -> Vec<HashMap<&[u8], &[u8]>> {
    let mut results: Vec<HashMap<&[u8], &[u8]>> = Vec::new();
    let passports = input.split("\x0D\x0A").collect::<Vec<&str>>();
    for _passport in passports.iter() {
        let passport: &'lifetime [u8] = _passport.as_bytes();
        let mut map: HashMap<&[u8], &[u8]> = HashMap::new();
        results.push(map.to_owned());
        let fields = passport.split(|x| *x == b' ').collect::<Vec<&[u8]>>();
        for _field in fields.iter() {
            let field: &'lifetime [u8] = &_field[0..3];
            let value: &'lifetime [u8] = &_field[4..];
            match field {
                b"byr" | b"iyr" | b"eyr" | b"hgt" | b"hcl" | b"ecl" | b"pid" | b"cid" => {
                    map.insert(field, value);
                }
                _ => {}
            }
        }
    }
    return results;
}

pub fn part_1(input: &Vec<HashMap<&[u8], &[u8]>>) -> isize {
    let mut count: isize = 0;

    for passport in input.iter() {
        let field_count = passport.len();

        if field_count == 8 || (field_count == 7 && !passport.contains_key(b"cid".as_ref())) {
            count += 1;
        }
    }
    return count;
}

pub fn part_2(input: &Vec<HashMap<&[u8], &[u8]>>) -> isize {
    let mut count = 0;
    /*
    for passport in input.iter() {
        let fields = passport.split(" ").collect::<Vec<&str>>();
        if !(fields.len() == 8 || (fields.len() == 7 && !passport.contains("cid"))) {
            continue;
        }
        for field in fields {
            match field.split(":").collect::<Vec<&str>>()[0] {
                "byr" => {}
                "iyr" => {}
                "eyr" => {}
                "hgt" => {}
                "hcl" => {}
                "ecl" => {}
                "pid" => {}
                _ => {}
            }
        }
    }*/


    return count as isize;
}*/