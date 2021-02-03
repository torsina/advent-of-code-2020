use std::collections::HashSet;
use std::str::FromStr;

pub fn input_generator(input: &str) -> HashSet<isize> {
    input.lines().flat_map(isize::from_str).collect()
}

pub fn solve_part1_slow(input: &HashSet<isize>) -> isize {
    for v1 in input {
        for v2 in input {
            if v1 + v2 == 2020 {
                return v1 * v2;
            }
        }
    }

    0
}

pub fn solve_part2_slow(input: &HashSet<isize>) -> isize {
    for v1 in input {
        for v2 in input {
            for v3 in input {
                if v1 + v2 + v3 == 2020 {
                    return v1 * v2 * v3;
                }
            }
        }
    }

    0
}

pub fn solve_part1_set(input: &HashSet<isize>) -> isize {
    for v in input {
        if input.contains(&(2020 - v)) {
            return v * (2020 - v);
        }
    }

    0
}

pub fn solve_part2_set(input: &HashSet<isize>) -> isize {
    for v1 in input {
        for v2 in input {
            if input.contains(&(2020 - v1 - v2)) {
                return v1 * v2 * (2020 - v1 - v2);
            }
        }
    }

    0
}
