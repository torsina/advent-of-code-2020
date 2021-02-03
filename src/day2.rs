use std::str::FromStr;

pub struct Password {
    first_number: u32,
    second_number: u32,
    letter: char,
    password: String
}

pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|x| {
            let line: Vec<&str> = str::split(x, " ").collect();
            let limits: Vec<u32> = line[0].split("-").flat_map(u32::from_str).collect();
            return Password {
                first_number: limits[0],
                second_number: limits[1],
                letter: line[1].chars().next().unwrap(),
                password: line[2].into()
            }
        }).collect()
}

pub fn part_1(input: &Vec<Password>) -> isize {
    let mut final_count: u32 = 0;

    for password in input {
        let mut count: u32 = 0;
        for char in password.password.chars().into_iter() {
            if char == password.letter {
                count = count + 1;
            }
        }
        if count >= password.first_number && count <= password.second_number {
            final_count = final_count + 1;
        }
    }
    return final_count as isize;
}

pub fn part_2(input: &Vec<Password>) -> isize {
    let mut final_count: u32 = 0;

    for password in input {
        //password.password.chars().collect()[1]
        let first_check: bool = password.password.chars().collect::<Vec<char>>()[(password.first_number - 1) as usize] == password.letter;
        let second_check: bool = password.password.chars().collect::<Vec<char>>()[(password.second_number - 1) as usize] == password.letter;
        if (first_check && !second_check) || (!first_check && second_check) {
            final_count = final_count + 1;
        }
    }
    return final_count as isize;
}

pub fn part_2_bytes(input: &Vec<Password>) -> isize {
    let mut final_count: u32 = 0;

    for password in input {
        //password.password.chars().collect()[1]
        let first_check: bool = password.password.as_bytes()[(password.first_number - 1) as usize] == password.letter as u8;
        let second_check: bool = password.password.as_bytes()[(password.second_number - 1) as usize] == password.letter as u8;
        if (first_check && !second_check) || (!first_check && second_check) {
            final_count = final_count + 1;
        }
    }
    return final_count as isize;
}